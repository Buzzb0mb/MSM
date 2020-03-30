extern crate nom;

mod session_parser;
mod error;
mod session;

use session::Session;
use error::{MSMResult, MSMError};
use std::convert::TryFrom;
use std::fs::{self, DirEntry};
use std::io;

static XSESSIONS_PATH: &str = "/usr/share/xsessions/";

fn is_session_file(dent: &DirEntry) -> bool {
    match dent.path().extension() {
        Some(ext) => ext == "desktop",
        None => false
    }
}

fn read_and_parse(dent: &DirEntry) -> MSMResult<Session> {
    let data = fs::read_to_string(dent.path())?; // read file at dent.path() to a string
    Session::try_from(data.as_bytes()) // create a session from the bytes
}

fn process_input(sessions: &[Session]) -> MSMResult<()> {
    sessions.iter().enumerate().for_each(|(i, s)| println!("  {}: {}", i, s)); // print available sessions

    let mut input = String::new();
    io::stdin().read_line(&mut input)?; // read a line of input
    let index: usize = input.trim_end().parse()?; // try to parse it

    match sessions.get(index) {
        Some(s) => s.execute(), // run the x session
        None => Err(MSMError::InvalidSessionIndex(index))
    }
}

fn main() {
    let sessions: Vec<DirEntry> = fs::read_dir(XSESSIONS_PATH).expect("[ERROR] Couldn't read xsessions path")
        .filter_map(|dent| dent.ok()) // ignore dirents we couldn't read
        .filter(is_session_file)      // ignore non .desktop files
        .collect();

    let parsed_sessions: Vec<Session> = sessions.iter()
        .filter_map(|dent| match read_and_parse(dent) { // read and parse each session file, warn about files we couldn't
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("[WARN] ({}) {}", dent.path().as_path().display(), e);
                None
            }
        })
        .collect();

    loop {
        match process_input(&parsed_sessions) {
            Ok(_) => (),
            Err(e) => eprintln!("[WARN] {}", e)
        };
    }
}
