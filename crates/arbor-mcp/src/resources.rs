use {
    crate::{ArborMcp, map_daemon_error},
    arbor_daemon_client::{
        default_mcp_resource_templates, default_mcp_resources, parse_terminal_snapshot_resource,
        parse_worktree_changes_resource, read_json_text_resource,
    },
    rmcp::{
        ErrorData,
        model::{
            AnnotateAble, ListResourceTemplatesResult, ListResourcesResult, PaginatedRequestParams,
            RawResource, RawResourceTemplate, ReadResourceResult, ResourceContents,
        },
    },
};

impl ArborMcp {
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
            "arbor://tasks" => {
                read_json_text_resource(&self.daemon.list_tasks().map_err(map_daemon_error)?)
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

    pub(crate) fn list_resources_result(
        &self,
        _request: Option<PaginatedRequestParams>,
    ) -> Result<ListResourcesResult, ErrorData> {
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
    }

    pub(crate) fn list_resource_templates_result(
        &self,
        _request: Option<PaginatedRequestParams>,
    ) -> Result<ListResourceTemplatesResult, ErrorData> {
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
    }
}
