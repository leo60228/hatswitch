use chrono::{TimeZone, Utc};
use hatswitch::{Entry, EntryType, GameState};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(Debug, StructOpt)]
#[structopt(about = "hat switch save editor")]
enum HatSwitch {
    Unpack {
        #[structopt(long, short, parse(from_os_str), default_value = ".")]
        output: PathBuf,
        #[structopt(name = "FILE", parse(from_os_str))]
        file: PathBuf,
    },
    Pack {
        #[structopt(name = "DIR", parse(from_os_str))]
        dir: PathBuf,
        #[structopt(name = "FILE", parse(from_os_str))]
        file: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match HatSwitch::from_args() {
        HatSwitch::Unpack { output, file } => {
            let data = fs::read(file)?;
            let state = hatswitch::parse(&data)
                .unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    exit(1);
                })
                .1;
            for entry in &state.entries {
                match entry.typ {
                    EntryType::File => {
                        let mut path = output.clone();
                        path.push(entry.name.trim_start_matches('/'));
                        println!("{}", path.display());
                        if let Some(parent) = path.parent() {
                            fs::create_dir_all(parent)?;
                        }
                        fs::write(path, &entry.data)?;
                    }
                    EntryType::Directory => {
                        let mut path = output.clone();
                        path.push(entry.name.trim_start_matches('/'));
                        println!("{}", path.display());
                        fs::create_dir_all(path)?;
                    }
                }
            }
        }
        HatSwitch::Pack { dir, file } => {
            let epoch = Utc.timestamp_nanos(0);
            let mut entries = vec![];
            let mut dirs = HashSet::new();

            for entry in WalkDir::new(&dir) {
                let entry = entry?;
                if !entry.file_type().is_file() {
                    continue;
                }
                let mut path = PathBuf::from("/");
                path.push(entry
                    .path()
                    .strip_prefix(&dir)
                    .unwrap_or_else(|_| entry.path()));

                if let Some(parent) = path.parent() {
                    if !dirs.contains(parent) {
                        println!("{}", parent.display());
                        entries.push(Entry {
                            typ: EntryType::Directory,
                            create: epoch,
                            access: epoch,
                            modify: epoch,
                            name: parent.to_string_lossy().into_owned(),
                            data: <&[u8]>::into(&[]),
                        });
                        dirs.insert(PathBuf::from(parent));
                    }
                }

                println!("{}", path.display());

                entries.push(Entry {
                    typ: EntryType::File,
                    create: epoch,
                    access: epoch,
                    modify: epoch,
                    name: path.to_string_lossy().into_owned(),
                    data: fs::read(entry.path())?.into(),
                });
            }

            let gamestate = GameState { entries };

            let mut file = fs::OpenOptions::new().write(true).create(true).open(file)?;
            hatswitch::write(&mut file, &gamestate)?;
        }
    }

    Ok(())
}
