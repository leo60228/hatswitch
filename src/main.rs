use hatswitch::EntryType;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let file = env::args().nth(1).unwrap();
    let data = fs::read(file).unwrap();
    let state = hatswitch::gamestate(&data).unwrap().1;
    for i in 0..2 {
        for entry in &state.entries {
            if (i == 0) ^ (entry.typ == EntryType::Directory) {
                continue;
            }
            print!(
                "{}|{}|",
                entry.name,
                match entry.typ {
                    hatswitch::EntryType::File => "F",
                    hatswitch::EntryType::Directory => "D",
                }
            );
            print!(
                "{}|{}|{}|{:x}",
                entry.create,
                entry.access,
                entry.modify,
                entry.data.len()
            );
            print!("\n");
        }
    }
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
}
