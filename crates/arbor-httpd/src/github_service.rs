use {
    futures_util::future::BoxFuture,
    std::{
        process::{Command, Stdio},
        sync::{Arc, OnceLock},
    },
};

pub trait GitHubPrService: Send + Sync {
    fn lookup_pr_for_branch(
        &self,
        repo_slug: Option<String>,
        branch: String,
        is_primary: bool,
    ) -> BoxFuture<'static, (Option<u64>, Option<String>)>;
}

pub struct OctocrabGitHubPrService;

impl GitHubPrService for OctocrabGitHubPrService {
    fn lookup_pr_for_branch(
        &self,
        repo_slug: Option<String>,
        branch: String,
        is_primary: bool,
    ) -> BoxFuture<'static, (Option<u64>, Option<String>)> {
        Box::pin(async move {
            let Some(slug) = repo_slug else {
                return (None, None);
            };

            if is_primary || branch == "-" || branch.is_empty() {
                return (None, None);
            }

            let lower = branch.to_ascii_lowercase();
            if matches!(
                lower.as_str(),
                "main" | "master" | "develop" | "dev" | "trunk"
            ) {
                return (None, None);
            }

            let Some((owner, repo_name)) = slug.split_once('/') else {
                return (None, None);
            };

            let Some(client) = shared_github_client() else {
                return (None, None);
            };

            let owner = owner.to_owned();
            let repo_name = repo_name.to_owned();

            let page = client
                .pulls(&owner, &repo_name)
                .list()
                .head(format!("{owner}:{branch}"))
                .state(octocrab::params::State::All)
                .per_page(1)
                .send()
                .await;

            let Ok(page) = page else {
                return (None, None);
            };

            match page.items.first() {
                Some(pr) => {
                    let number = pr.number;
                    let url = format!("https://github.com/{owner}/{repo_name}/pull/{number}");
                    (Some(number), Some(url))
                },
                None => (None, None),
            }
        })
    }
}

pub fn default_github_pr_service() -> Arc<dyn GitHubPrService> {
    Arc::new(OctocrabGitHubPrService)
}

/// Reuse a single Octocrab client to avoid creating a new reqwest connection
/// pool (and spawning `gh auth token`) on every PR lookup.
fn shared_github_client() -> Option<&'static octocrab::Octocrab> {
    static CLIENT: OnceLock<Option<octocrab::Octocrab>> = OnceLock::new();
    CLIENT
        .get_or_init(|| {
            let token = resolve_github_token()?;
            octocrab::Octocrab::builder()
                .personal_token(token)
                .build()
                .ok()
        })
        .as_ref()
}

fn resolve_github_token() -> Option<String> {
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        let trimmed = token.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_owned());
        }
    }

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
