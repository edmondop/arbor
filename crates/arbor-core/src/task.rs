use {
    schemars::JsonSchema,
    serde::{Deserialize, Serialize},
};

/// Status of a scheduled task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum TaskStatus {
    Idle,
    Running,
    Disabled,
}

/// Which AI agent to invoke when a task trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum AgentKind {
    Claude,
    Codex,
}

/// Runtime information about a scheduled task.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TaskInfo {
    pub name: String,
    pub schedule: String,
    pub command: String,
    pub status: TaskStatus,
    pub has_trigger: bool,
    pub last_run_unix_ms: Option<u64>,
    pub last_exit_code: Option<i32>,
    pub next_run_unix_ms: Option<u64>,
    pub run_count: u32,
}

/// A single execution record for history tracking.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TaskExecution {
    pub task_name: String,
    pub started_at_unix_ms: u64,
    pub finished_at_unix_ms: Option<u64>,
    pub exit_code: Option<i32>,
    pub stdout_tail: Option<String>,
    pub agent_spawned: bool,
}
