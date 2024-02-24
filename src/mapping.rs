use anyhow::Result;
use serde::Deserialize;
use std::{fs::File, path::Path};

use crate::toggl::TogglTimeEntry;

pub struct ActivityMapper {
    mappings: Vec<ActivityMapping>,
}

impl ActivityMapper {
    pub fn parse(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path)?;
        let mappings: Vec<ActivityMapping> = serde_json::from_reader(file)?;
        Ok(Self { mappings })
    }

    /// Map a Toggl time entry to a Timeular activity id
    pub fn map(&self, entry: &TogglTimeEntry) -> Option<String> {
        self.mappings.iter().find_map(|mapping| mapping.map(entry))
    }
}

#[derive(Deserialize, Debug)]
struct TimeEntryFilter {
    project: String,
    description: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ActivityMapping {
    filter: TimeEntryFilter,
    activity_id: String,
}

impl ActivityMapping {
    fn map(&self, entry: &TogglTimeEntry) -> Option<String> {
        if entry.project != self.filter.project {
            return None;
        }

        match self.filter.description.as_ref() {
            Some(description) if description.to_lowercase() != entry.description.to_lowercase() => {
                None
            }
            _ => Some(self.activity_id.clone()),
        }
    }
}
