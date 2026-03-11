use {
    crate::ArborMcp,
    rmcp::{
        ErrorData,
        model::{
            GetPromptRequestParams, GetPromptResult, Prompt, PromptArgument, PromptMessage,
            PromptMessageRole,
        },
    },
};

impl ArborMcp {
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

pub(crate) fn required_prompt_argument(name: &str, description: &str) -> PromptArgument {
    PromptArgument::new(name)
        .with_description(description)
        .with_required(true)
}

pub(crate) fn required_argument(
    request: &GetPromptRequestParams,
    name: &str,
) -> Result<String, ErrorData> {
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
