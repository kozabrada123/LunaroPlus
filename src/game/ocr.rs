/*

Ocr file that has direct links to leptess for ocr.
called by game

*/

use std::{error::Error as ErrorTrait, fmt, fmt::Error, num::ParseIntError};
use substring::Substring;

pub fn ocr_ping() -> Result<u16, ParseIntError> {
    // Use leptonica to get ocr and parse it
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image("./ping.png");

    let result = lt.get_utf8_text().unwrap();

    // We now have raw utf8, get pingeth
    let ping = result.substring(
        result.to_lowercase().find("ping").unwrap_or(0) + 3 + 2,
        result.to_lowercase().rfind("ms").unwrap_or(0),
    );

    // Return whether or not we have a valid ping
    ping.parse::<u16>()
}

pub fn ocr_text() -> String {
    // Use leptonica to get ocr and parse it
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image("./ping.png");

    let result = lt.get_utf8_text().unwrap();

    let mut out = result.clone();

    for char in out.clone().chars() {
        if !(char.is_alphanumeric() || ['.', '-', '_'].contains(&char)) {
            // If it isnt any of the above, remove all occurences of the char
            out = out.replace(char, "");
        }
    }

    out
}
