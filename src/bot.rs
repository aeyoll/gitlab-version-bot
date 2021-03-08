use failure::Error;

use crate::gitlab_tag::GitlabTag;
use crate::gitlab_version::GitlabVersion;

#[derive(Debug)]
/// Represent the url of the gitlab api and the Rocket Chat webhook token.
pub struct Bot {
    /// Gitlab instance url.
    pub gitlab_url: String,

    /// Gitlab API token.
    pub gitlab_token: String,

    /// Rocket chat instance url
    pub rocket_chat_url: String,

    /// Rocket Chat webhook token.
    pub rocket_chat_token: String,
}

impl Bot {
    /// Fetch the gitlab version from the gitlab API.
    fn get_version(&self) -> Result<GitlabVersion, Error> {
        let url = format!("{}/api/v4/version", &self.gitlab_url);
        debug!("Calling api: {}", url);

        let version: GitlabVersion = ureq::get(&url)
            .set("PRIVATE-TOKEN", &self.gitlab_token)
            .call()?
            .into_json()?;

        Ok(version)
    }

    /// Post a message to Rocket Chat.
    fn publish_message(&self, message: &str) -> Result<(), Error> {
        debug!("Publishing message: {}", message);

        let request_url = format!(
            "{url}/hooks/{token}",
            url = self.rocket_chat_url,
            token = self.rocket_chat_token
        );

        let _response = ureq::post(&request_url)
            .send_json(ureq::json!({ "text": message }))?
            .into_string()?;

        Ok(())
    }

    /// The bot process.
    pub fn exec(&self) -> Result<(), Error> {
        debug!("Using api url: {}", self.gitlab_url);
        debug!("Using api token: {}", self.gitlab_token);
        debug!("Using rocket chat url: {}", self.rocket_chat_token);
        debug!("Using rocket chat token: {}", self.rocket_chat_token);

        let gitlab_latest_version = GitlabTag::get_latest_version()?;
        info!("Latest Gitlab version is {}", gitlab_latest_version);

        let version = self.get_version()?;
        info!("Current Gitlab version is {}", version);

        if gitlab_latest_version != version {
            let message = format!(
                "The version of Gitlab ({}) is no longer up to date, {} is now available.",
                version, gitlab_latest_version
            );
            self.publish_message(&message)?;
        }

        Ok(())
    }
}
