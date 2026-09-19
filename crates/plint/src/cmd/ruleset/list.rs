use crate::{storage::IndexFile, utils::hyperlink};

pub(crate) fn list(path: bool, path_nolink: bool) {
    let index_file = match IndexFile::load() {
        Ok(index_file) => index_file,
        Err(e) => {
            eprintln!("Failed to load index file: {:#?}", e);
            return;
        }
    };

    for (ruleset_name, ruleset_path) in index_file.rulesets {
        println!("{}", ruleset_name);

        if path_nolink {
            println!("  {}", ruleset_path.display());
        } else if path {
            println!("  {}", hyperlink(&ruleset_path));
        }
    }
}
