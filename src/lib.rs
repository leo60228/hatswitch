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
    pub flags: u32,
    pub unknown: &'a [u8],
    pub name: String,
    pub raw_length: Option<u64>,
    pub magic: Option<u32>,
    pub data: Vec<u8>,
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
        combinator::value(EntryType::File, bytes::tag(&[0xb1u8, 0xcau8])),
        combinator::value(EntryType::Directory, bytes::tag(&[0x01u8, 0xa4u8])),
    ))(inp)
}

fn first_match<'a>(strings: &'a [&[u8]]) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> + 'a {
    move |inp| {
        let possibilities = strings.iter().copied().map(|s| bytes::take_until(s)(inp));
        possibilities
            .min_by_key(|r| {
                if let Ok((_, o)) = r {
                    o.len() as isize
                } else {
                    -1
                }
            })
            .unwrap()
    }
}

pub fn entry(inp: &[u8]) -> IResult<&[u8], Entry> {
    let (r, (typ, flags, unknown, name)) =
        sequence::tuple((entry_type, number::le_u32, bytes::take(28usize), c_str))(inp)?;
    let (r, (raw_length, magic, data)) = sequence::tuple((
        combinator::cond(typ == EntryType::File, number::le_u64),
        combinator::cond(typ == EntryType::File, number::le_u32),
        branch::alt((first_match(&[&[0x01, 0xa4], &[0xb1, 0xca]]), combinator::rest)),
    ))(r)?;
    Ok((
        r,
        Entry {
            typ,
            flags,
            unknown,
            name,
            raw_length,
            magic,
            data: data.into(),
        },
    ))
}

pub fn gamestate(inp: &[u8]) -> IResult<&[u8], GameState> {
    let (r, (header, entries)) = sequence::tuple((bytes::take(8usize), multi::many0(entry)))(inp)?;
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
