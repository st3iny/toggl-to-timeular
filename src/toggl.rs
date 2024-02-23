use anyhow::{bail, Result};
use chrono::{DateTime, NaiveDateTime, TimeZone};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TogglTimeEntry {
    #[serde(rename = "User")]
    pub user: String,

    #[serde(rename = "Email")]
    pub email: String,

    #[serde(rename = "Client")]
    pub client: String,

    #[serde(rename = "Project")]
    pub project: String,

    #[serde(rename = "Task")]
    pub task: String,

    #[serde(rename = "Description")]
    pub description: String,

    #[serde(rename = "Billable")]
    pub billable: String,

    #[serde(rename = "Start date")]
    pub start_date: String,

    #[serde(rename = "Start time")]
    pub start_time: String,

    #[serde(rename = "End date")]
    pub end_date: String,

    #[serde(rename = "End time")]
    pub end_time: String,

    #[serde(rename = "Duration")]
    pub duration: String,

    #[serde(rename = "Tags")]
    pub tags: String,

    #[serde(rename = "Amount ()")]
    pub amount: String,
}

impl TogglTimeEntry {
    pub fn start<Tz: TimeZone>(&self, time_zone: Tz) -> Result<DateTime<Tz>> {
        parse_date_time(&self.start_date, &self.start_time, time_zone)
    }

    pub fn end<Tz: TimeZone>(&self, time_zone: Tz) -> Result<DateTime<Tz>> {
        parse_date_time(&self.end_date, &self.end_time, time_zone)
    }
}

fn parse_date_time<Tz: TimeZone>(date: &str, time: &str, time_zone: Tz) -> Result<DateTime<Tz>> {
    let raw_date_time = format!("{date} {time}");
    let Some(date_time) = NaiveDateTime::parse_from_str(&raw_date_time, "%Y-%m-%d %H:%M:%S")?
        .and_local_timezone(time_zone.clone())
        .single()
    else {
        bail!("Failed to parse date time: {raw_date_time}")
    };
    Ok(date_time)
}
