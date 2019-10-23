use super::*;
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::{DateTime, Utc};
use std::convert::TryInto;
use std::io::prelude::*;
use std::io::{self, Error, ErrorKind};

pub fn timestamp(w: &mut dyn Write, time: DateTime<Utc>) -> io::Result<()> {
    const DIFFERENCE: i64 = 11_644_473_600_000 * 10000; // difference between windows epoch and unix epoch in 100ns
    let unix_100ns = time.timestamp_nanos() / 100;
    let windows = unix_100ns + DIFFERENCE;
    w.write_u64::<LittleEndian>(windows.try_into().unwrap())?;
    Ok(())
}

pub fn entry(w: &mut dyn Write, entry: &Entry) -> io::Result<()> {
    w.write_all(match entry.typ {
        EntryType::File => &[
            0xb1u8, 0xcau8, 0x20u8, 0x97u8, 0xb4u8, 0x81u8, 0x00u8, 0x00u8,
        ],
        EntryType::Directory => &[
            0x01u8, 0xa4u8, 0x20u8, 0xb2u8, 0xfdu8, 0x41u8, 0x00u8, 0x00u8,
        ],
    })?;
    timestamp(w, entry.create)?;
    timestamp(w, entry.access)?;
    timestamp(w, entry.modify)?;
    w.write_u16::<LittleEndian>(
        entry
            .name
            .len()
            .try_into()
            .map_err(|_| Error::new(ErrorKind::Other, "name too long"))?,
    )?;
    w.write_all(entry.name.as_ref())?;
    w.write_u8(0)?;
    if entry.typ == EntryType::File {
        w.write_u64::<LittleEndian>(entry.data.len() as u64)?;
        w.write_all(&[0xe9, 0xb7, 0x12, 0x3a])?;
        w.write_all(entry.data)?;
    }
    Ok(())
}

pub fn gamestate(w: &mut dyn Write, gamestate: &GameState) -> io::Result<()> {
    w.write_all(&[
        0x13u8, 0x22u8, 0xb2u8, 0xe5u8, 0xa8u8, 0x04u8, 0x10u8, 0xdcu8,
    ])?; // magic
    for e in &gamestate.entries {
        entry(w, e)?;
    }
    Ok(())
}
