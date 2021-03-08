use ureq::Error;
use std::fmt;

use serde::{Deserialize};

#[derive(Deserialize)]
struct GitlabVersion {
    version: String,
    revision: String,
}

impl fmt::Display for GitlabVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ version: \"{}\", revision: \"{}\" }}",
            self.version, self.revision,
        )
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
        info!("Calling api: {}", self.api);

        let version: GitlabVersion = ureq::get(&self.api)
            .set("PRIVATE-TOKEN", &self.gitlab_token)
            .call()?
            .into_json()?;

        Ok(version)
    }

    /// Post a message to Rocket Chat.
    fn publish_message(self: &Self, message: &String) -> Result<(), Error> {
        info!("Publishing message: {}", message);

        let request_url = format!(
            "https://discuss.kaizen-hosting.com/hooks/{token}",
            token = self.rocket_token
        );

        let _response = ureq::post(&request_url)
            .send_json(ureq::json!({
                "text": message
            }))?
            .into_string()?;

        Ok(())
    }

    /// The bot process.
    pub fn exec(self: &Self) -> Result<(), Error> {
        info!("Using api url: {}", self.api);
        info!("Using api token: {}", self.gitlab_token);
        info!("Using rocket token: {}", self.rocket_token);

        let version = self.get_version()?;
        info!("Found version: {}", version);

        // if version.len() != 0 {
            // message = format!("Aujourd'hui, {api_message}", api_message = message);
            // self.publish_message(&message)?;
        // }

        Ok(())
    }
}
