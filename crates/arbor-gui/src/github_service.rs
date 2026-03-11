use {
    serde::Deserialize,
    std::{
        process::{Command, Stdio},
        sync::Arc,
    },
};

// ---------------------------------------------------------------------------
// Review comment types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffSide {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct ReviewComment {
    pub id: u64,
    pub author: String,
    pub body: String,
    pub created_at: String,
    #[allow(dead_code)]
    pub outdated: bool,
    #[allow(dead_code)]
    pub path: String,
    #[allow(dead_code)]
    pub line: Option<usize>,
    #[allow(dead_code)]
    pub start_line: Option<usize>,
    #[allow(dead_code)]
    pub side: DiffSide,
    #[allow(dead_code)]
    pub in_reply_to: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ReviewThread {
    pub id: String,
    pub path: String,
    pub line: Option<usize>,
    #[allow(dead_code)]
    pub start_line: Option<usize>,
    pub side: DiffSide,
    pub is_resolved: bool,
    #[allow(dead_code)]
    pub is_outdated: bool,
    pub comments: Vec<ReviewComment>,
}

// ---------------------------------------------------------------------------
// GitHubReviewService trait
// ---------------------------------------------------------------------------

#[allow(dead_code)]
pub trait GitHubReviewService: Send + Sync {
    /// Fetch all review threads for a PR.
    fn fetch_review_threads(
        &self,
        repo_slug: &str,
        pr_number: u64,
    ) -> Result<Vec<ReviewThread>, String>;

    /// Post a new inline comment on a PR.
    fn post_review_comment(
        &self,
        repo_slug: &str,
        pr_number: u64,
        path: &str,
        line: usize,
        side: DiffSide,
        body: &str,
        commit_sha: &str,
    ) -> Result<ReviewComment, String>;

    /// Reply to an existing comment thread.
    fn reply_to_thread(
        &self,
        repo_slug: &str,
        pr_number: u64,
        comment_id: u64,
        body: &str,
    ) -> Result<ReviewComment, String>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrState {
    Open,
    Draft,
    Merged,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewDecision {
    Approved,
    ChangesRequested,
    Pending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckStatus {
    Success,
    Failure,
    Pending,
}

#[derive(Debug, Clone)]
pub struct PrDetails {
    pub number: u64,
    pub title: String,
    pub url: String,
    pub state: PrState,
    pub base_ref_name: String,
    pub additions: usize,
    pub deletions: usize,
    pub review_decision: ReviewDecision,
    pub checks_status: CheckStatus,
    pub checks: Vec<(String, CheckStatus)>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GhPrResponse {
    number: u64,
    title: String,
    url: String,
    state: String,
    is_draft: bool,
    base_ref_name: String,
    additions: usize,
    deletions: usize,
    review_decision: Option<String>,
    #[serde(default)]
    status_check_rollup: Vec<GhCheckContext>,
}

#[derive(Deserialize)]
struct GhCheckContext {
    name: Option<String>,
    context: Option<String>,
    conclusion: Option<String>,
    status: Option<String>,
    state: Option<String>,
}

impl GhCheckContext {
    fn display_name(&self) -> String {
        self.name
            .as_deref()
            .or(self.context.as_deref())
            .unwrap_or("check")
            .to_owned()
    }

    fn to_check_status(&self) -> CheckStatus {
        if self
            .status
            .as_deref()
            .is_some_and(|status| !status.eq_ignore_ascii_case("completed"))
        {
            return CheckStatus::Pending;
        }

        if let Some(conclusion) = &self.conclusion {
            return match conclusion.as_str() {
                "" => CheckStatus::Pending,
                "SUCCESS" | "success" | "NEUTRAL" | "neutral" | "SKIPPED" | "skipped" => {
                    CheckStatus::Success
                },
                _ => CheckStatus::Failure,
            };
        }
        if let Some(state) = &self.state {
            return match state.as_str() {
                "SUCCESS" | "success" => CheckStatus::Success,
                "FAILURE" | "failure" | "ERROR" | "error" => CheckStatus::Failure,
                _ => CheckStatus::Pending,
            };
        }
        if let Some(status) = &self.status {
            return match status.as_str() {
                "COMPLETED" | "completed" => CheckStatus::Success,
                _ => CheckStatus::Pending,
            };
        }
        CheckStatus::Pending
    }
}

fn parse_pr_details(response: GhPrResponse) -> PrDetails {
    let state = if response.is_draft {
        PrState::Draft
    } else {
        match response.state.as_str() {
            "MERGED" | "merged" => PrState::Merged,
            "CLOSED" | "closed" => PrState::Closed,
            _ => PrState::Open,
        }
    };

    let review_decision = match response.review_decision.as_deref() {
        Some("APPROVED") => ReviewDecision::Approved,
        Some("CHANGES_REQUESTED") => ReviewDecision::ChangesRequested,
        _ => ReviewDecision::Pending,
    };

    let checks: Vec<(String, CheckStatus)> = response
        .status_check_rollup
        .iter()
        .map(|c| (c.display_name(), c.to_check_status()))
        .collect();

    let checks_status = if checks.is_empty() {
        CheckStatus::Pending
    } else if checks.iter().any(|(_, s)| *s == CheckStatus::Failure) {
        CheckStatus::Failure
    } else if checks.iter().all(|(_, s)| *s == CheckStatus::Success) {
        CheckStatus::Success
    } else {
        CheckStatus::Pending
    };

    PrDetails {
        number: response.number,
        title: response.title,
        url: response.url,
        state,
        base_ref_name: response.base_ref_name,
        additions: response.additions,
        deletions: response.deletions,
        review_decision,
        checks_status,
        checks,
    }
}

/// Fetch rich PR details using `gh pr view`. Returns `None` if `gh` is not
/// installed, the command fails, or no PR exists for the given branch.
pub fn pull_request_details(repo_slug: &str, branch: &str) -> Option<PrDetails> {
    let output = Command::new("gh")
        .args([
            "pr",
            "view",
            "--repo",
            repo_slug,
            "--json",
            "number,title,url,state,isDraft,baseRefName,additions,deletions,reviewDecision,statusCheckRollup",
            branch,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let response: GhPrResponse = serde_json::from_slice(&output.stdout).ok()?;
    Some(parse_pr_details(response))
}

pub trait GitHubService: Send + Sync {
    fn create_pull_request(
        &self,
        repo_slug: &str,
        title: &str,
        branch: &str,
        base_branch: &str,
        token: &str,
    ) -> Result<String, String>;

    fn pull_request_number(&self, repo_slug: &str, branch: &str, token: &str) -> Option<u64>;
}

pub struct OctocrabGitHubService;

impl GitHubService for OctocrabGitHubService {
    fn create_pull_request(
        &self,
        repo_slug: &str,
        title: &str,
        branch: &str,
        base_branch: &str,
        token: &str,
    ) -> Result<String, String> {
        let (owner, repo_name) = repo_slug
            .split_once('/')
            .ok_or_else(|| format!("invalid repository slug: {repo_slug}"))?;

        let owner = owner.to_owned();
        let repo_name = repo_name.to_owned();
        let title = title.to_owned();
        let branch = branch.to_owned();
        let base_branch = base_branch.to_owned();
        let token = token.to_owned();

        let runtime = tokio::runtime::Runtime::new()
            .map_err(|error| format!("failed to create runtime: {error}"))?;

        runtime.block_on(async move {
            let octocrab = octocrab::Octocrab::builder()
                .personal_token(token)
                .build()
                .map_err(|error| format!("failed to create GitHub client: {error}"))?;

            let pr = octocrab
                .pulls(&owner, &repo_name)
                .create(&title, &branch, &base_branch)
                .send()
                .await
                .map_err(|error| format!("failed to create pull request: {error}"))?;

            let url = pr.html_url.map(|u| u.to_string()).unwrap_or_default();
            Ok(format!("created PR: {url}"))
        })
    }

    fn pull_request_number(&self, repo_slug: &str, branch: &str, token: &str) -> Option<u64> {
        let (owner, repo_name) = repo_slug.split_once('/')?;
        let owner = owner.to_owned();
        let repo_name = repo_name.to_owned();
        let branch = branch.to_owned();
        let token = token.to_owned();

        let runtime = tokio::runtime::Runtime::new().ok()?;
        runtime.block_on(async move {
            let octocrab = octocrab::Octocrab::builder()
                .personal_token(token)
                .build()
                .ok()?;

            let page = octocrab
                .pulls(&owner, &repo_name)
                .list()
                .head(format!("{owner}:{branch}"))
                .state(octocrab::params::State::All)
                .per_page(1)
                .send()
                .await
                .ok()?;

            page.items.first().map(|pr| pr.number)
        })
    }
}

pub fn default_github_service() -> Arc<dyn GitHubService> {
    Arc::new(OctocrabGitHubService)
}

// ---------------------------------------------------------------------------
// GhCliReviewService — uses `gh api graphql` / `gh api` REST
// ---------------------------------------------------------------------------

pub struct GhCliReviewService;

impl GhCliReviewService {
    fn split_slug(repo_slug: &str) -> Result<(&str, &str), String> {
        repo_slug
            .split_once('/')
            .ok_or_else(|| format!("invalid repository slug: {repo_slug}"))
    }
}

// Serde types for GraphQL response -----------------------------------------

#[derive(Deserialize)]
struct GqlResponse {
    data: Option<GqlData>,
    errors: Option<Vec<GqlError>>,
}

#[derive(Deserialize)]
struct GqlError {
    message: String,
}

#[derive(Deserialize)]
struct GqlData {
    repository: Option<GqlRepository>,
}

#[derive(Deserialize)]
struct GqlRepository {
    #[serde(rename = "pullRequest")]
    pull_request: Option<GqlPullRequest>,
}

#[derive(Deserialize)]
struct GqlPullRequest {
    #[serde(rename = "reviewThreads")]
    review_threads: GqlReviewThreadConnection,
}

#[derive(Deserialize)]
struct GqlReviewThreadConnection {
    nodes: Vec<GqlReviewThread>,
}

#[derive(Deserialize)]
struct GqlReviewThread {
    id: String,
    #[serde(rename = "isResolved")]
    is_resolved: bool,
    #[serde(rename = "isOutdated")]
    is_outdated: bool,
    path: String,
    line: Option<usize>,
    #[serde(rename = "startLine")]
    start_line: Option<usize>,
    #[serde(rename = "diffSide")]
    diff_side: Option<String>,
    comments: GqlCommentConnection,
}

#[derive(Deserialize)]
struct GqlCommentConnection {
    nodes: Vec<GqlComment>,
}

#[derive(Deserialize)]
struct GqlComment {
    #[serde(rename = "databaseId")]
    database_id: Option<u64>,
    body: String,
    author: Option<GqlAuthor>,
    #[serde(rename = "createdAt")]
    created_at: String,
    outdated: bool,
    path: String,
    line: Option<usize>,
    #[serde(rename = "startLine")]
    start_line: Option<usize>,
}

#[derive(Deserialize)]
struct GqlAuthor {
    login: String,
}

// REST response for posting a comment ---------------------------------------

#[allow(dead_code)]
#[derive(Deserialize)]
struct RestCommentResponse {
    id: u64,
    body: String,
    path: String,
    line: Option<usize>,
    #[serde(rename = "start_line")]
    start_line: Option<usize>,
    side: Option<String>,
    user: Option<RestUser>,
    created_at: String,
    #[serde(rename = "in_reply_to_id")]
    in_reply_to_id: Option<u64>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct RestUser {
    login: String,
}

fn parse_diff_side(raw: Option<&str>) -> DiffSide {
    match raw {
        Some("LEFT") => DiffSide::Left,
        _ => DiffSide::Right,
    }
}

impl GitHubReviewService for GhCliReviewService {
    fn fetch_review_threads(
        &self,
        repo_slug: &str,
        pr_number: u64,
    ) -> Result<Vec<ReviewThread>, String> {
        let (owner, repo) = Self::split_slug(repo_slug)?;

        let query = r#"
query($owner: String!, $repo: String!, $pr: Int!) {
  repository(owner: $owner, name: $repo) {
    pullRequest(number: $pr) {
      reviewThreads(first: 100) {
        nodes {
          id
          isResolved
          isOutdated
          path
          line
          startLine
          diffSide
          comments(first: 50) {
            nodes {
              databaseId
              body
              author { login }
              createdAt
              outdated
              path
              line
              startLine
            }
          }
        }
      }
    }
  }
}
"#;

        let variables = format!(
            r#"{{"owner":"{}","repo":"{}","pr":{}}}"#,
            owner, repo, pr_number
        );

        let output = Command::new("gh")
            .args([
                "api",
                "graphql",
                "-f",
                &format!("query={query}"),
                "-f",
                &format!("variables={variables}"),
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("failed to run `gh api graphql`: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("gh api graphql failed: {stderr}"));
        }

        let response: GqlResponse = serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("failed to parse GraphQL response: {e}"))?;

        if let Some(errors) = response.errors {
            let msgs: Vec<String> = errors.into_iter().map(|e| e.message).collect();
            return Err(format!("GraphQL errors: {}", msgs.join(", ")));
        }

        let threads = response
            .data
            .and_then(|d| d.repository)
            .and_then(|r| r.pull_request)
            .map(|pr| pr.review_threads.nodes)
            .unwrap_or_default();

        Ok(threads
            .into_iter()
            .map(|thread| {
                let side = parse_diff_side(thread.diff_side.as_deref());
                let first_comment_id = thread
                    .comments
                    .nodes
                    .first()
                    .and_then(|first| first.database_id);
                let comments = thread
                    .comments
                    .nodes
                    .into_iter()
                    .enumerate()
                    .map(|(i, c)| ReviewComment {
                        id: c.database_id.unwrap_or(0),
                        author: c
                            .author
                            .map(|a| a.login)
                            .unwrap_or_else(|| "ghost".to_owned()),
                        body: c.body,
                        created_at: c.created_at,
                        outdated: c.outdated,
                        path: c.path,
                        line: c.line,
                        start_line: c.start_line,
                        side,
                        in_reply_to: if i > 0 {
                            first_comment_id
                        } else {
                            None
                        },
                    })
                    .collect::<Vec<_>>();

                ReviewThread {
                    id: thread.id,
                    path: thread.path,
                    line: thread.line,
                    start_line: thread.start_line,
                    side,
                    is_resolved: thread.is_resolved,
                    is_outdated: thread.is_outdated,
                    comments,
                }
            })
            .collect())
    }

    fn post_review_comment(
        &self,
        repo_slug: &str,
        pr_number: u64,
        path: &str,
        line: usize,
        side: DiffSide,
        body: &str,
        commit_sha: &str,
    ) -> Result<ReviewComment, String> {
        let side_str = match side {
            DiffSide::Left => "LEFT",
            DiffSide::Right => "RIGHT",
        };

        let endpoint = format!("repos/{repo_slug}/pulls/{pr_number}/comments");

        let output = Command::new("gh")
            .args([
                "api",
                &endpoint,
                "-f",
                &format!("body={body}"),
                "-f",
                &format!("commit_id={commit_sha}"),
                "-f",
                &format!("path={path}"),
                "-F",
                &format!("line={line}"),
                "-f",
                &format!("side={side_str}"),
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("failed to run `gh api`: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("gh api POST failed: {stderr}"));
        }

        let resp: RestCommentResponse = serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("failed to parse comment response: {e}"))?;

        Ok(ReviewComment {
            id: resp.id,
            author: resp
                .user
                .map(|u| u.login)
                .unwrap_or_else(|| "ghost".to_owned()),
            body: resp.body,
            created_at: resp.created_at,
            outdated: false,
            path: resp.path,
            line: resp.line,
            start_line: resp.start_line,
            side: parse_diff_side(resp.side.as_deref()),
            in_reply_to: resp.in_reply_to_id,
        })
    }

    fn reply_to_thread(
        &self,
        repo_slug: &str,
        pr_number: u64,
        comment_id: u64,
        body: &str,
    ) -> Result<ReviewComment, String> {
        let endpoint = format!("repos/{repo_slug}/pulls/{pr_number}/comments/{comment_id}/replies");

        let output = Command::new("gh")
            .args(["api", &endpoint, "-f", &format!("body={body}")])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("failed to run `gh api`: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("gh api POST reply failed: {stderr}"));
        }

        let resp: RestCommentResponse = serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("failed to parse reply response: {e}"))?;

        Ok(ReviewComment {
            id: resp.id,
            author: resp
                .user
                .map(|u| u.login)
                .unwrap_or_else(|| "ghost".to_owned()),
            body: resp.body,
            created_at: resp.created_at,
            outdated: false,
            path: resp.path,
            line: resp.line,
            start_line: resp.start_line,
            side: parse_diff_side(resp.side.as_deref()),
            in_reply_to: resp.in_reply_to_id,
        })
    }
}

pub fn default_review_service() -> Arc<dyn GitHubReviewService> {
    Arc::new(GhCliReviewService)
}

pub fn github_access_token_from_gh_cli() -> Option<String> {
    let output = Command::new("gh")
        .args(["auth", "token"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let token = stdout.trim();
    (!token.is_empty()).then_some(token.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{CheckStatus, GhCheckContext, GhPrResponse, parse_pr_details};

    #[test]
    fn in_progress_checks_remain_pending_without_conclusion() {
        let details = parse_pr_details(GhPrResponse {
            number: 40,
            title: "Fix native toolbar buttons and linked worktree grouping".to_owned(),
            url: "https://github.com/penso/arbor/pull/40".to_owned(),
            state: "OPEN".to_owned(),
            is_draft: false,
            base_ref_name: "main".to_owned(),
            additions: 10,
            deletions: 2,
            review_decision: None,
            status_check_rollup: vec![
                GhCheckContext {
                    name: Some("Clippy".to_owned()),
                    context: None,
                    conclusion: Some(String::new()),
                    status: Some("IN_PROGRESS".to_owned()),
                    state: None,
                },
                GhCheckContext {
                    name: Some("Test".to_owned()),
                    context: None,
                    conclusion: Some(String::new()),
                    status: Some("IN_PROGRESS".to_owned()),
                    state: None,
                },
            ],
        });

        assert_eq!(details.checks_status, CheckStatus::Pending);
        assert_eq!(details.checks, vec![
            ("Clippy".to_owned(), CheckStatus::Pending),
            ("Test".to_owned(), CheckStatus::Pending),
        ]);
    }
}
