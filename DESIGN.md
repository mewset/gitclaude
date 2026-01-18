# Gitclaude - Design Document

> A system daemon that listens to git commands and triggers Claude Code sessions

## Vision

Gitclaude is a tool that automatically reacts to git events (commits, push, merge, etc.) and spins up Claude Code to provide feedback, code reviews, changelog generation, and more.

**Key Principles:**
- Uses the user's Claude subscription via CLI (no API costs)
- Fully configurable behavior
- Token-efficient
- Works both globally and per-repo

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        gitclaude                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │   CLI    │  │  Config  │  │  Hooks   │  │ Templates│   │
│  │  Parser  │  │  Loader  │  │ Manager  │  │  Engine  │   │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘   │
│       │             │             │             │          │
│  ┌────▼─────────────▼─────────────▼─────────────▼────┐    │
│  │                    Core Engine                     │    │
│  ├───────────────────────────────────────────────────┤    │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐           │    │
│  │  │  Rate   │  │ Context │  │ Monorepo│           │    │
│  │  │ Limiter │  │ Builder │  │ Detector│           │    │
│  │  └─────────┘  └─────────┘  └─────────┘           │    │
│  └───────────────────────┬───────────────────────────┘    │
│                          │                                 │
│  ┌───────────────────────▼───────────────────────────┐    │
│  │               Claude Bridge                        │    │
│  │  ┌─────────────────────────────────────────────┐  │    │
│  │  │  claude --print "$(cat context.md)"         │  │    │
│  │  │  claude -p "Review this" --output-format md │  │    │
│  │  └─────────────────────────────────────────────┘  │    │
│  └───────────────────────┬───────────────────────────┘    │
│                          │                                 │
│  ┌───────────────────────▼───────────────────────────┐    │
│  │              Output Handlers                       │    │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐     │    │
│  │  │ Notify │ │  File  │ │Terminal│ │Git Note│     │    │
│  │  └────────┘ └────────┘ └────────┘ └────────┘     │    │
│  └───────────────────────────────────────────────────┘    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Components

### CLI Parser
Handles all user commands via `clap`.

### Config Loader
Loads and merges configuration from:
1. Repo-specific: `.gitclaude/config.toml`
2. Global: `~/.config/gitclaude/config.toml`

### Hooks Manager
Installs and manages git hooks:
- `post-commit`
- `post-push`
- `pre-commit`
- `post-merge`
- `post-checkout`

### Templates Engine
Renders prompt templates with Handlebars:
- `{{commit_message}}`
- `{{diff}}`
- `{{author}}`
- `{{affected_files}}`
- etc.

### Rate Limiter
Prevents token spam during high commit activity.

### Context Builder
Builds intelligent context based on configured level.

### Monorepo Detector
Identifies monorepo structure and scoped context.

### Claude Bridge
Runs Claude CLI with the right arguments and context.

### Output Handlers
Handles Claude responses: notifications, files, terminal, git notes.

---

## CLI Commands

```bash
gitclaude init              # Interactive onboarding
gitclaude enable            # Enable in current repo
gitclaude disable           # Disable in current repo
gitclaude config            # Open/edit config
gitclaude config --global   # Edit global config
gitclaude status            # Show active configuration
gitclaude logs              # Show previous responses
gitclaude run <event>       # Manually trigger event
gitclaude templates         # Manage templates
gitclaude templates list    # List templates
gitclaude templates edit    # Edit template
```

---

## Git Events

| Event | Trigger | Default Template | Use Case |
|-------|---------|------------------|----------|
| `post-commit` | After commit | `review` | Code review |
| `post-push` | After push | `changelog` | Changelog generation |
| `pre-commit` | Before commit | `validate` | Validation (blocking) |
| `post-merge` | After merge | `summary` | Merge summary |
| `post-checkout` | After checkout | `context` | Branch context |

---

## Context Levels

| Level | Includes | Tokens (approx) | Use case |
|-------|----------|-----------------|----------|
| `minimal` | Commit message | ~100 | Quick feedback |
| `standard` | Message + diff | ~1000-4000 | Standard review |
| `extended` | + related files, recent commits | ~4000-8000 | Deep analysis |
| `full` | Entire repo via claude | Varies | Architecture feedback |

### Smart Truncation

For large diffs:
1. Calculate complexity score per file
2. Prioritize by file type and change amount
3. Include diff --stat for overview
4. Truncate at token budget

```rust
struct DiffChunk {
    file: String,
    change_type: ChangeType,
    lines_changed: usize,
    complexity_score: f32,
}
```

---

## Rate Limiting

### Strategies

| Strategy | Description |
|----------|-------------|
| `debounce` | Wait X seconds after last commit |
| `batch` | Collect commits, run once |
| `cooldown` | Minimum time between runs |
| `confirm` | Ask the user |
| `smart` | Combined logic |

### Smart Detection

```rust
fn should_run(commits_last_hour: usize, last_run: Duration) -> Decision {
    match (commits_last_hour, last_run) {
        (n, _) if n > 10 => Decision::Skip("High activity"),
        (_, d) if d < Duration::minutes(2) => Decision::Debounce(30),
        _ => Decision::Run,
    }
}
```

---

## Global vs Per-repo

### Hook Resolution

```
1. Check .gitclaude/config.toml in repo
   └─ If exists: use repo-specific config

2. Otherwise: check ~/.config/gitclaude/config.toml
   └─ If global_listen = true: use global config
   └─ If repo is in ignore_repos[]: skip

3. Merge: repo config overrides global
```

---

## Monorepo Support

### Detection

```rust
fn detect_monorepo(repo_root: &Path) -> MonorepoType {
    if exists("pnpm-workspace.yaml") { MonorepoType::Pnpm }
    else if exists("lerna.json") { MonorepoType::Lerna }
    else if exists("Cargo.toml") && has_workspace() { MonorepoType::CargoWorkspace }
    else if exists("packages/") || exists("apps/") { MonorepoType::Generic }
    else { MonorepoType::None }
}
```

### Scoped Context

- Detect affected packages from diff
- Only send relevant context
- Support for nested `.gitclaude.toml`

---

## Output Handlers

| Handler | Description | Config |
|---------|-------------|--------|
| `notify` | Desktop notification | urgency, timeout |
| `file` | Save to file | path, format |
| `terminal` | Open in terminal | terminal app |
| `git-note` | Add as git note | - |
| `clipboard` | Copy to clipboard | - |
| `session` | Open interactive Claude | auto_open |

---

## Rust Crates

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
dialoguer = "0.11"
indicatif = "0.17"
toml = "0.8"
serde = { version = "1", features = ["derive"] }
git2 = "0.18"
notify-rust = "4"
tokio = { version = "1", features = ["process", "fs", "rt-multi-thread"] }
directories = "5"
handlebars = "5"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

## File Structure

```
gitclaude/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   ├── enable.rs
│   │   ├── config.rs
│   │   └── run.rs
│   ├── config/
│   │   ├── mod.rs
│   │   ├── loader.rs
│   │   └── types.rs
│   ├── hooks/
│   │   ├── mod.rs
│   │   ├── manager.rs
│   │   └── scripts.rs
│   ├── context/
│   │   ├── mod.rs
│   │   ├── builder.rs
│   │   ├── diff.rs
│   │   └── monorepo.rs
│   ├── rate_limit/
│   │   ├── mod.rs
│   │   └── strategies.rs
│   ├── claude/
│   │   ├── mod.rs
│   │   └── bridge.rs
│   ├── output/
│   │   ├── mod.rs
│   │   ├── notify.rs
│   │   ├── file.rs
│   │   └── terminal.rs
│   └── templates/
│       ├── mod.rs
│       └── engine.rs
├── templates/
│   ├── review.md
│   ├── changelog.md
│   ├── validate.md
│   └── summary.md
└── examples/
    ├── config.toml
    └── repo-config.toml
```

---

## Onboarding Flow

```
$ gitclaude init

┌─────────────────────────────────────────────────────┐
│  🎉 Welcome to gitclaude!                           │
└─────────────────────────────────────────────────────┘

? Should gitclaude listen globally or only in specific repos?
  ○ Globally (all git repos, can exclude some)
  ● Only enabled repos

? Which git events should trigger Claude?
  ☑ post-commit
  ☐ post-push
  ☐ pre-commit
  ☐ post-merge

? How should Claude run?
  ○ Synchronously
  ● Asynchronously
  ○ Ask each time

? Output format?
  ☑ Desktop notification
  ☐ Save to file
  ☐ Open interactive session

? Context level?
  ○ Minimal
  ● Standard
  ○ Extended
  ○ Full

? Rate limiting?
  ● Debounce (30s)
  ○ Batch
  ○ Cooldown
  ○ Smart

✅ Configuration saved!
```

---

## Implementation Order

### Phase 1: Foundation
- [ ] Project setup (Cargo.toml, structure)
- [ ] CLI parsing with clap
- [ ] Config types and loader
- [ ] Basic hook installation

### Phase 2: Core
- [ ] Context builder (minimal + standard)
- [ ] Claude bridge
- [ ] Output: notify
- [ ] Rate limiting: debounce

### Phase 3: Features
- [ ] Interactive onboarding
- [ ] Templates engine
- [ ] Extended context
- [ ] Monorepo detection
- [ ] All output handlers

### Phase 4: Polish
- [ ] Error handling
- [ ] Logging
- [ ] Tests
- [ ] Documentation
- [ ] Release automation

---

## Open Questions (Resolved)

1. ✅ Language: Rust
2. ✅ Global/per-repo: Both, configurable
3. ✅ Large diffs: Smart truncation + prioritization
4. ✅ Rate limiting: Multiple strategies, configurable
5. ✅ Monorepo: Automatic detection + scoped context
