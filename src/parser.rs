use super::*;
use chrono::{DateTime, TimeZone, Utc};
use nom::branch;
use nom::bytes::complete as bytes;
use nom::combinator;
use nom::multi;
use nom::number::complete as number;
use nom::sequence;
use nom::IResult;

pub fn c_str(inp: &[u8]) -> IResult<&[u8], String> {
    let (mut r, o) = bytes::take_until(b"\0" as &[u8])(inp)?;
    r = &r[1..];
    Ok((r, String::from_utf8_lossy(o).into_owned()))
}

pub fn entry_type(inp: &[u8]) -> IResult<&[u8], EntryType> {
    branch::alt((
        combinator::value(
            EntryType::File,
            bytes::tag(&[
                0xb1u8, 0xcau8, 0x20u8, 0x97u8, 0xb4u8, 0x81u8, 0x00u8, 0x00u8,
            ]),
        ),
        combinator::value(
            EntryType::Directory,
            bytes::tag(&[
                0x01u8, 0xa4u8, 0x20u8, 0xb2u8, 0xfdu8, 0x41u8, 0x00u8, 0x00u8,
            ]),
        ),
    ))(inp)
}

pub fn windows_to_datetime(win: u64) -> DateTime<Utc> {
    const DIFFERENCE: i128 = 11_644_473_600_000 * 10000; // difference between windows epoch and unix epoch in 100ns
    let unix_100ns = ((win as i128) - DIFFERENCE) as i64;
    let unix_ns = unix_100ns * 100;
    Utc.timestamp_nanos(unix_ns)
}

pub fn filetime(inp: &[u8]) -> IResult<&[u8], DateTime<Utc>> {
    combinator::map(number::le_u64, windows_to_datetime)(inp)
}

pub fn entry(inp: &[u8]) -> IResult<&[u8], Entry> {
    let (r, (typ, create, access, modify, _, name)) = sequence::tuple((
        entry_type,
        filetime,
        filetime,
        filetime,
        number::le_u16,
        c_str,
    ))(inp)?;
    let (r, length) = combinator::cond(
        typ == EntryType::File,
        sequence::terminated(number::le_u64, number::le_u32),
    )(r)?;
    let (r, data): (&[u8], &[u8]) = if let Some(length) = length {
        bytes::take(length)(r)?
    } else {
        (r, &[])
    };
    Ok((
        r,
        Entry {
            typ,
            create,
            access,
            modify,
            name,
            data,
        },
    ))
}

pub fn gamestate(inp: &[u8]) -> IResult<&[u8], GameState> {
    let (r, (_, entries)) = sequence::tuple((bytes::take(8usize), multi::many0(entry)))(inp)?;
    Ok((r, GameState { entries }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_c_str() {
        assert_eq!(
            c_str(b"test\0end" as &[u8]),
            Ok((b"end" as &[u8], "test".to_string()))
        );
    }
}
