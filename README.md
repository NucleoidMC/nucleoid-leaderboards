# nucleoid-leaderboards

Contains the JSON files used to control the leaderboards for Nucleoid, including translations, along with rust code for working with them.

You can translate this project [on Weblate](https://hosted.weblate.org/engage/nucleoid/).

## Repo contents

| Path | Description |
| --- | --- |
| `leaderboards/` | The JSON files that represent the leaderboards themselves |
| `translations/` | The JSON files containing translations. |
| `src/` | Rust code containing types for working with the leaderboard JSON data |
| `validator/` | Small rust program to check that all the leaderboard files are valid. Is used by GitHub Actions to check pull requests |

GitHub Actions is used to ensure that leaderboard JSON files are all correct and valid whenever changes are made, and will fail the workflow run if they are not.
