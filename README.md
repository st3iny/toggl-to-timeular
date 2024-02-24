## toggl-to-timeular

Migrate time entries from Toggl Track to Timeular.

## How to use?

1. Clone the repository.
2. Create a new API key at Timeular: https://app.timeular.com/#/settings/account
3. Save the API key in a file called `credentials.txt`. The file should contain 2 lines:
   - The first line should contain the API key.
   - The second line should contain the API secret.
4. List all of your activites and save the output: `cargo run -- --credentials=credentials.txt list-activities > activities.json`
5. Create a time entry mapping spec at `activity_mapping.json`. The syntax is described below.
6. Export time entries as csv files from Toggl Track and lookup your time zone. See below for
   instructions. Ids of activities can be found in the `activities.json` file from step 4.
7. Import time entries: `cargo run -- --credentials=credentials.txt import --activity-mappings=activity_mappings.json --toggl-time-zone=Europe/Berlin --dry-run ~/downloads/Toggl_time_entries*.csv`
8. If the dry run looks good, remove the `--dry-run` flag and run the program again.

Run `cargo run -- --help` or `cargo run -- <COMMAND> --help` for more information.
Set the `RUST_LOG` environment variable to `debug` to get more verbose logging.

## Exporting time entries from Toggl

1. Open your profile settings on Toggl Track: https://track.toggl.com/profile
2. **Important:** Note your `reports timezone` at the top of the page. You will need it later.
2. Click on `Export account data` on the top right.
3. Enter the year you want to export below `Time entries` and click `Export time entries`.
4. A csv file will downloaded, e.g `Toggl_time_entries_2023-01-01_to_2023-12-31.csv`
5. *Optional:* Filter the time entries using your favorite text editor if you only want to import a
   certain month.

## Mapping spec

The rules to map Toggl Track time entries to Timeular activities are defined in a JSON file.
They will be matched from top to bottom, so the first match will be used.
Entries without a match will be ignored.

```json
[
  {
    "filter": {
      "project": "work",
      "description": "team call"
    },
    "activity_id": "100000"
  },
  {
    "filter": {
      "project": "work"
    },
    "activity_id": "100001"
  },
  {
    "filter": {
      "project": "uni",
      "description": "algorithms lecture"
    },
    "activity_id": "100002"
  },
  {
    "filter": {
      "project": "uni",
      "description": "thesis"
    },
    "activity_id": "100003"
  },
  {
    "filter": {
      "project": "uni"
    },
    "activity_id": "100004"
  }
]
```

The project is matched case sensitively.
The description is matched case insensitively and can be omitted.

The filter code is located in `src/mapping.rs` and can be extended to match more fields in case you
need more complex rules.
The code should be self-explanatory.
