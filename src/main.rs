use hatswitch::EntryType;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let file = env::args().nth(1).unwrap();
    let out = env::args().nth(2).unwrap();
    let data = fs::read(file).unwrap();
    let state = hatswitch::parse(&data).unwrap().1;
    for entry in &state.entries {
        match entry.typ {
            EntryType::File => {
                let path = Path::new(entry.name.trim_start_matches('/'));
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                fs::write(path, &entry.data).unwrap();
            }
            EntryType::Directory => {
                let path = Path::new(entry.name.trim_start_matches('/'));
                fs::create_dir_all(path).unwrap();
            }
        }
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(out)
        .unwrap();
    hatswitch::write(&mut file, &state).unwrap();
}
