use {
    arbor_core::agent::{AgentSessionRecord, AgentSessionStore, AgentState},
    serde::{Deserialize, Serialize},
    std::path::{Path, PathBuf},
};

/// Reads agent session state from JSON files in a directory.
pub struct FileAgentSessionStore {
    dir: PathBuf,
}

impl FileAgentSessionStore {
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    /// Resolve directory using `AGENT_SESSION_STATE_DIR`, then `XDG_STATE_HOME`,
    /// falling back to `~/.local/state/agent-sessions/`.
    pub fn default_dir() -> Option<PathBuf> {
        if let Ok(dir) = std::env::var("AGENT_SESSION_STATE_DIR") {
            return Some(PathBuf::from(dir));
        }
        if let Ok(state_home) = std::env::var("XDG_STATE_HOME") {
            return Some(PathBuf::from(state_home).join("agent-sessions"));
        }
        std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join(".local/state/agent-sessions"))
    }
}

impl AgentSessionStore for FileAgentSessionStore {
    fn load_all(&self) -> Result<Vec<AgentSessionRecord>, Box<dyn std::error::Error>> {
        let entries = match std::fs::read_dir(&self.dir) {
            Ok(entries) => entries,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(e.into()),
        };

        let mut records = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json") {
                match load_state_file(&path) {
                    Ok(Some(record)) => records.push(record),
                    Ok(None) => {},
                    Err(e) => {
                        tracing::debug!(path = %path.display(), error = %e, "skipping state file");
                    },
                }
            }
        }

        Ok(records)
    }

    fn save(&self, _record: &AgentSessionRecord) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

const DONE_EXPIRY_SECS: i64 = 600;

#[derive(Deserialize)]
struct RawSessionState {
    session_id: String,
    #[serde(default)]
    cwd: String,
    #[serde(default = "default_status")]
    status: String,
    pid: Option<u32>,
    updated_at: Option<String>,
    project: Option<String>,
    branch: Option<String>,
    workspace: Option<String>,
    blocked_on: Option<String>,
    message: Option<String>,
}

fn default_status() -> String {
    "idle".to_owned()
}

#[derive(Serialize)]
struct SessionMetadata<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocked_on: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid: Option<String>,
}

fn load_state_file(path: &Path) -> Result<Option<AgentSessionRecord>, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path)?;
    let raw: RawSessionState = serde_json::from_str(&data)?;

    if let Some(pid) = raw.pid
        && !is_process_alive(pid)
    {
        return Ok(None);
    }

    if matches!(raw.status.as_str(), "done" | "idle")
        && let Some(ref ts) = raw.updated_at
        && let Ok(dt) = chrono::DateTime::parse_from_rfc3339(ts)
    {
        let age = chrono::Utc::now().signed_duration_since(dt);
        if age.num_seconds() > DONE_EXPIRY_SECS {
            return Ok(None);
        }
    }

    let state = match raw.status.as_str() {
        "working" => AgentState::Working,
        "done" | "idle" => AgentState::Done,
        _ => AgentState::Waiting,
    };

    let updated_at_unix_ms = raw
        .updated_at
        .as_deref()
        .and_then(|ts| chrono::DateTime::parse_from_rfc3339(ts).ok())
        .map(|dt| dt.timestamp_millis() as u64)
        .unwrap_or(0);

    let meta = SessionMetadata {
        project: raw.project.as_deref().filter(|s| !s.is_empty()),
        branch: raw.branch.as_deref().filter(|s| !s.is_empty()),
        workspace: raw.workspace.as_deref().filter(|s| !s.is_empty()),
        blocked_on: raw.blocked_on.as_deref().filter(|s| !s.is_empty()),
        message: raw.message.as_deref().filter(|s| !s.is_empty()),
        pid: raw.pid.map(|p| p.to_string()),
    };
    let metadata = serde_json::to_value(&meta)
        .ok()
        .filter(|v| v.as_object().is_some_and(|m| !m.is_empty()));

    Ok(Some(AgentSessionRecord {
        session_id: raw.session_id,
        cwd: raw.cwd,
        state,
        updated_at_unix_ms,
        metadata,
        pid: raw.pid,
    }))
}

fn is_process_alive(pid: u32) -> bool {
    #[cfg(unix)]
    {
        if PathBuf::from(format!("/proc/{pid}")).exists() {
            return true;
        }
        std::process::Command::new("kill")
            .args(["-0", &pid.to_string()])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(true)
    }
    #[cfg(not(unix))]
    {
        let _ = pid;
        true
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn load_all_returns_empty_for_missing_dir() {
        let store = FileAgentSessionStore::new(PathBuf::from("/tmp/nonexistent-agent-state-dir"));
        let records = store.load_all().unwrap();
        assert!(records.is_empty());
    }

    #[test]
    fn load_state_file_parses_working() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test-session.json");
        std::fs::write(
            &path,
            serde_json::json!({
                "session_id": "abc-123",
                "cwd": "/home/user/project",
                "status": "working",
                "pid": std::process::id(),
                "updated_at": chrono::Utc::now().to_rfc3339(),
            })
            .to_string(),
        )
        .unwrap();

        let record = load_state_file(&path).unwrap().unwrap();
        assert_eq!(record.session_id, "abc-123");
        assert_eq!(record.state, AgentState::Working);
    }

    #[test]
    fn load_state_file_maps_blocked_to_waiting() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test-session.json");
        std::fs::write(
            &path,
            serde_json::json!({
                "session_id": "abc-456",
                "cwd": "/home/user/project",
                "status": "blocked",
                "blocked_on": "permission_prompt",
                "pid": std::process::id(),
                "updated_at": chrono::Utc::now().to_rfc3339(),
            })
            .to_string(),
        )
        .unwrap();

        let record = load_state_file(&path).unwrap().unwrap();
        assert_eq!(record.state, AgentState::Waiting);
        let meta = record.metadata.unwrap();
        assert_eq!(meta["blocked_on"], "permission_prompt");
    }

    #[test]
    fn load_state_file_filters_expired_done() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test-session.json");
        let old_time = chrono::Utc::now() - chrono::Duration::minutes(15);
        std::fs::write(
            &path,
            serde_json::json!({
                "session_id": "old-done",
                "cwd": "/tmp",
                "status": "done",
                "pid": std::process::id(),
                "updated_at": old_time.to_rfc3339(),
            })
            .to_string(),
        )
        .unwrap();

        let record = load_state_file(&path).unwrap();
        assert!(record.is_none());
    }
}
