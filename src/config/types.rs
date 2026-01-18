use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub general: GeneralConfig,
    pub global: GlobalConfig,
    pub events: HashMap<String, EventConfig>,
    pub context: ContextConfig,
    pub rate_limit: RateLimitConfig,
    pub output: OutputConfig,
    pub monorepo: MonorepoConfig,
    pub templates: TemplatesConfig,
    pub claude: ClaudeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_true")]
    pub async_mode: bool,
    #[serde(default = "default_true")]
    pub notify: bool,
    #[serde(default)]
    pub log_responses: bool,
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            async_mode: true,
            notify: true,
            log_responses: false,
            log_level: "info".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalConfig {
    #[serde(default)]
    pub listen_globally: bool,
    #[serde(default = "default_profile")]
    pub default_profile: String,
    #[serde(default)]
    pub ignore_repos: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub template: String,
    #[serde(default = "default_context")]
    pub context: String,
    #[serde(default)]
    pub output: Vec<String>,
    #[serde(default)]
    pub blocking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    #[serde(default = "default_strategy")]
    pub strategy: String,
    #[serde(default)]
    pub smart: SmartContextConfig,
    #[serde(default)]
    pub exclude: ExcludeConfig,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            max_tokens: 4000,
            strategy: "smart".to_string(),
            smart: SmartContextConfig::default(),
            exclude: ExcludeConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SmartContextConfig {
    #[serde(default)]
    pub always_include: Vec<String>,
    #[serde(default)]
    pub priority_by_extension: HashMap<String, u8>,
    #[serde(default = "default_true")]
    pub include_diff_stat: bool,
    #[serde(default = "default_truncate_at")]
    pub truncate_at: usize,
    #[serde(default = "default_recent_commits")]
    pub include_recent_commits: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExcludeConfig {
    #[serde(default)]
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    #[serde(default = "default_rate_strategy")]
    pub strategy: String,
    #[serde(default = "default_debounce")]
    pub debounce_seconds: u64,
    #[serde(default = "default_batch_window")]
    pub batch_window_seconds: u64,
    #[serde(default = "default_cooldown")]
    pub cooldown_minutes: u64,
    #[serde(default = "default_max_runs")]
    pub max_runs_per_hour: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            strategy: "debounce".to_string(),
            debounce_seconds: 30,
            batch_window_seconds: 120,
            cooldown_minutes: 5,
            max_runs_per_hour: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutputConfig {
    #[serde(default)]
    pub notify: NotifyConfig,
    #[serde(default)]
    pub file: FileOutputConfig,
    #[serde(default)]
    pub terminal: TerminalConfig,
    #[serde(default)]
    pub session: SessionConfig,
    #[serde(default)]
    pub clipboard: ClipboardConfig,
    #[serde(default)]
    pub git_note: GitNoteConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyConfig {
    #[serde(default = "default_urgency")]
    pub urgency: String,
    #[serde(default = "default_timeout")]
    pub timeout: u32,
    #[serde(default = "default_app_name")]
    pub app_name: String,
}

impl Default for NotifyConfig {
    fn default() -> Self {
        Self {
            urgency: "normal".to_string(),
            timeout: 5000,
            app_name: "gitclaude".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOutputConfig {
    #[serde(default = "default_file_path")]
    pub path: PathBuf,
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_true")]
    pub timestamp: bool,
}

impl Default for FileOutputConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from(".gitclaude/responses/"),
            format: "markdown".to_string(),
            timestamp: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalConfig {
    #[serde(default)]
    pub auto_open: bool,
    #[serde(default = "default_terminal")]
    pub terminal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionConfig {
    #[serde(default)]
    pub auto_open: bool,
    #[serde(default = "default_working_dir")]
    pub working_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClipboardConfig {
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitNoteConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_note_ref")]
    pub note_ref: String,
}

impl Default for GitNoteConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            note_ref: "refs/notes/claude".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonorepoConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub detect_affected: bool,
    #[serde(default = "default_true")]
    pub scope_context: bool,
    #[serde(default = "default_package_dirs")]
    pub package_dirs: Vec<String>,
}

impl Default for MonorepoConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            detect_affected: true,
            scope_context: true,
            package_dirs: vec![
                "packages".to_string(),
                "apps".to_string(),
                "libs".to_string(),
                "crates".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatesConfig {
    #[serde(default = "default_templates_dir")]
    pub directory: PathBuf,
    #[serde(default = "default_true")]
    pub fallback_builtin: bool,
}

impl Default for TemplatesConfig {
    fn default() -> Self {
        Self {
            directory: PathBuf::from("~/.config/gitclaude/templates/"),
            fallback_builtin: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeConfig {
    pub binary: Option<PathBuf>,
    #[serde(default)]
    pub extra_args: Vec<String>,
    #[serde(default = "default_claude_timeout")]
    pub timeout: u64,
    #[serde(default = "default_format")]
    pub output_format: String,
}

impl Default for ClaudeConfig {
    fn default() -> Self {
        Self {
            binary: None,
            extra_args: vec![],
            timeout: 120,
            output_format: "markdown".to_string(),
        }
    }
}

// Default value helpers
fn default_true() -> bool { true }
fn default_log_level() -> String { "info".to_string() }
fn default_profile() -> String { "standard".to_string() }
fn default_context() -> String { "standard".to_string() }
fn default_max_tokens() -> usize { 4000 }
fn default_strategy() -> String { "smart".to_string() }
fn default_truncate_at() -> usize { 500 }
fn default_recent_commits() -> usize { 3 }
fn default_rate_strategy() -> String { "debounce".to_string() }
fn default_debounce() -> u64 { 30 }
fn default_batch_window() -> u64 { 120 }
fn default_cooldown() -> u64 { 5 }
fn default_max_runs() -> u32 { 10 }
fn default_urgency() -> String { "normal".to_string() }
fn default_timeout() -> u32 { 5000 }
fn default_app_name() -> String { "gitclaude".to_string() }
fn default_file_path() -> PathBuf { PathBuf::from(".gitclaude/responses/") }
fn default_format() -> String { "markdown".to_string() }
fn default_terminal() -> String { "alacritty".to_string() }
fn default_working_dir() -> String { "repo".to_string() }
fn default_note_ref() -> String { "refs/notes/claude".to_string() }
fn default_package_dirs() -> Vec<String> {
    vec!["packages".to_string(), "apps".to_string(), "libs".to_string(), "crates".to_string()]
}
fn default_templates_dir() -> PathBuf { PathBuf::from("~/.config/gitclaude/templates/") }
fn default_claude_timeout() -> u64 { 120 }
