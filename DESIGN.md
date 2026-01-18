# Gitclaude - Design Document

> En systemdaemon som lyssnar på git-kommandon och triggar Claude Code sessioner

## Vision

Gitclaude är ett verktyg som automatiskt reagerar på git-events (commits, push, merge etc.) och spinner upp Claude Code för att ge feedback, code reviews, changelog-generering och mer.

**Nyckelprinciper:**
- Användarens Claude-prenumeration via CLI (ingen API-kostnad)
- Helt konfigurerbart beteende
- Token-effektivt
- Fungerar både globalt och per-repo

---

## Arkitektur

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

## Komponenter

### CLI Parser
Hanterar alla användarkommandon via `clap`.

### Config Loader
Laddar och mergar konfiguration från:
1. Repo-specifik: `.gitclaude/config.toml`
2. Global: `~/.config/gitclaude/config.toml`

### Hooks Manager
Installerar och hanterar git hooks:
- `post-commit`
- `post-push`
- `pre-commit`
- `post-merge`
- `post-checkout`

### Templates Engine
Renderar prompt-templates med Handlebars:
- `{{commit_message}}`
- `{{diff}}`
- `{{author}}`
- `{{affected_files}}`
- etc.

### Rate Limiter
Förhindrar token-spam vid hög commit-aktivitet.

### Context Builder
Bygger intelligent kontext baserat på konfigurerad nivå.

### Monorepo Detector
Identifierar monorepo-struktur och scoped kontext.

### Claude Bridge
Kör Claude CLI med rätt argument och kontext.

### Output Handlers
Hanterar Claude-svar: notifikationer, filer, terminal, git notes.

---

## CLI-kommandon

```bash
gitclaude init              # Interaktiv onboarding
gitclaude enable            # Aktivera i current repo
gitclaude disable           # Inaktivera i current repo
gitclaude config            # Öppna/editera config
gitclaude config --global   # Editera global config
gitclaude status            # Visa aktiv konfiguration
gitclaude logs              # Visa tidigare responses
gitclaude run <event>       # Manuellt trigga event
gitclaude templates         # Hantera templates
gitclaude templates list    # Lista templates
gitclaude templates edit    # Editera template
```

---

## Git Events

| Event | Trigger | Default Template | Use Case |
|-------|---------|------------------|----------|
| `post-commit` | Efter commit | `review` | Code review |
| `post-push` | Efter push | `changelog` | Changelog-generering |
| `pre-commit` | Innan commit | `validate` | Validering (blocking) |
| `post-merge` | Efter merge | `summary` | Merge-sammanfattning |
| `post-checkout` | Efter checkout | `context` | Branch-kontext |

---

## Kontextnivåer

| Nivå | Inkluderar | Tokens (approx) | Use case |
|------|-----------|-----------------|----------|
| `minimal` | Commit message | ~100 | Snabb feedback |
| `standard` | Message + diff | ~1000-4000 | Standard review |
| `extended` | + relaterade filer, senaste commits | ~4000-8000 | Djup analys |
| `full` | Hela repo via claude | Varierar | Arkitektur-feedback |

### Smart Truncation

För stora diffar:
1. Beräkna complexity score per fil
2. Prioritera efter filtyp och ändringsmängd
3. Inkludera diff --stat för overview
4. Truncera vid token-budget

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

### Strategier

| Strategi | Beskrivning |
|----------|-------------|
| `debounce` | Vänta X sekunder efter senaste commit |
| `batch` | Samla commits, kör en gång |
| `cooldown` | Minsta tid mellan körningar |
| `confirm` | Fråga användaren |
| `smart` | Kombinerad logik |

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
1. Kolla .gitclaude/config.toml i repo
   └─ Om finns: använd repo-specifik config

2. Annars: kolla ~/.config/gitclaude/config.toml
   └─ Om global_listen = true: använd global config
   └─ Om repo är i ignore_repos[]: skip

3. Merge: repo-config överskrider global
```

---

## Monorepo-stöd

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

- Detektera affected packages från diff
- Endast skicka relevant kontext
- Stöd för nested `.gitclaude.toml`

---

## Output Handlers

| Handler | Beskrivning | Config |
|---------|-------------|--------|
| `notify` | Desktop-notifikation | urgency, timeout |
| `file` | Spara till fil | path, format |
| `terminal` | Öppna i terminal | terminal app |
| `git-note` | Lägg till som git note | - |
| `clipboard` | Kopiera till clipboard | - |
| `session` | Öppna interaktiv Claude | auto_open |

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

## Filstruktur

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
│  🎉 Välkommen till gitclaude!                       │
└─────────────────────────────────────────────────────┘

? Ska gitclaude lyssna globalt eller bara i specifika repos?
  ○ Globalt (alla git repos, kan exkludera vissa)
  ● Endast aktiverade repos

? Vilka git-events vill du reagera på?
  ☑ post-commit
  ☐ post-push
  ☐ pre-commit
  ☐ post-merge

? Hur ska Claude köras?
  ○ Synkront
  ● Asynkront
  ○ Fråga varje gång

? Output-format?
  ☑ Desktop-notifikation
  ☐ Spara till fil
  ☐ Öppna interaktiv session

? Kontextnivå?
  ○ Minimal
  ● Standard
  ○ Extended
  ○ Full

? Rate limiting?
  ● Debounce (30s)
  ○ Batch
  ○ Cooldown
  ○ Smart

✅ Konfiguration sparad!
```

---

## Implementation Order

### Fas 1: Foundation
- [ ] Projekt-setup (Cargo.toml, struktur)
- [ ] CLI parsing med clap
- [ ] Config types och loader
- [ ] Basic hook installation

### Fas 2: Core
- [ ] Context builder (minimal + standard)
- [ ] Claude bridge
- [ ] Output: notify
- [ ] Rate limiting: debounce

### Fas 3: Features
- [ ] Interaktiv onboarding
- [ ] Templates engine
- [ ] Extended context
- [ ] Monorepo detection
- [ ] Alla output handlers

### Fas 4: Polish
- [ ] Error handling
- [ ] Logging
- [ ] Tests
- [ ] Documentation
- [ ] Release automation

---

## Open Questions (Resolved)

1. ✅ Språk: Rust
2. ✅ Global/per-repo: Båda, konfigurerbart
3. ✅ Stora diffar: Smart truncation + prioritering
4. ✅ Rate limiting: Multiple strategies, konfigurerbart
5. ✅ Monorepo: Automatisk detection + scoped context
