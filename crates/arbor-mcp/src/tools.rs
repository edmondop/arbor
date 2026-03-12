use {
    crate::{ArborMcp, string_error},
    arbor_core::task::{TaskExecution, TaskInfo},
    arbor_daemon_client::{
        CommitWorktreeRequest, CreateTerminalRequest, CreateTerminalResponse,
        CreateWorktreeRequest, DeleteWorktreeRequest, GitActionResponse, HealthResponse,
        PushWorktreeRequest, TerminalResizeRequest, TerminalSignalRequest,
        WorktreeMutationResponse,
    },
    rmcp::{
        Json,
        handler::server::{router::tool::ToolRouter, wrapper::Parameters},
        tool, tool_router,
    },
    schemars::JsonSchema,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProcessNameInput {
    pub name: String,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(default)]
pub struct WorktreeListInput {
    pub repo_root: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TerminalReadInput {
    pub session_id: String,
    pub max_lines: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TerminalWriteInput {
    pub session_id: String,
    pub data: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ChangesInput {
    pub path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TerminalTargetInput {
    pub session_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CommitInput {
    pub path: String,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PushInput {
    pub path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TerminalResizeInput {
    pub session_id: String,
    pub cols: u16,
    pub rows: u16,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TerminalSignalInput {
    pub session_id: String,
    pub signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ActionStatus {
    pub ok: bool,
    pub message: String,
}

impl ActionStatus {
    pub(crate) fn ok(message: String) -> Self {
        Self { ok: true, message }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RepositoriesOutput {
    pub repositories: Vec<arbor_daemon_client::RepositoryDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WorktreesOutput {
    pub worktrees: Vec<arbor_daemon_client::WorktreeDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ChangedFilesOutput {
    pub files: Vec<arbor_daemon_client::ChangedFileDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TerminalsOutput {
    pub terminals: Vec<arbor_core::daemon::DaemonSessionRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentActivityOutput {
    pub sessions: Vec<arbor_daemon_client::AgentSessionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessesOutput {
    pub processes: Vec<arbor_core::process::ProcessInfo>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TaskNameInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TasksOutput {
    pub tasks: Vec<TaskInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TaskHistoryOutput {
    pub executions: Vec<TaskExecution>,
}

impl ArborMcp {
    pub(crate) fn create_tool_router() -> ToolRouter<Self> {
        Self::tool_router()
    }
}

#[tool_router(router = tool_router)]
impl ArborMcp {
    #[tool(description = "Get Arbor daemon health and version information")]
    pub async fn health(&self) -> Result<Json<HealthResponse>, String> {
        self.daemon.health().map(Json).map_err(string_error)
    }

    #[tool(description = "List Arbor repositories known to the daemon")]
    pub async fn list_repositories(&self) -> Result<Json<RepositoriesOutput>, String> {
        self.daemon
            .list_repositories()
            .map(|repositories| Json(RepositoriesOutput { repositories }))
            .map_err(string_error)
    }

    #[tool(description = "List Arbor worktrees, optionally filtered by repository root")]
    pub async fn list_worktrees(
        &self,
        input: Parameters<WorktreeListInput>,
    ) -> Result<Json<WorktreesOutput>, String> {
        let repo_root = input
            .0
            .repo_root
            .as_deref()
            .filter(|value| !value.trim().is_empty());
        self.daemon
            .list_worktrees(repo_root)
            .map(|worktrees| Json(WorktreesOutput { worktrees }))
            .map_err(string_error)
    }

    #[tool(description = "Create a git worktree through Arbor's daemon API")]
    pub async fn create_worktree(
        &self,
        input: Parameters<CreateWorktreeRequest>,
    ) -> Result<Json<WorktreeMutationResponse>, String> {
        self.daemon
            .create_worktree(&input.0)
            .map(Json)
            .map_err(string_error)
    }

    #[tool(description = "Delete a non-primary git worktree through Arbor's daemon API")]
    pub async fn delete_worktree(
        &self,
        input: Parameters<DeleteWorktreeRequest>,
    ) -> Result<Json<WorktreeMutationResponse>, String> {
        self.daemon
            .delete_worktree(&input.0)
            .map(Json)
            .map_err(string_error)
    }

    #[tool(description = "List changed files in a worktree")]
    pub async fn list_changed_files(
        &self,
        input: Parameters<ChangesInput>,
    ) -> Result<Json<ChangedFilesOutput>, String> {
        self.daemon
            .list_changed_files(&input.0.path)
            .map(|files| Json(ChangedFilesOutput { files }))
            .map_err(string_error)
    }

    #[tool(
        description = "Create a git commit in a worktree, auto-generating a message when omitted"
    )]
    pub async fn commit_worktree(
        &self,
        input: Parameters<CommitInput>,
    ) -> Result<Json<GitActionResponse>, String> {
        self.daemon
            .commit_worktree(&CommitWorktreeRequest {
                path: input.0.path.clone(),
                message: input.0.message.clone(),
            })
            .map(Json)
            .map_err(string_error)
    }

    #[tool(description = "Push the current branch for a worktree to origin")]
    pub async fn push_worktree(
        &self,
        input: Parameters<PushInput>,
    ) -> Result<Json<GitActionResponse>, String> {
        self.daemon
            .push_worktree(&PushWorktreeRequest {
                path: input.0.path.clone(),
            })
            .map(Json)
            .map_err(string_error)
    }

    #[tool(description = "List daemon-managed terminal sessions")]
    pub async fn list_terminals(&self) -> Result<Json<TerminalsOutput>, String> {
        self.daemon
            .list_terminals()
            .map(|terminals| Json(TerminalsOutput { terminals }))
            .map_err(string_error)
    }

    #[tool(description = "Create or attach to a daemon-managed terminal session")]
    pub async fn create_terminal(
        &self,
        input: Parameters<CreateTerminalRequest>,
    ) -> Result<Json<CreateTerminalResponse>, String> {
        self.daemon
            .create_terminal(&input.0)
            .map(Json)
            .map_err(string_error)
    }

    #[tool(description = "Read terminal output for one daemon-managed session")]
    pub async fn read_terminal_output(
        &self,
        input: Parameters<TerminalReadInput>,
    ) -> Result<Json<arbor_core::daemon::TerminalSnapshot>, String> {
        self.daemon
            .read_terminal_output(&input.0.session_id, input.0.max_lines)
            .map(Json)
            .map_err(string_error)
    }

    #[tool(description = "Write UTF-8 input bytes to a daemon-managed terminal session")]
    pub async fn write_terminal_input(
        &self,
        input: Parameters<TerminalWriteInput>,
    ) -> Result<Json<ActionStatus>, String> {
        self.daemon
            .write_terminal_input(&input.0.session_id, input.0.data.as_bytes())
            .map_err(string_error)?;
        Ok(Json(ActionStatus::ok(format!(
            "sent input to terminal `{}`",
            input.0.session_id
        ))))
    }

    #[tool(description = "Resize a daemon-managed terminal session")]
    pub async fn resize_terminal(
        &self,
        input: Parameters<TerminalResizeInput>,
    ) -> Result<Json<ActionStatus>, String> {
        self.daemon
            .resize_terminal(&input.0.session_id, &TerminalResizeRequest {
                cols: input.0.cols,
                rows: input.0.rows,
            })
            .map_err(string_error)?;
        Ok(Json(ActionStatus::ok(format!(
            "resized terminal `{}` to {}x{}",
            input.0.session_id, input.0.cols, input.0.rows
        ))))
    }

    #[tool(description = "Send a signal to a daemon-managed terminal session")]
    pub async fn signal_terminal(
        &self,
        input: Parameters<TerminalSignalInput>,
    ) -> Result<Json<ActionStatus>, String> {
        self.daemon
            .signal_terminal(&input.0.session_id, &TerminalSignalRequest {
                signal: input.0.signal.clone(),
            })
            .map_err(string_error)?;
        Ok(Json(ActionStatus::ok(format!(
            "sent {} to terminal `{}`",
            input.0.signal, input.0.session_id
        ))))
    }

    #[tool(description = "Detach from a daemon-managed terminal session without killing it")]
    pub async fn detach_terminal(
        &self,
        input: Parameters<TerminalTargetInput>,
    ) -> Result<Json<ActionStatus>, String> {
        self.daemon
            .detach_terminal(&input.0.session_id)
            .map_err(string_error)?;
        Ok(Json(ActionStatus::ok(format!(
            "detached terminal `{}`",
            input.0.session_id
        ))))
    }

    #[tool(description = "Kill and remove a daemon-managed terminal session")]
    pub async fn kill_terminal(
        &self,
        input: Parameters<TerminalTargetInput>,
    ) -> Result<Json<ActionStatus>, String> {
        self.daemon
            .kill_terminal(&input.0.session_id)
            .map_err(string_error)?;
        Ok(Json(ActionStatus::ok(format!(
            "killed terminal `{}`",
            input.0.session_id
        ))))
    }

    #[tool(description = "List current agent activity known to Arbor")]
    pub async fn list_agent_activity(&self) -> Result<Json<AgentActivityOutput>, String> {
        self.daemon
            .list_agent_activity()
            .map(|sessions| Json(AgentActivityOutput { sessions }))
            .map_err(string_error)
    }

    #[tool(description = "List Arbor managed processes")]
    pub async fn list_processes(&self) -> Result<Json<ProcessesOutput>, String> {
        self.daemon
            .list_processes()
            .map(|processes| Json(ProcessesOutput { processes }))
            .map_err(string_error)
    }

    #[tool(description = "Start all Arbor managed processes configured for this daemon")]
    pub async fn start_all_processes(&self) -> Result<Json<ProcessesOutput>, String> {
        self.daemon
            .start_all_processes()
            .map(|processes| Json(ProcessesOutput { processes }))
            .map_err(string_error)
    }

    #[tool(description = "Stop all Arbor managed processes currently running")]
    pub async fn stop_all_processes(&self) -> Result<Json<ProcessesOutput>, String> {
        self.daemon
            .stop_all_processes()
            .map(|processes| Json(ProcessesOutput { processes }))
            .map_err(string_error)
    }

    #[tool(description = "Start one Arbor managed process by name")]
    pub async fn start_process(
        &self,
        input: Parameters<ProcessNameInput>,
    ) -> Result<Json<arbor_core::process::ProcessInfo>, String> {
        self.daemon
            .start_process(&input.0.name)
            .map(Json)
            .map_err(string_error)
    }

    #[tool(description = "Stop one Arbor managed process by name")]
    pub async fn stop_process(
        &self,
        input: Parameters<ProcessNameInput>,
    ) -> Result<Json<arbor_core::process::ProcessInfo>, String> {
        self.daemon
            .stop_process(&input.0.name)
            .map(Json)
            .map_err(string_error)
    }

    #[tool(description = "Restart one Arbor managed process by name")]
    pub async fn restart_process(
        &self,
        input: Parameters<ProcessNameInput>,
    ) -> Result<Json<arbor_core::process::ProcessInfo>, String> {
        self.daemon
            .restart_process(&input.0.name)
            .map(Json)
            .map_err(string_error)
    }

    #[tool(description = "List Arbor scheduled tasks configured in arbor.toml")]
    pub async fn list_tasks(&self) -> Result<Json<TasksOutput>, String> {
        self.daemon
            .list_tasks()
            .map(|tasks| Json(TasksOutput { tasks }))
            .map_err(string_error)
    }

    #[tool(description = "Manually trigger a scheduled task by name, ignoring its cron schedule")]
    pub async fn run_task(
        &self,
        input: Parameters<TaskNameInput>,
    ) -> Result<Json<TaskInfo>, String> {
        self.daemon
            .run_task(&input.0.name)
            .map(Json)
            .map_err(string_error)
    }

    #[tool(description = "Get execution history for a scheduled task")]
    pub async fn task_history(
        &self,
        input: Parameters<TaskNameInput>,
    ) -> Result<Json<TaskHistoryOutput>, String> {
        self.daemon
            .task_history(&input.0.name)
            .map(|executions| Json(TaskHistoryOutput { executions }))
            .map_err(string_error)
    }
}
