use nom::branch;
use nom::bytes::complete as bytes;
use nom::combinator;
use nom::sequence;
use nom::IResult as DefaultIResult;

type IResult<T1, T2> = DefaultIResult<T1, T2, nom::error::VerboseError<T1>>;

#[derive(Debug)]
pub struct GameState<'a> {
    pub header: &'a [u8],
    pub entry: Vec<Entry<'a>>,
}

#[derive(Debug)]
pub struct Entry<'a> {
    pub unknown: &'a [u8],
    pub typ: EntryType,
    pub unknown2: &'a [u8],
    pub name: String,
}

#[derive(Debug, Copy, Clone)]
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
    dbg!(&inp[..4]);
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
    let (r, (unknown, typ, unknown2, name)) = sequence::tuple((
        bytes::take(4usize),
        entry_type,
        bytes::take(28usize),
        c_str,
    ))(inp)?;
    Ok((
        r,
        Entry {
            unknown,
            typ,
            unknown2,
            name,
        },
    ))
}

pub fn gamestate(inp: &[u8]) -> IResult<&[u8], GameState> {
    let (r, (header, entry)) = sequence::tuple((bytes::take(6usize), entry))(inp)?;
    Ok((
        r,
        GameState {
            header,
            entry: vec![entry],
        },
    ))
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
