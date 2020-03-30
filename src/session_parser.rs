use nom::{
    named, do_parse, tag, many0, map_res, char, terminated, take_until,
    character::complete::alphanumeric1
};
use std::str;
use std::collections::HashMap;

// using parser combinators to parse a (key=value\n)* file is definitely overkill but idc

#[allow(dead_code)] // rust is dum
fn byte_slice_to_str<'a>(s: &'a[u8]) -> Result<&'a str, str::Utf8Error> {
    str::from_utf8(s)
}

named!(pub session<HashMap<&str, &str>>,
    do_parse!(
        tag!("[Desktop Entry]\n") >>
        kvs: many0!(terminated!(keyval, char!('\n'))) >>
        (kvs.into_iter().collect())
    )
);

named!(keyval<(&str, &str)>,
    do_parse!(
        key: map_res!(alphanumeric1, byte_slice_to_str) >>
        char!('=') >>
        value: map_res!(take_until!("\n"), byte_slice_to_str) >>
        (key, value)
    )
);
