/// Configuration management
mod config;

/// Research agent implementation
mod agent;

/// Web search and other tools
mod tools;

use anyhow::Result;
use clap::Parser;
use std::io::{self, Write};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::agent::ResearchAgent;
use crate::config::Config;

#[derive(Parser, Debug)]
#[command(
    name = "ai-research-agent",
    author = "qunisid",
    version = "0.1.0",
    about = "An AI-powered research assistant that searches the web and summarizes findings",
    long_about = r#"
AI Research Agent - Your intelligent research companion!

This tool uses local LLMs (via Ollama) and web search to help you research any topic.
It will:
  1. Search the web for relevant information
  2. Analyze and synthesize the results
  3. Provide a comprehensive summary with sources

PREREQUISITES:
  1. Install Ollama: https://ollama.ai
  2. Pull a model: ollama pull llama3.2
  3. Start Ollama: ollama serve

EXAMPLES:
  # Basic research query
  ai-research-agent "What are the latest developments in Rust async?"
  
  # Quick search without synthesis
  ai-research-agent --quick "Rust web frameworks 2024"
  
  # Use a specific model
  ai-research-agent --model deepseek-v3.2 "Machine learning in Rust"
"#
)]
struct Args {
    /// The research topic or question to investigate
    #[arg(help = "The topic to research", value_name = "QUERY")]
    query: Option<String>,

    /// Interactive mode - ask questions one by one
    #[arg(
        short = 'i',
        long = "interactive",
        help = "Enter interactive REPL mode",
        default_value = "false"
    )]
    interactive: bool,

    /// The Ollama model to use (overrides OLLAMA_MODEL env var)
    #[arg(
        short = 'm',
        long = "model",
        help = "Ollama model to use",
        env = "OLLAMA_MODEL"
    )]
    model: Option<String>,

    /// Quick search mode - just search, don't synthesize
    #[arg(
        short = 'q',
        long = "quick",
        help = "Quick search mode (no AI synthesis)",
        default_value = "false"
    )]
    quick: bool,

    /// Verbose output (debug logging)
    #[arg(
        short = 'v',
        long = "verbose",
        help = "Enable verbose/debug logging",
        default_value = "false"
    )]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    // Clap handles --help, --version, and error messages automatically
    let args = Args::parse();

    // Initialize logging
    init_logging(args.verbose)?;

    info!("AI Research Agent starting up...");

    // Load configuration from environment/.env file
    let mut config = Config::from_env()?;

    // Override model if specified on command line
    //
    // # Rust Concept: Option Type
    // Option<T> is either Some(value) or None.
    // if let Some(x) = option { } is a concise way to handle this.
    if let Some(model) = args.model {
        info!(model = %model, "Using model from command line");
        config.model = model;
    }

    // Validate configuration
    config.validate()?;

    info!(
        model = %config.model,
        host = %config.ollama_host,
        "Configuration loaded"
    );

    // Create the research agent
    let mut agent = ResearchAgent::new(config);

    // Check if interactive mode or single query
    if args.interactive {
        run_interactive(&mut agent).await?;
    } else if let Some(query) = args.query {
        // Execute the query
        let result = if args.quick {
            info!("Running in quick search mode");
            agent.quick_search(&query).await
        } else {
            info!("Running full research mode");
            agent.chat(&query).await
        };

        handle_result(result)?;
    } else {
        // No query provided and not interactive - show help
        eprintln!("Error: Please provide a query or use --interactive mode");
        eprintln!("\nUsage:");
        eprintln!("  ai-research-agent \"Your question here\"");
        eprintln!("  ai-research-agent --interactive");
        eprintln!("\nRun with --help for more options.");
        anyhow::bail!("No query provided");
    }

    info!("Research completed successfully");
    Ok(())
}

/// Run interactive REPL mode
async fn run_interactive(agent: &mut ResearchAgent) -> Result<()> {
    println!("\n{}", "=".repeat(60));
    println!("AI Research Agent - Interactive Mode");
    println!("{}", "=".repeat(60));
    println!("Type your question and press Enter.");
    println!("Commands: 'clear' to clear history, 'quit' or 'exit' to quit.");
    println!("{}\n", "=".repeat(60));

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // Handle commands
        match input.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "clear" => {
                agent.clear_history();
                println!("Conversation history cleared.\n");
                continue;
            }
            "" => continue,
            _ => {}
        }

        // Process the question
        match agent.chat(input).await {
            Ok(response) => {
                println!("\n{}", "=".repeat(60));
                println!("AI:");
                println!("{}", response);
                println!("{}\n", "=".repeat(60));
            }
            Err(e) => {
                eprintln!("\n❌ Error: {}", e);
                if e.to_string().contains("connection refused") {
                    eprintln!("💡 Tip: Make sure Ollama is running (ollama serve)");
                } else if e.to_string().contains("model") {
                    eprintln!("💡 Tip: Make sure the model is installed (ollama pull llama3.2)");
                }
            }
        }
    }

    Ok(())
}

/// Handle the result and print response
fn handle_result(result: Result<String>) -> Result<()> {
    match result {
        Ok(response) => {
            println!("\n{}", "=".repeat(60));
            println!("RESEARCH RESULTS");
            println!("{}\n", "=".repeat(60));
            println!("{}", response);
            println!("\n{}", "=".repeat(60));
        }
        Err(e) => {
            error!(error = %e, "Research failed");
            eprintln!("\n❌ Research failed: {}", e);

            if e.to_string().contains("connection refused") {
                eprintln!("\n💡 Tip: Make sure Ollama is running:");
                eprintln!("   ollama serve");
            } else if e.to_string().contains("model") {
                eprintln!("\n💡 Tip: Make sure the model is installed:");
                eprintln!("   ollama pull llama3.2");
            }
            return Err(e);
        }
    }
    Ok(())
}

fn init_logging(verbose: bool) -> Result<()> {
    // Set log level based on verbose flag
    let level = if verbose { Level::DEBUG } else { Level::INFO };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(true) // Show the module that logged
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .finish();

    // Set as the global default
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| anyhow::anyhow!("Failed to set logging subscriber: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing() {
        // Test that CLI args parse correctly
        let args = Args::parse_from(["test", "What is Rust?"]);
        assert_eq!(args.query, Some("What is Rust?".to_string()));
        assert!(!args.quick);
        assert!(!args.verbose);
    }

    #[test]
    fn test_args_with_flags() {
        let args = Args::parse_from([
            "test",
            "--quick",
            "--verbose",
            "--model",
            "llama3.2",
            "Test query",
        ]);

        assert_eq!(args.query, Some("Test query".to_string()));
        assert!(args.quick);
        assert!(args.verbose);
        assert_eq!(args.model, Some("llama3.2".to_string()));
    }
}
