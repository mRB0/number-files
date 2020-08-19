use std::io;
use std::fs::{self, DirEntry, Metadata};
use std::time::SystemTime;
use std::error::Error;

#[derive(Debug)]
struct ExpandedDirEntry {
    dir_entry: DirEntry,
    file_name: String,
    metadata: Metadata,
    modified: SystemTime
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        println!("usage: {} path [...]", std::env::args().nth(0).unwrap_or("main".to_owned()));
        std::process::exit(2);
    }

    for path in args {
        println!("{}", path);

        let mut entries: Vec<ExpandedDirEntry> = 
            fs::read_dir(path)?
            .map(|entry_result| { entry_result
                .map(|entry| {
                    let metadata = entry.metadata()?;

                    let file_name = entry.file_name().into_string().map_err(|_| "The filename couldn't be decoded as Unicode; cowardly refusing to operate on it")?;

                    if !metadata.is_file() {
                        eprintln!("warning: skipping non-file: {}", entry.path().to_string_lossy());
                        Ok(None)
                    } else {

                        let modified = metadata.modified()?;
                        Ok(Some(ExpandedDirEntry { dir_entry: entry, file_name, metadata, modified }))

                    }

                })
            })
            
            .collect::<Result<Vec<Result<Option<ExpandedDirEntry>, Box<dyn Error>>>, io::Error>>()?
            .into_iter().collect::<Result<Vec<Option<ExpandedDirEntry>>, Box<dyn Error>>>()?
            .into_iter().filter_map(|opt| opt).collect();

        entries.sort_by_key(|ede| ede.modified);

        for (i, ede) in entries.iter().enumerate() {
            if let Some(parent) = ede.dir_entry.path().parent() {
                let mut new_path = parent.to_owned();

                let file_name = &ede.file_name;
                let new_file_name = format!("{:02}. {}", i + 1, file_name);
                new_path.push(&new_file_name);

                println!("    {} -> {}", ede.file_name, new_file_name);

                if let Err(io_err) = fs::rename(ede.dir_entry.path(), new_path) {
                    println!("        Error: {}", io_err.to_string());
                };

            }
        }
    }

    Ok(())

}
