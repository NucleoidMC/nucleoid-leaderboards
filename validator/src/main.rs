use std::{fs::File, collections::HashMap};
use walkdir::WalkDir;
use nucleoid_leaderboards::model::LeaderboardDefinition;

fn main() {
    let translations = load_translations();
    let mut failed = false;

    for entry in WalkDir::new("leaderboards/").into_iter()
        .filter_map(|e| e.ok()) {
        let filename = entry.file_name().to_string_lossy();
        if filename.ends_with(".json") {
            match serde_json::from_reader::<_, LeaderboardDefinition>(&File::open(entry.path()).unwrap()) {
                Ok(leaderboard) => {
                    let (ident, path) = leaderboard.id.split_once(":").unwrap();
                    let key = format!("leaderboard.{}.{}", ident, path);
                    if let None = translations.get(&key) {
                        eprintln!("Missing translation for leaderboard {} ({}) in en_us language!", leaderboard.id, filename);
                        failed = true;
                    }
                }
                Err(e) => {
                    eprintln!("{:?}: {}", entry.path(), e);
                    failed = true;
                }
            }
        }
    }

    if failed {
        std::process::exit(1);
    }
}

fn load_translations() -> HashMap<String, String> {
    serde_json::from_reader(&File::open("translations/en_us.json").unwrap()).expect("Failed to parse en_us translations file")
}
