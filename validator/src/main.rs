use std::fs::File;
use walkdir::WalkDir;
use nucleoid_leaderboards::model::LeaderboardDefinition;

fn main() {
    let mut failed = false;

    for entry in WalkDir::new("leaderboards/").into_iter()
        .filter_map(|e| e.ok()) {
        let filename = entry.file_name().to_string_lossy();
        if filename.ends_with(".json") {
            if let Err(e) = serde_json::from_reader::<_, LeaderboardDefinition>(&File::open(entry.path()).unwrap()) {
                println!("{:?}: {}", entry.path(), e);
                failed = true;
            }
        }
    }

    if failed {
        std::process::exit(1);
    }
}
