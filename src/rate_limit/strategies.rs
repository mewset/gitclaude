use anyhow::Result;
use std::path::Path;
use std::time::{Duration, SystemTime};

use crate::config::RateLimitConfig;

/// Decision from rate limiter
#[derive(Debug)]
pub enum RateLimitDecision {
    /// Proceed with Claude call
    Run,
    /// Skip this call
    Skip { reason: String },
    /// Wait before running
    Debounce { seconds: u64 },
    /// Add to batch
    Batch,
}

/// Check if we should run based on rate limiting config
pub fn should_run(config: &RateLimitConfig, repo_root: &Path) -> Result<RateLimitDecision> {
    let state = load_state(repo_root)?;

    match config.strategy.as_str() {
        "debounce" => check_debounce(config, &state),
        "cooldown" => check_cooldown(config, &state),
        "batch" => check_batch(config, &state),
        "smart" => check_smart(config, &state),
        "none" | _ => Ok(RateLimitDecision::Run),
    }
}

/// Record that we ran Claude
pub fn record_run(repo_root: &Path) -> Result<()> {
    let state_file = state_file_path(repo_root);

    let state = RateLimitState {
        last_run: Some(SystemTime::now()),
        runs_this_hour: count_runs_this_hour(repo_root)? + 1,
        pending_batch: vec![],
    };

    let content = serde_json::to_string(&state)?;
    std::fs::write(state_file, content)?;

    Ok(())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct RateLimitState {
    last_run: Option<SystemTime>,
    runs_this_hour: u32,
    pending_batch: Vec<String>,
}

fn load_state(repo_root: &Path) -> Result<RateLimitState> {
    let state_file = state_file_path(repo_root);

    if !state_file.exists() {
        return Ok(RateLimitState::default());
    }

    let content = std::fs::read_to_string(state_file)?;
    let state: RateLimitState = serde_json::from_str(&content)?;

    Ok(state)
}

fn state_file_path(repo_root: &Path) -> std::path::PathBuf {
    repo_root.join(".gitclaude").join(".state.json")
}

fn check_debounce(config: &RateLimitConfig, state: &RateLimitState) -> Result<RateLimitDecision> {
    if let Some(last_run) = state.last_run {
        let elapsed = last_run.elapsed().unwrap_or_default();
        let debounce = Duration::from_secs(config.debounce_seconds);

        if elapsed < debounce {
            let remaining = debounce.saturating_sub(elapsed);
            return Ok(RateLimitDecision::Debounce {
                seconds: remaining.as_secs(),
            });
        }
    }

    Ok(RateLimitDecision::Run)
}

fn check_cooldown(config: &RateLimitConfig, state: &RateLimitState) -> Result<RateLimitDecision> {
    if let Some(last_run) = state.last_run {
        let elapsed = last_run.elapsed().unwrap_or_default();
        let cooldown = Duration::from_secs(config.cooldown_minutes * 60);

        if elapsed < cooldown {
            return Ok(RateLimitDecision::Skip {
                reason: format!(
                    "Cooldown active ({} min remaining)",
                    (cooldown.saturating_sub(elapsed)).as_secs() / 60
                ),
            });
        }
    }

    Ok(RateLimitDecision::Run)
}

fn check_batch(_config: &RateLimitConfig, _state: &RateLimitState) -> Result<RateLimitDecision> {
    // TODO: Implement batch logic
    Ok(RateLimitDecision::Batch)
}

fn check_smart(config: &RateLimitConfig, state: &RateLimitState) -> Result<RateLimitDecision> {
    // Check max runs per hour
    if config.max_runs_per_hour > 0 && state.runs_this_hour >= config.max_runs_per_hour {
        return Ok(RateLimitDecision::Skip {
            reason: format!("Max {} runs per hour reached", config.max_runs_per_hour),
        });
    }

    // Apply debounce
    check_debounce(config, state)
}

fn count_runs_this_hour(_repo_root: &Path) -> Result<u32> {
    // TODO: Count actual runs from logs
    Ok(0)
}
