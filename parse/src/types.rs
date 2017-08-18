use std::str::FromStr;
use std;

use nom::{digit};

named!(float_string,
    recognize!(
        tuple!(
            opt!(alt!(tag!("+") | tag!("-"))),
            digit,
            opt!(tuple!(tag!("."), digit))
        )
    )
);

named!(pub float<&[u8], f32>,
    map_res!(
        map_res!(float_string, std::str::from_utf8),
        f32::from_str
    )
);

named!(pub unsigned_integer<&[u8], u32>,
    map_res!(
        map_res!(digit, std::str::from_utf8),
        u32::from_str
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;
    #[test]
    fn parse_float() {
        assert_eq!(float(b"1.0"),       IResult::Done(&b""[..], 1f32));
        assert_eq!(float(b"1 "),        IResult::Done(&b" "[..], 1f32));
        assert_eq!(float(b"+1.0"),      IResult::Done(&b""[..], 1f32));
        assert_eq!(float(b"-1.0"),      IResult::Done(&b""[..], -1f32));
    }

    #[test]
    fn parse_unsigned_integer() {
        assert_eq!(unsigned_integer(b"1"),  IResult::Done(&b""[..], 1u32));
        assert_eq!(unsigned_integer(b"2"),  IResult::Done(&b""[..], 2u32));
    }

}
