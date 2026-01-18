# gitclaude

> Git hook daemon that triggers Claude Code sessions for automatic code review, changelog generation, and more.

## Features

- 🪝 **Git Hook Integration** - Automatically react to commits, pushes, merges
- 🤖 **Claude CLI Bridge** - Uses your Claude subscription (no API costs)
- 📝 **Smart Templates** - Customizable prompts for different events
- 🎯 **Context-Aware** - Intelligent diff analysis with token optimization
- 📦 **Monorepo Support** - Detect affected packages, scope context
- ⚡ **Rate Limiting** - Prevent token spam with debounce/batch/cooldown
- 🔔 **Flexible Output** - Notifications, files, terminal, git notes

## Installation

```bash
# From source
cargo install --path .

# Or build locally
cargo build --release
./target/release/gitclaude --help
```

## Quick Start

```bash
# Interactive setup
gitclaude init

# Enable in a repository
cd your-project
gitclaude enable

# That's it! Claude will now review your commits
git commit -m "Add new feature"
# → Desktop notification with code review
```

## Commands

| Command | Description |
|---------|-------------|
| `gitclaude init` | Interactive setup wizard |
| `gitclaude enable` | Enable in current repo |
| `gitclaude disable` | Disable in current repo |
| `gitclaude config` | View/edit configuration |
| `gitclaude status` | Show current status |
| `gitclaude logs` | View response history |
| `gitclaude run <event>` | Manually trigger event |
| `gitclaude templates` | Manage templates |

## Configuration

Global config: `~/.config/gitclaude/config.toml`
Repo config: `.gitclaude/config.toml`

```toml
[general]
async = true
notify = true

[events.post-commit]
enabled = true
template = "review"
context = "standard"
output = ["notify"]

[context]
max_tokens = 4000
strategy = "smart"

[rate_limit]
strategy = "debounce"
debounce_seconds = 30
```

See `examples/config.toml` for full configuration options.

## Templates

Built-in templates:
- `review` - Code review after commit
- `changelog` - Generate changelog on push
- `validate` - Pre-commit validation (blocking)
- `summary` - Merge summary

Custom templates go in `~/.config/gitclaude/templates/` or `.gitclaude/templates/`

### Template Variables

| Variable | Description |
|----------|-------------|
| `{{commit_hash}}` | Short commit hash |
| `{{commit_message}}` | Full commit message |
| `{{author}}` | Commit author |
| `{{date}}` | Commit date |
| `{{branch}}` | Current branch |
| `{{diff}}` | Diff content |
| `{{diff_stat}}` | Diff statistics |
| `{{affected_files}}` | List of changed files |
| `{{affected_packages}}` | Affected monorepo packages |
| `{{recent_commits}}` | Recent commit history |

## Context Levels

| Level | Content | Use Case |
|-------|---------|----------|
| `minimal` | Commit message only | Quick feedback |
| `standard` | Message + diff | Normal review |
| `extended` | + related files, history | Deep analysis |
| `full` | Full repo context | Architecture feedback |

## Rate Limiting

Prevent token spam during high commit activity:

| Strategy | Behavior |
|----------|----------|
| `debounce` | Wait N seconds after last commit |
| `batch` | Collect commits, run once |
| `cooldown` | Minimum time between runs |
| `smart` | Combined logic |

## Requirements

- [Claude CLI](https://claude.ai/code) installed and authenticated
- Git 2.x+
- Linux/macOS (Windows support planned)

## License

MIT
