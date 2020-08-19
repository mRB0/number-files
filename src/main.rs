use std::io;
use std::fs::{self, DirEntry, Metadata};
use std::time::SystemTime;

#[derive(Debug)]
struct ExpandedDirEntry {
    dir_entry: DirEntry,
    file_name: String,
    metadata: Metadata,
    modified: SystemTime
}

fn main() -> io::Result<()> {
    for path in std::env::args().skip(1) {
        println!("{}", path);

        let mut entries: Vec<ExpandedDirEntry> = fs::read_dir(path)?
            .filter_map(|entry_result| entry_result
                .map(|entry| {

                    let file_name = entry.file_name().into_string().map_err(|_| io::Error::new(io::ErrorKind::Other, "Bad unicodes in the filename!"))?;
                    let metadata = entry.metadata()?;
                    let modified = metadata.modified()?;

                    Ok(ExpandedDirEntry { dir_entry: entry, file_name, metadata, modified })

                }).ok().map(|inner: io::Result<ExpandedDirEntry>| inner.ok()).flatten()

                .filter(|ede| ede.metadata.is_file())
                .filter(|ede| ede.file_name.ends_with(".mp3"))
            )
            .collect();

        entries.sort_by_key(|ede| ede.modified);

        for (i, ede) in entries.iter().enumerate() {
            if let Some(parent) = ede.dir_entry.path().parent() {
                let mut new_path = parent.to_owned();

                let file_name = &ede.file_name;
                let new_file_name = format!("{:02}. {}", i + 1, file_name);
                new_path.push(&new_file_name);

                println!("    {} -> {}", ede.file_name, new_file_name);

                if let Err(io_err) = fs::rename(ede.dir_entry.path(), new_path) {
                    println!("@@@ Error: {}", io_err.to_string());
                };

            }
        }
    }

    Ok(())
}
