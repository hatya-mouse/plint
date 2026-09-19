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
        if path_nolink {
            println!("{}: {}", ruleset_name, ruleset_path.display());
        } else if path {
            println!("{}: {}", ruleset_name, hyperlink(&ruleset_path));
        } else {
            println!("{}", ruleset_name);
        }
    }
}
