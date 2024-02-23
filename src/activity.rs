use anyhow::Result;
use serde::Deserialize;
use std::{fs::File, path::Path};

use crate::toggl::TogglTimeEntry;

#[derive(Deserialize, Debug)]
pub struct ActivityMapping {
    project: String,
    description: Option<String>,
    activity_id: String,
}

impl ActivityMapping {
    pub fn parse(path: impl AsRef<Path>) -> Result<Vec<Self>> {
        let file = File::open(path)?;
        let definitions: Vec<ActivityMapping> = serde_json::from_reader(file)?;
        Ok(definitions)
    }

    /// Map a Toggl time entry to a Timeular activity id
    pub fn map(&self, entry: &TogglTimeEntry) -> Option<String> {
        if entry.project != self.project {
            return None;
        }

        match self.description.as_ref() {
            Some(description) if description != &entry.description => None,
            _ => Some(self.activity_id.clone()),
        }
    }
}
