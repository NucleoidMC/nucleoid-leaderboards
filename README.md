# nucleoid-leaderboards

Contains the JSON files used to control the leaderboards for Nucleoid, along with rust code for working with them.

**This is currently heavily WIP, and there are only a few leaderboards included that are used for testing.**

## Repo contents

| Path | Description |
| --- | --- |
| `leaderboards/` | The JSON files that represent the leaderboards themselves |
| `src/` | Rust code containing types for working with the leaderboard JSON data |
| `validator/` | Small rust program to check that all the leaderboard files are valid. Is used by GitHub Actions to check pull requests |

GitHub Actions is used to ensure that leaderboard JSON files are all correct and valid whenever changes are made, and will fail the workflow run if they are not.
