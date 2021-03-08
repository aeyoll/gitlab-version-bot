use failure::Error;
use serde::Deserialize;
use std::fmt;

use crate::gitlab_version::GitlabVersion;

#[derive(Deserialize)]
/// Represent a git tag fetched from gitlab official API.
pub struct GitlabTag {
    name: String,
}

impl GitlabTag {
    /// Fetch the latest stable version from gitlab.
    /// Example:
    ///
    /// ```
    /// use crate::gitlab_tag::GitlabTag;
    /// let gitlab_latest_version = GitlabTag::get_latest_version()?;
    /// info!("Latest Gitlab version is {}", gitlab_latest_version);
    /// ```
    pub fn get_latest_version() -> Result<GitlabVersion, Error> {
        let url = "https://gitlab.com/api/v4/projects/13083/repository/tags";

        let mut tags: Vec<GitlabTag> = ureq::get(&url).call()?.into_json()?;

        tags.sort_by(|a, b| b.name.cmp(&a.name));

        let mut stable_tags = tags
            .into_iter()
            .filter(|tag| !tag.name.contains("rc"))
            .map(|tag| GitlabVersion::new(tag.name.replace("v", "")));

        let latest_version = stable_tags.next().unwrap();

        Ok(latest_version)
    }
}

impl fmt::Display for GitlabTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ name: \"{}\" }}", self.name,)
    }
}
