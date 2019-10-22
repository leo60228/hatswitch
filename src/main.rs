use std::env;
use std::fs;

fn main() {
    let file = env::args().nth(1).unwrap();
    let data = fs::read(file).unwrap();
    let state = hatswitch::gamestate(&data).unwrap().1;
    for i in 0..2 {
        for entry in &state.entries {
            if (i == 0) ^ (entry.typ == hatswitch::EntryType::Directory) {
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
            for byte in entry.unknown {
                print!("{:02x}, ", byte);
            }
            print!("|{:x}", entry.data.len());
            if let Some(len) = entry.raw_length {
                let diff: isize = (entry.data.len() as isize) - (len as isize);
                print!("|{:x}|{}", len, diff);
            }
            print!("\n");
        }
    }
}
