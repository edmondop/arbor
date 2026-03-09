use {
    arbor_daemon_client::{
        AgentSessionDto, ChangedFileDto, CommitWorktreeRequest, CreateTerminalRequest,
        CreateTerminalResponse, CreateWorktreeRequest, DaemonClient, DaemonClientError,
        DeleteWorktreeRequest, GitActionResponse, HealthResponse, PushWorktreeRequest,
        RepositoryDto, TerminalResizeRequest, TerminalSignalRequest, WorktreeDto,
        WorktreeMutationResponse, default_mcp_resource_templates, default_mcp_resources,
        parse_terminal_snapshot_resource, parse_worktree_changes_resource, read_json_text_resource,
    },
    rmcp::{
        ErrorData, Json, RoleServer, ServerHandler, ServiceExt,
        handler::server::{router::tool::ToolRouter, wrapper::Parameters},
        model::{
            AnnotateAble, GetPromptRequestParams, GetPromptResult, Implementation,
            ListPromptsResult, ListResourceTemplatesResult, ListResourcesResult,
            PaginatedRequestParams, Prompt, PromptArgument, PromptMessage, PromptMessageRole,
            RawResource, RawResourceTemplate, ReadResourceRequestParams, ReadResourceResult,
            ResourceContents, ServerCapabilities, ServerInfo,
        },
        service::RequestContext,
        tool, tool_handler, tool_router,
    },
    schemars::JsonSchema,
    serde::{Deserialize, Serialize},
    std::{future::Future, sync::Arc},
};

pub trait DaemonApi: Send + Sync {
    fn health(&self) -> Result<HealthResponse, DaemonClientError>;
    fn list_repositories(&self) -> Result<Vec<RepositoryDto>, DaemonClientError>;
    fn list_worktrees(
        &self,
        repo_root: Option<&str>,
    ) -> Result<Vec<WorktreeDto>, DaemonClientError>;
    fn create_worktree(
        &self,
        request: &CreateWorktreeRequest,
    ) -> Result<WorktreeMutationResponse, DaemonClientError>;
    fn delete_worktree(
        &self,
        request: &DeleteWorktreeRequest,
    ) -> Result<WorktreeMutationResponse, DaemonClientError>;
    fn list_changed_files(&self, path: &str) -> Result<Vec<ChangedFileDto>, DaemonClientError>;
    fn commit_worktree(
        &self,
        request: &CommitWorktreeRequest,
    ) -> Result<GitActionResponse, DaemonClientError>;
    fn push_worktree(
        &self,
        request: &PushWorktreeRequest,
    ) -> Result<GitActionResponse, DaemonClientError>;
    fn list_terminals(
        &self,
    ) -> Result<Vec<arbor_core::daemon::DaemonSessionRecord>, DaemonClientError>;
    fn create_terminal(
        &self,
        request: &CreateTerminalRequest,
    ) -> Result<CreateTerminalResponse, DaemonClientError>;
    fn read_terminal_output(
        &self,
        session_id: &str,
        max_lines: Option<usize>,
    ) -> Result<arbor_core::daemon::TerminalSnapshot, DaemonClientError>;
    fn write_terminal_input(&self, session_id: &str, data: &[u8]) -> Result<(), DaemonClientError>;
    fn resize_terminal(
        &self,
        session_id: &str,
        request: &TerminalResizeRequest,
    ) -> Result<(), DaemonClientError>;
    fn signal_terminal(
        &self,
        session_id: &str,
        request: &TerminalSignalRequest,
    ) -> Result<(), DaemonClientError>;
    fn detach_terminal(&self, session_id: &str) -> Result<(), DaemonClientError>;
    fn kill_terminal(&self, session_id: &str) -> Result<(), DaemonClientError>;
    fn list_agent_activity(&self) -> Result<Vec<AgentSessionDto>, DaemonClientError>;
    fn list_processes(&self) -> Result<Vec<arbor_core::process::ProcessInfo>, DaemonClientError>;
    fn start_all_processes(
        &self,
    ) -> Result<Vec<arbor_core::process::ProcessInfo>, DaemonClientError>;
    fn stop_all_processes(
        &self,
    ) -> Result<Vec<arbor_core::process::ProcessInfo>, DaemonClientError>;
    fn start_process(
        &self,
        name: &str,
    ) -> Result<arbor_core::process::ProcessInfo, DaemonClientError>;
    fn stop_process(
        &self,
        name: &str,
    ) -> Result<arbor_core::process::ProcessInfo, DaemonClientError>;
    fn restart_process(
        &self,
        name: &str,
    ) -> Result<arbor_core::process::ProcessInfo, DaemonClientError>;
}

impl DaemonApi for DaemonClient {
    fn health(&self) -> Result<HealthResponse, DaemonClientError> {
        self.health()
    }

    fn list_repositories(&self) -> Result<Vec<RepositoryDto>, DaemonClientError> {
        self.list_repositories()
    }

    fn list_worktrees(
        &self,
        repo_root: Option<&str>,
    ) -> Result<Vec<WorktreeDto>, DaemonClientError> {
        self.list_worktrees(repo_root)
    }

    fn create_worktree(
        &self,
        request: &CreateWorktreeRequest,
    ) -> Result<WorktreeMutationResponse, DaemonClientError> {
        self.create_worktree(request)
    }

    fn delete_worktree(
        &self,
        request: &DeleteWorktreeRequest,
    ) -> Result<WorktreeMutationResponse, DaemonClientError> {
        self.delete_worktree(request)
    }

    fn list_changed_files(&self, path: &str) -> Result<Vec<ChangedFileDto>, DaemonClientError> {
        self.list_changed_files(path)
    }

    fn commit_worktree(
        &self,
        request: &CommitWorktreeRequest,
    ) -> Result<GitActionResponse, DaemonClientError> {
        self.commit_worktree(request)
    }

    fn push_worktree(
        &self,
        request: &PushWorktreeRequest,
    ) -> Result<GitActionResponse, DaemonClientError> {
        self.push_worktree(request)
    }

    fn list_terminals(
        &self,
    ) -> Result<Vec<arbor_core::daemon::DaemonSessionRecord>, DaemonClientError> {
        self.list_terminals()
    }

    fn create_terminal(
        &self,
        request: &CreateTerminalRequest,
    ) -> Result<CreateTerminalResponse, DaemonClientError> {
        self.create_terminal(request)
    }

    fn read_terminal_output(
        &self,
        session_id: &str,
        max_lines: Option<usize>,
    ) -> Result<arbor_core::daemon::TerminalSnapshot, DaemonClientError> {
        self.read_terminal_output(session_id, max_lines)
    }

    fn write_terminal_input(&self, session_id: &str, data: &[u8]) -> Result<(), DaemonClientError> {
        self.write_terminal_input(session_id, data)
    }

    fn resize_terminal(
        &self,
        session_id: &str,
        request: &TerminalResizeRequest,
    ) -> Result<(), DaemonClientError> {
        self.resize_terminal(session_id, request)
    }

    fn signal_terminal(
        &self,
        session_id: &str,
        request: &TerminalSignalRequest,
    ) -> Result<(), DaemonClientError> {
        self.signal_terminal(session_id, request)
    }

    fn detach_terminal(&self, session_id: &str) -> Result<(), DaemonClientError> {
        self.detach_terminal(session_id)
    }

    fn kill_terminal(&self, session_id: &str) -> Result<(), DaemonClientError> {
        self.kill_terminal(session_id)
    }

    fn list_agent_activity(&self) -> Result<Vec<AgentSessionDto>, DaemonClientError> {
        self.list_agent_activity()
    }

    fn list_processes(&self) -> Result<Vec<arbor_core::process::ProcessInfo>, DaemonClientError> {
        self.list_processes()
    }

    fn start_all_processes(
        &self,
    ) -> Result<Vec<arbor_core::process::ProcessInfo>, DaemonClientError> {
        self.start_all_processes()
    }

    fn stop_all_processes(
        &self,
    ) -> Result<Vec<arbor_core::process::ProcessInfo>, DaemonClientError> {
        self.stop_all_processes()
    }

    fn start_process(
        &self,
        name: &str,
    ) -> Result<arbor_core::process::ProcessInfo, DaemonClientError> {
        self.start_process(name)
    }

    fn stop_process(
        &self,
        name: &str,
    ) -> Result<arbor_core::process::ProcessInfo, DaemonClientError> {
        self.stop_process(name)
    }

    fn restart_process(
        &self,
        name: &str,
    ) -> Result<arbor_core::process::ProcessInfo, DaemonClientError> {
        self.restart_process(name)
    }
}

#[derive(Clone)]
pub struct ArborMcp {
    daemon: Arc<dyn DaemonApi>,
    tool_router: ToolRouter<Self>,
}

impl Default for ArborMcp {
    fn default() -> Self {
        Self::new()
    }
}

impl ArborMcp {
    pub fn new() -> Self {
        Self::with_client(Arc::new(DaemonClient::from_env()))
    }

    pub fn with_client(daemon: Arc<dyn DaemonApi>) -> Self {
        Self {
            daemon,
            tool_router: Self::tool_router(),
        }
    }

    pub fn read_resource_contents(&self, uri: &str) -> Result<ReadResourceResult, ErrorData> {
        let text = match uri {
            "arbor://health" => {
                read_json_text_resource(&self.daemon.health().map_err(map_daemon_error)?)
                    .map_err(map_daemon_error)?
            },
            "arbor://repositories" => {
                read_json_text_resource(&self.daemon.list_repositories().map_err(map_daemon_error)?)
                    .map_err(map_daemon_error)?
            },
            "arbor://worktrees" => read_json_text_resource(
                &self.daemon.list_worktrees(None).map_err(map_daemon_error)?,
            )
            .map_err(map_daemon_error)?,
            "arbor://processes" => {
                read_json_text_resource(&self.daemon.list_processes().map_err(map_daemon_error)?)
                    .map_err(map_daemon_error)?
            },
            "arbor://terminals" => {
                read_json_text_resource(&self.daemon.list_terminals().map_err(map_daemon_error)?)
                    .map_err(map_daemon_error)?
            },
            "arbor://agent-activity" => read_json_text_resource(
                &self
                    .daemon
                    .list_agent_activity()
                    .map_err(map_daemon_error)?,
            )
            .map_err(map_daemon_error)?,
            uri => {
                if let Some(path) = parse_worktree_changes_resource(uri) {
                    read_json_text_resource(
                        &self
                            .daemon
                            .list_changed_files(&path.display().to_string())
                            .map_err(map_daemon_error)?,
                    )
                    .map_err(map_daemon_error)?
                } else if let Some(session_id) = parse_terminal_snapshot_resource(uri) {
                    read_json_text_resource(
                        &self
                            .daemon
                            .read_terminal_output(&session_id, None)
                            .map_err(map_daemon_error)?,
                    )
                    .map_err(map_daemon_error)?
                } else {
                    return Err(ErrorData::resource_not_found(
                        format!("resource `{uri}` was not found"),
                        None,
                    ));
                }
            },
        };

        Ok(ReadResourceResult::new(vec![
            ResourceContents::text(text, uri).with_mime_type("application/json"),
        ]))
    }

    pub fn prompt_definitions(&self) -> Vec<Prompt> {
        vec![
            Prompt::new(
                "review-worktree",
                Some("Review the changes and runtime state for a worktree."),
                Some(vec![required_prompt_argument(
                    "path",
                    "Absolute worktree path to review.",
                )]),
            )
            .with_title("Review Worktree"),
            Prompt::new(
                "stabilize-process",
                Some("Investigate and stabilize an Arbor-managed process."),
                Some(vec![required_prompt_argument(
                    "name",
                    "Managed process name from Arbor.",
                )]),
            )
            .with_title("Stabilize Process"),
            Prompt::new(
                "recover-terminal",
                Some("Recover a stuck or failed daemon terminal session."),
                Some(vec![required_prompt_argument(
                    "session_id",
                    "Daemon terminal session id.",
                )]),
            )
            .with_title("Recover Terminal"),
        ]
    }

    pub fn prompt_response(
        &self,
        request: GetPromptRequestParams,
    ) -> Result<GetPromptResult, ErrorData> {
        match request.name.as_str() {
            "review-worktree" => {
                let path = required_argument(&request, "path")?;
                Ok(GetPromptResult::new(vec![
                    PromptMessage::new_text(
                        PromptMessageRole::User,
                        format!(
                            "Review the Arbor worktree at `{path}`. Inspect changed files, the current terminal state, and any managed processes that relate to this worktree."
                        ),
                    ),
                    PromptMessage::new_text(
                        PromptMessageRole::Assistant,
                        "Use `list_changed_files`, `list_terminals`, `read_terminal_output`, and `list_processes`. Prefer Arbor resources like `arbor://worktrees` and `arbor://processes` for context before changing anything.",
                    ),
                ])
                .with_description("Review one worktree using Arbor's daemon-backed state."))
            },
            "stabilize-process" => {
                let name = required_argument(&request, "name")?;
                Ok(GetPromptResult::new(vec![
                    PromptMessage::new_text(
                        PromptMessageRole::User,
                        format!(
                            "Investigate the Arbor-managed process `{name}` and stabilize it if needed."
                        ),
                    ),
                    PromptMessage::new_text(
                        PromptMessageRole::Assistant,
                        "Start with `list_processes` and `arbor://processes`, then inspect linked terminals. Use `restart_process`, `start_process`, or `stop_process` only after you understand the current state.",
                    ),
                ])
                .with_description("Troubleshoot one managed Arbor process."))
            },
            "recover-terminal" => {
                let session_id = required_argument(&request, "session_id")?;
                Ok(GetPromptResult::new(vec![
                    PromptMessage::new_text(
                        PromptMessageRole::User,
                        format!(
                            "Recover the Arbor terminal session `{session_id}` without losing useful context."
                        ),
                    ),
                    PromptMessage::new_text(
                        PromptMessageRole::Assistant,
                        "Read the terminal snapshot first. Prefer `write_terminal_input`, `signal_terminal`, and `detach_terminal`; use `kill_terminal` only if the session is unrecoverable.",
                    ),
                ])
                .with_description("Recover or clean up a daemon-managed terminal session."))
            },
            other => Err(ErrorData::invalid_params(
                format!("prompt `{other}` is not supported"),
                None,
            )),
        }
    }
}

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
    fn ok(message: String) -> Self {
        Self { ok: true, message }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RepositoriesOutput {
    pub repositories: Vec<RepositoryDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WorktreesOutput {
    pub worktrees: Vec<WorktreeDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ChangedFilesOutput {
    pub files: Vec<ChangedFileDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TerminalsOutput {
    pub terminals: Vec<arbor_core::daemon::DaemonSessionRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentActivityOutput {
    pub sessions: Vec<AgentSessionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessesOutput {
    pub processes: Vec<arbor_core::process::ProcessInfo>,
}

#[tool_handler]
impl ServerHandler for ArborMcp {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::default();
        info.capabilities = ServerCapabilities::builder()
            .enable_tools()
            .enable_resources()
            .enable_prompts()
            .build();
        info.server_info = Implementation::from_build_env();
        info.instructions = Some(
            "Arbor MCP server. Tools, prompts, and resources are backed by arbor-httpd. Configure ARBOR_DAEMON_URL for a non-default daemon address and ARBOR_DAEMON_AUTH_TOKEN for remote authenticated daemons."
                .to_owned(),
        );
        info
    }

    fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListResourcesResult, ErrorData>> + Send + '_ {
        std::future::ready({
            let result = ListResourcesResult {
                resources: default_mcp_resources()
                    .into_iter()
                    .map(|(uri, name, description)| {
                        RawResource::new(uri, name)
                            .with_description(description)
                            .with_mime_type("application/json")
                            .no_annotation()
                    })
                    .collect(),
                ..Default::default()
            };
            Ok(result)
        })
    }

    fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListResourceTemplatesResult, ErrorData>> + Send + '_ {
        std::future::ready({
            let result = ListResourceTemplatesResult {
                resource_templates: default_mcp_resource_templates()
                    .into_iter()
                    .map(|(uri_template, name, description)| {
                        RawResourceTemplate::new(uri_template, name)
                            .with_description(description)
                            .with_mime_type("application/json")
                            .no_annotation()
                    })
                    .collect(),
                ..Default::default()
            };
            Ok(result)
        })
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ReadResourceResult, ErrorData>> + Send + '_ {
        std::future::ready(self.read_resource_contents(&request.uri))
    }

    fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListPromptsResult, ErrorData>> + Send + '_ {
        std::future::ready({
            let result = ListPromptsResult {
                prompts: self.prompt_definitions(),
                ..Default::default()
            };
            Ok(result)
        })
    }

    fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<GetPromptResult, ErrorData>> + Send + '_ {
        std::future::ready(self.prompt_response(request))
    }
}

fn required_prompt_argument(name: &str, description: &str) -> PromptArgument {
    PromptArgument::new(name)
        .with_description(description)
        .with_required(true)
}

fn required_argument(request: &GetPromptRequestParams, name: &str) -> Result<String, ErrorData> {
    request
        .arguments
        .as_ref()
        .and_then(|arguments| arguments.get(name))
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("prompt argument `{name}` is required"), None)
        })
}

fn string_error(error: DaemonClientError) -> String {
    error.to_string()
}

fn map_daemon_error(error: DaemonClientError) -> ErrorData {
    ErrorData::internal_error(error.to_string(), None)
}

#[cfg(feature = "stdio-server")]
pub async fn serve_stdio() -> anyhow::Result<()> {
    let service = ArborMcp::new().serve(rmcp::transport::io::stdio()).await?;
    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        arbor_core::{
            daemon::{DaemonSessionRecord, TerminalSessionState, TerminalSnapshot},
            process::{ProcessInfo, ProcessStatus},
        },
    };

    #[derive(Default)]
    struct FakeDaemon;

    impl DaemonApi for FakeDaemon {
        fn health(&self) -> Result<HealthResponse, DaemonClientError> {
            Ok(HealthResponse {
                status: "ok".to_owned(),
                version: "test".to_owned(),
            })
        }

        fn list_repositories(&self) -> Result<Vec<RepositoryDto>, DaemonClientError> {
            Ok(vec![RepositoryDto {
                root: "/tmp/repo".to_owned(),
                label: "repo".to_owned(),
                github_repo_slug: None,
                avatar_url: None,
            }])
        }

        fn list_worktrees(
            &self,
            _repo_root: Option<&str>,
        ) -> Result<Vec<WorktreeDto>, DaemonClientError> {
            Ok(vec![WorktreeDto {
                repo_root: "/tmp/repo".to_owned(),
                path: "/tmp/repo".to_owned(),
                branch: "main".to_owned(),
                is_primary_checkout: true,
                last_activity_unix_ms: None,
                diff_additions: None,
                diff_deletions: None,
                pr_number: None,
                pr_url: None,
            }])
        }

        fn create_worktree(
            &self,
            request: &CreateWorktreeRequest,
        ) -> Result<WorktreeMutationResponse, DaemonClientError> {
            Ok(WorktreeMutationResponse {
                repo_root: request.repo_root.clone(),
                path: request.path.clone(),
                branch: request.branch.clone(),
                deleted_branch: None,
                message: "created".to_owned(),
            })
        }

        fn delete_worktree(
            &self,
            request: &DeleteWorktreeRequest,
        ) -> Result<WorktreeMutationResponse, DaemonClientError> {
            Ok(WorktreeMutationResponse {
                repo_root: request.repo_root.clone(),
                path: request.path.clone(),
                branch: Some("feature".to_owned()),
                deleted_branch: Some("feature".to_owned()),
                message: "deleted".to_owned(),
            })
        }

        fn list_changed_files(
            &self,
            _path: &str,
        ) -> Result<Vec<ChangedFileDto>, DaemonClientError> {
            Ok(vec![ChangedFileDto {
                path: "src/main.rs".to_owned(),
                kind: "modified".to_owned(),
                additions: 3,
                deletions: 1,
            }])
        }

        fn commit_worktree(
            &self,
            request: &CommitWorktreeRequest,
        ) -> Result<GitActionResponse, DaemonClientError> {
            Ok(GitActionResponse {
                path: request.path.clone(),
                branch: Some("main".to_owned()),
                message: "commit complete".to_owned(),
                commit_message: request
                    .message
                    .clone()
                    .or_else(|| Some("generated".to_owned())),
            })
        }

        fn push_worktree(
            &self,
            request: &PushWorktreeRequest,
        ) -> Result<GitActionResponse, DaemonClientError> {
            Ok(GitActionResponse {
                path: request.path.clone(),
                branch: Some("main".to_owned()),
                message: "push complete".to_owned(),
                commit_message: None,
            })
        }

        fn list_terminals(&self) -> Result<Vec<DaemonSessionRecord>, DaemonClientError> {
            Ok(vec![DaemonSessionRecord {
                session_id: "daemon-1".to_owned(),
                workspace_id: "/tmp/repo".to_owned(),
                cwd: "/tmp/repo".into(),
                shell: "/bin/zsh".to_owned(),
                cols: 120,
                rows: 35,
                title: Some("shell".to_owned()),
                last_command: None,
                output_tail: None,
                exit_code: None,
                state: Some(TerminalSessionState::Running),
                updated_at_unix_ms: None,
            }])
        }

        fn create_terminal(
            &self,
            request: &CreateTerminalRequest,
        ) -> Result<CreateTerminalResponse, DaemonClientError> {
            Ok(CreateTerminalResponse {
                is_new_session: true,
                session: DaemonSessionRecord {
                    session_id: request
                        .session_id
                        .clone()
                        .unwrap_or_else(|| "daemon-1".to_owned()),
                    workspace_id: request
                        .workspace_id
                        .clone()
                        .unwrap_or_else(|| request.cwd.clone()),
                    cwd: request.cwd.clone().into(),
                    shell: request
                        .shell
                        .clone()
                        .unwrap_or_else(|| "/bin/zsh".to_owned()),
                    cols: request.cols.unwrap_or(120),
                    rows: request.rows.unwrap_or(35),
                    title: request.title.clone(),
                    last_command: None,
                    output_tail: None,
                    exit_code: None,
                    state: Some(TerminalSessionState::Running),
                    updated_at_unix_ms: None,
                },
            })
        }

        fn read_terminal_output(
            &self,
            session_id: &str,
            _max_lines: Option<usize>,
        ) -> Result<TerminalSnapshot, DaemonClientError> {
            Ok(TerminalSnapshot {
                session_id: session_id.to_owned(),
                output_tail: "ok".to_owned(),
                styled_lines: vec![],
                cursor: None,
                modes: Default::default(),
                exit_code: None,
                state: TerminalSessionState::Running,
                updated_at_unix_ms: None,
            })
        }

        fn write_terminal_input(
            &self,
            _session_id: &str,
            _data: &[u8],
        ) -> Result<(), DaemonClientError> {
            Ok(())
        }

        fn resize_terminal(
            &self,
            _session_id: &str,
            _request: &TerminalResizeRequest,
        ) -> Result<(), DaemonClientError> {
            Ok(())
        }

        fn signal_terminal(
            &self,
            _session_id: &str,
            _request: &TerminalSignalRequest,
        ) -> Result<(), DaemonClientError> {
            Ok(())
        }

        fn detach_terminal(&self, _session_id: &str) -> Result<(), DaemonClientError> {
            Ok(())
        }

        fn kill_terminal(&self, _session_id: &str) -> Result<(), DaemonClientError> {
            Ok(())
        }

        fn list_agent_activity(&self) -> Result<Vec<AgentSessionDto>, DaemonClientError> {
            Ok(vec![AgentSessionDto {
                cwd: "/tmp/repo".to_owned(),
                state: "working".to_owned(),
                updated_at_unix_ms: 1,
            }])
        }

        fn list_processes(&self) -> Result<Vec<ProcessInfo>, DaemonClientError> {
            Ok(vec![ProcessInfo {
                name: "web".to_owned(),
                command: "cargo run".to_owned(),
                status: ProcessStatus::Running,
                exit_code: None,
                restart_count: 0,
                session_id: Some("process-web".to_owned()),
            }])
        }

        fn start_all_processes(&self) -> Result<Vec<ProcessInfo>, DaemonClientError> {
            self.list_processes()
        }

        fn stop_all_processes(&self) -> Result<Vec<ProcessInfo>, DaemonClientError> {
            self.list_processes()
        }

        fn start_process(&self, _name: &str) -> Result<ProcessInfo, DaemonClientError> {
            Ok(self
                .list_processes()?
                .into_iter()
                .next()
                .unwrap_or(ProcessInfo {
                    name: "web".to_owned(),
                    command: "cargo run".to_owned(),
                    status: ProcessStatus::Running,
                    exit_code: None,
                    restart_count: 0,
                    session_id: Some("process-web".to_owned()),
                }))
        }

        fn stop_process(&self, _name: &str) -> Result<ProcessInfo, DaemonClientError> {
            self.start_process(_name)
        }

        fn restart_process(&self, _name: &str) -> Result<ProcessInfo, DaemonClientError> {
            self.start_process(_name)
        }
    }

    #[test]
    fn advertises_tools_prompts_and_resources() {
        let server = ArborMcp::with_client(Arc::new(FakeDaemon));
        let info = server.get_info();
        assert!(info.capabilities.tools.is_some());
        assert!(info.capabilities.resources.is_some());
        assert!(info.capabilities.prompts.is_some());
    }

    #[test]
    fn prompt_catalog_is_populated() {
        let server = ArborMcp::with_client(Arc::new(FakeDaemon));
        let prompts = server.prompt_definitions();
        assert_eq!(prompts.len(), 3);
        assert!(
            prompts
                .iter()
                .any(|prompt| prompt.name == "review-worktree")
        );
    }

    #[test]
    fn reads_health_resource() {
        let server = ArborMcp::with_client(Arc::new(FakeDaemon));
        let result = server
            .read_resource_contents("arbor://health")
            .unwrap_or_else(|e| panic!("health resource should be readable: {e:?}"));
        assert_eq!(result.contents.len(), 1);
    }

    #[test]
    fn tool_catalog_contains_structured_tools() {
        let server = ArborMcp::with_client(Arc::new(FakeDaemon));
        let tools = server.tool_router.list_all();
        assert!(tools.iter().any(|tool| tool.name == "list_repositories"));
        assert!(
            tools
                .iter()
                .find(|tool| tool.name == "list_repositories")
                .and_then(|tool| tool.output_schema.as_ref())
                .is_some()
        );
    }
}
