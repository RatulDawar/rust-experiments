# Rust Experiments - Blog Creation Framework

This repository contains **code experiments only**. Blog content is managed separately in the GitHub Pages repository.

## Repository Structure

```
rust-experiments/
├── AGENTS.md              (this file - blog framework)
├── README.md              (optional - repo overview)
├── Cargo.toml             (workspace configuration)
├── .gitignore             (excludes blog drafts)
└── <experiment-name>/     (individual experiments)
    ├── Cargo.toml
    ├── src/
    ├── benches/
    └── tests/
```

## Two-Repository System

### 1. rust-experiments (Code)
- **Repository**: `RatulDawar/rust-experiments`
- **Purpose**: Runnable code, benchmarks, experiments
- **What to commit**: Code, tests, benchmarks, build configs
- **What NOT to commit**: Blog posts, drafts, Medium versions

### 2. RatulDawar.github.io (Blog)
- **Repository**: `RatulDawar/RatulDawar.github.io`
- **Purpose**: Published blog posts with Jekyll
- **URL**: https://ratuldawar.github.io
- **What to commit**: Markdown posts, Jekyll config, assets

## Blog Creation Workflow

### Step 1: Create Experiment (Code Only)

```bash
cd /Users/ratuldawar/github/rust-experiments

# Create new experiment folder
mkdir <experiment-name>
cd <experiment-name>

# Initialize Cargo project
cargo init --lib

# Add to workspace
# Edit ../Cargo.toml and add to members = ["<experiment-name>"]

# Write code, benchmarks, tests
# Commit only the code
git add .
git commit -m "Add <experiment-name> experiment"
git push
```

### Step 2: Draft Blog Post (Separate Location)

**Use temporary folder outside repos:**

```bash
cd /Users/ratuldawar/github/rust-blogs  # Or any temp location
mkdir drafts  # Git-ignored

# Create blog draft
# File: drafts/<title>.md
```

**Draft Structure:**

```markdown
# Title

Introduction

## Problem Statement
## Solution
## Implementation Details
## Performance Results
## Code Examples

## Run It Yourself
Link to: https://github.com/RatulDawar/rust-experiments

## Conclusion
```

### Step 3: Publish to GitHub Pages

```bash
cd /Users/ratuldawar/github/RatulDawar.github.io

# Create post with Jekyll frontmatter
cat > posts/<slug>.md <<'EOF'
---
layout: default
title: "Your Title"
date: YYYY-MM-DD
categories: rust performance systems-programming
---

# Your Title

[Your blog content with TABLES - they render well on GitHub Pages]
EOF

# Commit and push
git add posts/<slug>.md
git commit -m "Add <title> blog post"
git push

# Site will be live at:
# https://ratuldawar.github.io/posts/<slug>.html
```

### Step 4: Create Medium Version

**Medium doesn't support:**
- Markdown tables
- Jekyll frontmatter
- Automatic formatting

**Create Medium-friendly version:**

```bash
cd /Users/ratuldawar/github/rust-blogs/drafts  # Temp folder

# Copy GitHub Pages version
cp /Users/ratuldawar/github/RatulDawar.github.io/posts/<slug>.md medium-<slug>.md

# Manual changes:
# 1. Remove Jekyll frontmatter (---)
# 2. Convert tables to bullet points
# 3. Keep code blocks (```rust)
# 4. Simplify formatting
```

**Post to Medium:**
1. Go to https://medium.com → "Write"
2. Copy content from medium-<slug>.md
3. Paste into Medium editor
4. Manually format using toolbar:
   - Headings: Highlight text → Click "T" or use ##
   - Bold: Highlight → Click "B" or Ctrl+B
   - Code: Highlight → Click "</>" or use backticks
   - Lists: Use "-" then space
5. Preview
6. Add tags: rust, performance, systems-programming, concurrency
7. Publish

### Step 5: Create LinkedIn Post

**Template:**

```markdown
🚀 Just published: "[Your Title]"

[Hook - surprising fact or problem]

Key findings:
• [Metric 1]
• [Metric 2]
• [Metric 3]

[Brief technical insight]

📖 Read: [Medium URL]
💻 Code: https://github.com/RatulDawar/rust-experiments

#Rust #PerformanceOptimization #SystemsProgramming
```

**Post on LinkedIn:**
1. Copy template
2. Fill in your data
3. Paste to LinkedIn
4. Add emojis if desired
5. Post

## Content Guidelines

### For Code Repository (rust-experiments)

✅ **DO Commit:**
- Rust source code
- Cargo.toml configurations
- Benchmark code
- Test suites
- Build scripts
- README.md (experiment overview)

❌ **DON'T Commit:**
- Blog post markdown
- Draft content
- Medium versions
- LinkedIn posts
- Screenshots
- Profiling data (*.trace/, *.xml)

### For Blog Repository (RatulDawar.github.io)

✅ **DO Commit:**
- Final blog posts with Jekyll frontmatter
- Use markdown tables (they work on GitHub Pages)
- Technical diagrams
- Performance charts

❌ **DON'T Commit:**
- Medium-specific versions
- Draft iterations
- Personal notes

## Performance Data Format

### GitHub Pages (Use Tables)

```markdown
| Version | Time | Speedup |
|---------|------|---------|
| Unpadded | 752ms | 1.0x |
| Padded | 167ms | 4.5x |
```

### Medium (Use Bullet Points)

```markdown
**Unpadded:**
- Time: 752ms
- Speedup: 1.0x

**Padded:**
- Time: 167ms
- Speedup: 4.5x
```

## Experiment Naming Convention

```
<topic>-<subtopic>
```

Examples:
- `cache-padding` (current)
- `lock-free-queue`
- `simd-optimization`
- `zero-copy-parsing`

## Blog Post Naming Convention

**GitHub Pages:**
```
posts/<slug>.md
```

**Slug format:** `topic-subtopic-description`

Examples:
- `cache-padding-false-sharing.md`
- `lock-free-queue-performance.md`

## Quick Reference Commands

### New Experiment
```bash
cd /Users/ratuldawar/github/rust-experiments
mkdir <name> && cd <name>
cargo init --lib
# Add to workspace in ../Cargo.toml
```

### Publish Blog
```bash
cd /Users/ratuldawar/github/RatulDawar.github.io
# Edit posts/<slug>.md
git add posts/<slug>.md
git commit -m "Add <title> post"
git push
```

### Check Blog Status
```bash
# GitHub Pages build status
gh api repos/RatulDawar/RatulDawar.github.io/pages

# View live site
open https://ratuldawar.github.io
```

## AI Agent Instructions

When creating new blog content:

1. ✅ Commit code to `rust-experiments`
2. ✅ Create blog post in `RatulDawar.github.io/posts/`
3. ✅ Use tables in GitHub Pages version
4. ✅ Create separate Medium version with bullet points
5. ✅ Generate LinkedIn post template
6. ❌ Never commit blog content to code repo
7. ❌ Never commit Medium drafts anywhere

## Links

- **Code Repository**: https://github.com/RatulDawar/rust-experiments
- **Blog**: https://ratuldawar.github.io
- **Medium**: https://medium.com/@ratuldawar11
- **LinkedIn**: [Your LinkedIn Profile]

## Notes

- Medium import from GitHub Pages doesn't work well (tables break)
- Always create separate Medium version
- GitHub Pages uses Jekyll (builds in ~2 minutes)
- Keep drafts outside git (use local `drafts/` folder)
- Blog content and code live in separate repos for clean separation
