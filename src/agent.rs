use anyhow::Result;
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::ollama;
use tracing::{debug, info};

use crate::config::Config;
use crate::tools::WebSearchTool;

// =============================================================================
// SYSTEM PROMPT
// =============================================================================
/// The system prompt defines the agent's personality and behavior.
/// Kept for reference; chat mode uses CHAT_SYSTEM_PROMPT instead.
#[allow(dead_code)]
const RESEARCH_SYSTEM_PROMPT: &str = r#"
You are a helpful AI research assistant. Your task is to research topics and provide summaries.

IMPORTANT INSTRUCTIONS:
1. Use the web_search tool ONCE to find relevant information
2. After getting search results, IMMEDIATELY synthesize them into a summary
3. DO NOT make multiple search requests - one search is sufficient
4. If the first search returns no results, try ONE simpler query, then summarize

When responding after a search:
- **Overview**: Brief introduction to the topic
- **Key Sources Found**: List the URLs from the search
- **Summary**: Synthesize what these sources likely cover based on their titles/domains
- **Next Steps**: Suggest what the user might explore

Always provide a response after seeing search results. Never keep searching indefinitely.
"#;

/// System prompt with conversation history support
const CHAT_SYSTEM_PROMPT: &str = r#"
You are an AI research assistant. You help users by searching the web and summarizing findings.

SEARCH RULES (CRITICAL - FOLLOW EXACTLY):
1. You have access to a web_search tool
2. Search ONCE only - do not repeat searches
3. After the search completes, you MUST provide your final answer directly
4. Stop after one search - do NOT call web_search again
5. Your response should include sources (URLs)

CONVERSATION HISTORY:
{history}

When the user asks a question:
- Search once using web_search
- After receiving results, give a complete answer with sources
- Do not ask follow-up questions or call tools again
"#;

pub struct ResearchAgent {
    /// Configuration for the agent
    config: Config,

    /// The web search tool
    search_tool: WebSearchTool,

    /// Conversation history: (user_question, ai_response)
    history: Vec<(String, String)>,
}

impl ResearchAgent {
    pub fn new(config: Config) -> Self {
        let search_tool = WebSearchTool::new(config.max_search_results);

        Self {
            config,
            search_tool,
            history: Vec::new(),
        }
    }

    /// Chat with the agent, maintaining conversation history
    pub async fn chat(&mut self, query: &str) -> Result<String> {
        info!(query = %query, "Starting chat query");

        std::env::set_var("OLLAMA_API_BASE_URL", &self.config.ollama_host);

        let ollama_client = ollama::Client::from_env();

        debug!(
            host = %self.config.ollama_host,
            model = %self.config.model,
            "Connected to Ollama"
        );

        // Build conversation history string
        let history_str = if self.history.is_empty() {
            "No previous conversation.".to_string()
        } else {
            self.history
                .iter()
                .enumerate()
                .map(|(i, (q, a))| format!("[Turn {}]\nUser: {}\nAI: {}", i + 1, q, a))
                .collect::<Vec<_>>()
                .join("\n\n")
        };

        // Replace {history} in the system prompt
        let system_prompt = CHAT_SYSTEM_PROMPT.replace("{history}", &history_str);

        let agent = ollama_client
            .agent(&self.config.model)
            .preamble(&system_prompt)
            .tool(self.search_tool.clone())
            .build();

        info!("Agent configured, executing chat query");

        let enhanced_query = format!(
            "Research and answer the following question. Use the web_search tool to find \
             current information, then provide a comprehensive summary with sources:\n\n{}",
            query
        );

        let response = agent
            .prompt(&enhanced_query)
            .multi_turn(5)
            .await
            .map_err(|e| anyhow::anyhow!("Agent execution failed: {}", e))?;

        // Save to history
        self.history.push((query.to_string(), response.clone()));

        info!("Chat completed successfully");

        Ok(response)
    }

    /// Clear conversation history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Get conversation history (for debugging)
    #[allow(dead_code)]
    pub fn history(&self) -> &[(String, String)] {
        &self.history
    }

    #[allow(dead_code)]
    pub async fn research(&self, query: &str) -> Result<String> {
        info!(query = %query, "Starting research task");

        std::env::set_var("OLLAMA_API_BASE_URL", &self.config.ollama_host);

        let ollama_client = ollama::Client::from_env();

        debug!(
            host = %self.config.ollama_host,
            model = %self.config.model,
            "Connected to Ollama"
        );

        let agent = ollama_client
            .agent(&self.config.model)
            .preamble(RESEARCH_SYSTEM_PROMPT)
            .tool(self.search_tool.clone())
            .build();

        info!("Agent configured, executing research query");

        // Step 3: Execute the research query
        let enhanced_query = format!(
            "Research the following topic thoroughly. Use the web_search tool to find \
             current information, then provide a comprehensive summary with sources:\n\n{}",
            query
        );

        let response = agent
            .prompt(&enhanced_query)
            .multi_turn(5) // Allow up to 5 iterations of tool calls
            .await
            .map_err(|e| anyhow::anyhow!("Agent execution failed: {}", e))?;

        info!("Research completed successfully");

        Ok(response)
    }

    pub async fn quick_search(&self, query: &str) -> Result<String> {
        info!(query = %query, "Performing quick search");

        let results = self
            .search_tool
            .search(query)
            .await
            .map_err(|e| anyhow::anyhow!("Search failed: {}", e))?;

        if results.is_empty() {
            return Ok(format!("No results found for: {}", query));
        }

        // Format results nicely
        let formatted: String = results
            .iter()
            .enumerate()
            .map(|(i, r)| {
                format!(
                    "{}. **{}**\n   {}\n   URL: {}\n",
                    i + 1,
                    r.title,
                    r.snippet,
                    r.url
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        Ok(format!("## Search Results\n\n{}", formatted))
    }
}

// =============================================================================
// UNIT TESTS
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let config = Config::default();
        let agent = ResearchAgent::new(config);

        assert_eq!(agent.config.model, "llama3.2");
        assert!(agent.history.is_empty());
    }

    #[test]
    fn test_system_prompt_not_empty() {
        assert!(!RESEARCH_SYSTEM_PROMPT.is_empty());
        assert!(RESEARCH_SYSTEM_PROMPT.contains("research"));
    }

    #[test]
    fn test_chat_system_prompt_not_empty() {
        assert!(!CHAT_SYSTEM_PROMPT.is_empty());
        assert!(CHAT_SYSTEM_PROMPT.contains("history"));
    }

    #[test]
    fn test_clear_history() {
        let config = Config::default();
        let mut agent = ResearchAgent::new(config);

        // Initially empty
        assert!(agent.history.is_empty());

        // Manually add to history for testing
        agent.history.push(("test".to_string(), "response".to_string()));
        assert_eq!(agent.history.len(), 1);

        // Clear history
        agent.clear_history();
        assert!(agent.history.is_empty());
    }

    #[test]
    fn test_history_getter() {
        let config = Config::default();
        let agent = ResearchAgent::new(config);

        assert_eq!(agent.history().len(), 0);
    }
}
