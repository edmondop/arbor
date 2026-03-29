#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    Working,
    Waiting,
    Done,
}

/// A persisted agent session record, used for cold-start hydration.
#[derive(Debug, Clone)]
pub struct AgentSessionRecord {
    pub session_id: String,
    pub cwd: String,
    pub state: AgentState,
    pub updated_at_unix_ms: u64,
    pub metadata: Option<serde_json::Value>,
    /// OS process ID of the agent — used to filter out dead sessions.
    pub pid: Option<u32>,
}

/// Pluggable persistence backend for agent sessions.
///
/// Implementations can back onto flat files, SQLite, etc.
/// The daemon loads all sessions on startup and optionally persists updates.
pub trait AgentSessionStore: Send + Sync {
    /// Load all persisted sessions. Implementations should filter out
    /// clearly stale entries (e.g. dead PIDs, expired timestamps).
    fn load_all(&self) -> Result<Vec<AgentSessionRecord>, Box<dyn std::error::Error>>;

    /// Persist a session update. Implementations may no-op if persistence
    /// is handled externally (e.g. hooks write their own files).
    fn save(&self, record: &AgentSessionRecord) -> Result<(), Box<dyn std::error::Error>>;
}
