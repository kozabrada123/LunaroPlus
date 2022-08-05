/*

Ocr file that has direct links to leptess for ocr.
called by game

*/


use std::{fmt::Error, error::Error as ErrorTrait, num::ParseIntError, fmt};
use substring::Substring;

pub fn ocr_ping() -> Result<u16, ParseIntError> {
    // Use leptonica to get ocr and parse it
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image("/tmp/ping.png");

    let result = lt.get_utf8_text().unwrap();

    // We now have raw utf8, get pingeth
    let ping = result.substring(result.to_lowercase().find("ping").unwrap_or(0) + 3 + 2, result.to_lowercase().rfind("ms").unwrap_or(0));

    // Return whether or not we have a valid ping
    ping.parse::<u16>()

}