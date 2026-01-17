# Git 使用指南

本项目使用 Git 进行版本控制，以下是常用命令和操作指南。

## 基础命令

### 查看状态
```bash
# 查看工作区状态
git status

# 查看修改内容
git diff

# 查看提交历史
git log --oneline
```

### 暂存与提交
```bash
# 添加所有修改到暂存区
git add .

# 提交修改
git commit -m "描述你的修改"

# 查看暂存区内容
git diff --cached
```

### 分支操作
```bash
# 查看所有分支
git branch

# 创建新分支
git branch feature/new-feature

# 切换分支
git checkout feature/new-feature

# 创建并切换到新分支
git checkout -b feature/new-feature

# 删除分支
git branch -d feature/new-feature
```

## 版本发布流程

### 1. 确保代码是最新的
```bash
git checkout main
git pull origin main
```

### 2. 创建版本 tag
```bash
# 创建 tag（语义化版本号）
git tag v0.1.0

# 创建带注释的 tag
git tag -a v0.1.0 -m "Release version 0.1.0"

# 列出所有 tag
git tag -l
```

### 3. 推送到远程
```bash
# 推送代码
git push origin main

# 推送 tag
git push origin v0.1.0

# 推送所有 tag
git push origin --tags
```

### 4. 发布到 GitHub
在 GitHub 仓库页面：
1. 点击 "Releases" → "Create a new release"
2. 选择 tag 版本
3. 填写发布说明
4. 点击 "Publish release"

## 错误恢复

### 恢复误删的文件
```bash
# 查看文件删除记录
git log --diff-filter=D --summary

# 恢复文件
git checkout <commit_hash> -- <file_path>
```

### 恢复误删的分支
```bash
# 查看所有分支（包括已删除的）
git reflog

# 恢复分支
git checkout -b <branch_name> <commit_hash>
```

### 回退提交

**软回退（保留修改在暂存区）**
```bash
git reset --soft HEAD~1
```

**混合回退（保留修改在工作区）**
```bash
git reset HEAD~1
```

**硬回退（删除所有修改）**
```bash
git reset --hard HEAD~1
```

### 恢复整个项目到某个版本
```bash
# 查看历史找到目标版本的 commit hash
git log --oneline

# 硬回退到该版本（谨慎使用！）
git reset --hard <commit_hash>

# 如果已经推送到远程，需要强制推送
git push --force origin main
```

### 撤销最后一次提交但保留修改
```bash
git reset --mixed HEAD~1
# 或
git reset HEAD~1
```

### 查看操作历史（恢复误操作）
```bash
git reflog
```

## GitHub 操作

### 首次推送本地仓库到 GitHub
```bash
# 创建本地仓库后
git init
git add .
git commit -m "Initial commit"

# 添加远程仓库
git remote add origin https://github.com/你的用户名/仓库名.git

# 推送代码和 tag
git push -u origin main
git push origin --tags
```

### Fork 后同步上游仓库
```bash
# 添加上游仓库
git remote add upstream https://github.com/original-owner/repo.git

# 获取上游更新
git fetch upstream

# 合并到 main 分支
git checkout main
git merge upstream/main

# 推送更新到你的 fork
git push origin main
```

## 常用配置

### 设置用户信息
```bash
git config user.name "Your Name"
git config user.email "your.email@example.com"
```

### 设置别名
```bash
git config --global alias.st status
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.ci commit
```

### 设置默认分支名
```bash
git config --global init.defaultBranch main
```

## 注意事项

1. **提交前检查**：使用 `git status` 和 `git diff` 确认修改内容
2. **提交信息**：写清楚提交目的，避免使用 "fix" 或 "update" 等模糊描述
3. **推送前拉取**：`git pull` 后再推送，避免冲突
4. **Tag 管理**：发布新版本时创建 tag，便于追踪
5. **敏感信息**：不要将 API keys、密码等提交到仓库
