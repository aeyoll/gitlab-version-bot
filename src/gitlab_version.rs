use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
/// Represent a gitlab version fetched a gitlab instance API.
pub struct GitlabVersion {
    /// Version tag
    version: String,
}

impl GitlabVersion {
    /// Returns a GitlabVersion given a version
    ///
    /// # Arguments
    ///
    /// * `version` - A string that holds a version  tag
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gitlab_version::GitlabVersion;
    /// let gitlab_version = GitlabVersion::new(String::from("13.9.3"));
    /// ```
    pub fn new(version: String) -> Self {
        GitlabVersion { version }
    }
}

impl fmt::Display for GitlabVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.version,)
    }
}

impl std::cmp::PartialEq for GitlabVersion {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}
