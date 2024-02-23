use std::fmt::Display;

use anyhow::{bail, Result};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client, ClientBuilder,
};
use serde::{Deserialize, Serialize};

fn url(path: impl Display) -> String {
    format!("https://api.timeular.com/api/v3{path}")
}

pub struct TimeularClient {
    client: Client,
}

impl TimeularClient {
    pub async fn sign_in(request: &SignInRequest) -> Result<Self> {
        let client = Client::new();
        let response = client
            .post(url("/developer/sign-in"))
            .json(&request)
            .send()
            .await?
            .json::<SignInResponse>()
            .await?;

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {}", response.token).parse()?);
        let client = ClientBuilder::new().default_headers(headers).build()?;
        Ok(Self { client })
    }

    pub async fn list_activities(&self) -> Result<ActivitiesResponse> {
        let response = self
            .client
            .get(url("/activities"))
            .send()
            .await?
            .json::<ActivitiesResponse>()
            .await?;
        Ok(response)
    }

    pub async fn create_time_entry(&self, request: &CreateTimeEntryRequest) -> Result<()> {
        let response = self
            .client
            .post(url("/time-entries"))
            .json(&request)
            .send()
            .await?
            .json::<CreateTimeEntryResponse>()
            .await?;
        match response.message {
            Some(message) => bail!("{message}"),
            None => Ok(()),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignInRequest {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Deserialize, Debug)]
pub struct SignInResponse {
    pub token: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActivitiesResponse {
    pub activities: Vec<Activity>,
    pub inactive_activities: Vec<Activity>,
    pub archived_activities: Vec<Activity>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: String,
    pub name: String,
    pub color: String,
    pub integration: String,
    pub space_id: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTimeEntryRequest {
    pub activity_id: String,
    pub started_at: String,
    pub stopped_at: String,
    pub note: Note,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateTimeEntryResponse {
    pub message: Option<String>,
}
