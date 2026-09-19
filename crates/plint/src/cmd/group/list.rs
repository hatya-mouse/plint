use crate::{storage::IndexFile, utils::hyperlink};

pub(crate) fn list(paths: bool, paths_nolinks: bool) {
    let index_file = match IndexFile::load() {
        Ok(index_file) => index_file,
        Err(e) => {
            eprintln!("Failed to load index file: {:#?}", e);
            return;
        }
    };

    for (group_name, group_path) in index_file.groups {
        println!("{}", group_name);

        if paths_nolinks {
            println!("  {}", group_path.display());
        } else if paths {
            println!("  {}", hyperlink(&group_path));
        }
    }
}
