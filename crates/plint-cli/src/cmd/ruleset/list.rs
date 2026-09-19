use crate::{storage::load_index_file, utils::hyperlink};

pub(crate) fn list(paths: bool, paths_nolinks: bool) {
    let index_file = match load_index_file() {
        Ok(index_file) => index_file,
        Err(e) => {
            eprintln!("Failed to load index file: {:#?}", e);
            return;
        }
    };

    for (ruleset_name, ruleset_path) in index_file.rulesets {
        println!("{}", ruleset_name);

        if paths_nolinks {
            println!("  {}", ruleset_path.display());
        } else if paths {
            println!("  {}", hyperlink(&ruleset_path));
        }
    }
}
