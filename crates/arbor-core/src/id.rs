use {
    schemars::JsonSchema,
    serde::{Deserialize, Serialize},
    std::fmt,
};

/// Unique identifier for a terminal daemon session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default, JsonSchema)]
#[serde(transparent)]
pub struct SessionId(String);

impl SessionId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for SessionId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for SessionId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

/// Unique identifier for a workspace.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default, JsonSchema)]
#[serde(transparent)]
pub struct WorkspaceId(String);

impl WorkspaceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for WorkspaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for WorkspaceId {
    fn from(s: String) -> Self {
        Self(s.to_owned())
    }
}

impl From<&str> for WorkspaceId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}
