use std::{
    fs::File,
    io::{stdout, BufRead, BufReader},
    path::Path,
};

use anyhow::{anyhow, Context, Result};
use chrono_tz::Tz;
use clap::Parser;

use crate::mapping::ActivityMapper;
use crate::timeular::{Note, SignInRequest, TimeularClient};
use crate::toggl::TogglTimeEntry;

mod mapping;
mod timeular;
mod toggl;

/// Import time entries from Toggl to Timeular
#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to a file with 2 lines: API key and API secret
    #[clap(long, required = true)]
    credentials: String,

    #[command(subcommand)]
    command: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Print all activities of Timeular as a JSON array to stdout
    ListActivities,

    /// Import time entries from Toggl to Timeular
    Import {
        /// Don't actually import anything
        #[clap(short = 'n', long)]
        dry_run: bool,

        /// Time zone to use when parsing time entries from Toggl
        #[clap(long, default_value = "Europe/Berlin")]
        toggl_time_zone: String,

        /// Path to a file with activity mappings (json array, matched top to bottom)
        #[clap(long, required = true)]
        activity_mappings: String,

        /// Paths to Toggl time entry csv exports
        #[clap(required = true)]
        toggl_exports: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let args = Args::parse();

    let (api_key, api_secret) = parse_credentials(args.credentials)?;
    let client = TimeularClient::sign_in(&SignInRequest {
        api_key,
        api_secret,
    })
    .await?;

    match args.command {
        Subcommand::ListActivities => {
            let activites = client.list_activities().await?;
            serde_json::to_writer_pretty(stdout().lock(), &activites.activities)?;
        }
        Subcommand::Import {
            dry_run,
            toggl_time_zone,
            toggl_exports,
            activity_mappings,
        } => {
            let toggl_tz: Tz = toggl_time_zone
                .as_str()
                .parse()
                .map_err(|err| anyhow!("Failed to parse time zone: {err}"))?;

            let activity_mapper = ActivityMapper::parse(activity_mappings)?;

            let mut unmapped_entries = Vec::new();
            let mut imported_count = 0;
            for export in toggl_exports {
                let mut reader = csv::Reader::from_path(export)?;
                for record in reader.deserialize() {
                    let time_entry: TogglTimeEntry = record?;
                    log::debug!("{time_entry:#?}");

                    // Timeular doesn't allow time entries shorter than 1 minute
                    let start = time_entry.start(toggl_tz)?;
                    let end = time_entry.end(toggl_tz)?;
                    if end - start <= chrono::Duration::seconds(60) {
                        log::warn!("Skipping short time entry: {time_entry:#?}");
                        continue;
                    }

                    let activity_id = activity_mapper.map(&time_entry);
                    let Some(activity_id) = activity_id else {
                        unmapped_entries.push(time_entry);
                        continue;
                    };

                    let request = timeular::CreateTimeEntryRequest {
                        started_at: start
                            .to_utc()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                            .trim_end_matches('Z')
                            .to_string(),
                        stopped_at: end
                            .to_utc()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                            .trim_end_matches('Z')
                            .to_string(),
                        activity_id,
                        note: Note {
                            text: time_entry.description.clone(),
                        },
                    };
                    log::debug!("{request:#?}");

                    imported_count += 1;

                    if dry_run {
                        log::info!("Would have imported: {request:#?}");
                        continue;
                    }

                    client.create_time_entry(&request).await?;
                    log::info!("Imported: {request:#?}");
                }
            }

            if !unmapped_entries.is_empty() {
                log::warn!(
                    "Skipped {} time entries without a mapping: {unmapped_entries:#?}",
                    unmapped_entries.len(),
                );
            }

            if dry_run {
                log::info!("Would have imported {imported_count} time entries");
            } else {
                log::info!("Imported {imported_count} time entries");
            }
        }
    }

    Ok(())
}

fn parse_credentials(path: impl AsRef<Path>) -> Result<(String, String)> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(file).lines();
    Ok((
        lines.next().context("Missing api key")??,
        lines.next().context("Missing api secret")??,
    ))
}
