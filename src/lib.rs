use nom::branch;
use nom::bytes::complete as bytes;
use nom::combinator;
use nom::multi;
use nom::number::complete as number;
use nom::sequence;
use nom::IResult as DefaultIResult;

type IResult<T1, T2> = DefaultIResult<T1, T2, nom::error::VerboseError<T1>>;

#[derive(Debug)]
pub struct GameState<'a> {
    pub header: &'a [u8],
    pub entries: Vec<Entry<'a>>,
}

#[derive(Debug)]
pub struct Entry<'a> {
    pub typ: EntryType,
    pub unknown: &'a [u8],
    pub name: String,
    pub raw_length: Option<u64>,
    pub magic: Option<u32>,
    data: Vec<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EntryType {
    File,
    Directory,
}

fn c_str(inp: &[u8]) -> IResult<&[u8], String> {
    let (mut r, o) = bytes::take_until(b"\0" as &[u8])(inp)?;
    r = &r[1..];
    Ok((r, String::from_utf8_lossy(o).into_owned()))
}

fn entry_type(inp: &[u8]) -> IResult<&[u8], EntryType> {
    branch::alt((
        combinator::value(
            EntryType::File,
            bytes::tag(&[0x20u8, 0x97u8, 0xb4u8, 0x81u8]),
        ),
        combinator::value(
            EntryType::Directory,
            bytes::tag(&[0x20u8, 0xb2u8, 0xfdu8, 0x41u8]),
        ),
    ))(inp)
}

pub fn entry(inp: &[u8]) -> IResult<&[u8], Entry> {
    let (r, (typ, unknown, name)) =
        sequence::tuple((entry_type, bytes::take(28usize), c_str))(inp)?;
    let (r, (raw_length, magic, data, _)) = sequence::tuple((
        combinator::cond(typ == EntryType::File, number::le_u64),
        combinator::cond(typ == EntryType::File, number::le_u32),
        branch::alt((bytes::take_until(b"\xb1\xca" as &[u8]), combinator::rest)),
        combinator::opt(bytes::tag(b"\xb1\xca" as &[u8])),
    ))(r)?;
    Ok((
        r,
        Entry {
            typ,
            unknown,
            name,
            raw_length,
            magic,
            data: data.into(),
        },
    ))
}

pub fn gamestate(inp: &[u8]) -> IResult<&[u8], GameState> {
    let (r, (header, entries)) = sequence::tuple((bytes::take(10usize), multi::many0(entry)))(inp)?;
    Ok((r, GameState { header, entries }))
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
