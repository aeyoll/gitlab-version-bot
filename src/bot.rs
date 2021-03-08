use failure::Error;
use std::fmt;

use serde::Deserialize;

#[derive(Deserialize)]
struct GitlabVersion {
    version: String,
}

impl GitlabVersion {
    pub fn new(version: String) -> Self {
        GitlabVersion { version }
    }
}

impl fmt::Display for GitlabVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.version,)
    }
}

#[derive(Deserialize)]
struct GitlabTag {
    name: String,
}

impl GitlabTag {
    pub fn get_lastest_version() -> Result<GitlabVersion, Error> {
        let url = "https://gitlab.com/api/v4/projects/13083/repository/tags";

        let mut tags: Vec<GitlabTag> = ureq::get(&url).call()?.into_json()?;

        tags.sort_by(|a, b| b.name.cmp(&a.name));

        let mut stable_tags = tags
            .into_iter()
            .filter(|tag| tag.name.contains("rc") == false)
            .map(|tag| GitlabVersion::new(tag.name.replace("v", "")));

        let latest_version = stable_tags.nth(0).unwrap();

        Ok(latest_version)
    }
}

impl fmt::Display for GitlabTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ name: \"{}\" }}", self.name,)
    }
}

#[derive(Debug)]
/// Represent the url of the gitlab api and the Rocket Chat webhook token.
pub struct Bot {
    /// url of the gitlab API.
    pub api: String,

    /// Gitlab API token.
    pub gitlab_token: String,

    /// Rocket Chat webhook token.
    pub rocket_token: String,
}

impl Bot {
    /// Fetch the gitlab version from the gitlab API.
    fn get_version(self: &Self) -> Result<GitlabVersion, Error> {
        let url = format!("{}/api/v4/version", &self.api);
        debug!("Calling api: {}", url);

        let version: GitlabVersion = ureq::get(&url)
            .set("PRIVATE-TOKEN", &self.gitlab_token)
            .call()?
            .into_json()?;

        Ok(version)
    }

    /// Post a message to Rocket Chat.
    fn publish_message(self: &Self, message: &String) -> Result<(), Error> {
        debug!("Publishing message: {}", message);

        let request_url = format!(
            "https://discuss.kaizen-hosting.com/hooks/{token}",
            token = self.rocket_token
        );

        let _response = ureq::post(&request_url)
            .send_json(ureq::json!({ "text": message }))?
            .into_string()?;

        Ok(())
    }

    /// The bot process.
    pub fn exec(self: &Self) -> Result<(), Error> {
        debug!("Using api url: {}", self.api);
        debug!("Using api token: {}", self.gitlab_token);
        debug!("Using rocket token: {}", self.rocket_token);

        let gitlab_latest_version = GitlabTag::get_lastest_version()?;
        info!("Latest Gitlab version is {}", gitlab_latest_version);

        let version = self.get_version()?;
        info!("Current Gitlab version is {}", version);

        // if version.len() != 0 {
        // message = format!("Aujourd'hui, {api_message}", api_message = message);
        // self.publish_message(&message)?;
        // }

        Ok(())
    }
}
