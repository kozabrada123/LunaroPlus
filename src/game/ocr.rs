/*

Ocr file that has direct links to leptess for ocr.
called by game

*/

use regex::Regex;
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

    let out_num: Result<u16, ParseIntError> = ping.parse::<u16>();

    if out_num.is_err() {
        // we have no valid ping measurement, check if we are in a game of lunaro / warframe
        if result.to_lowercase().contains("rate") {
            // We are, either in a random wf game or we are host
            // Signify that we are host
            return Ok(0);
        }
    }

    // Return whether or not we have a valid ping
    out_num
}

pub fn ocr_text() -> String {
    // Use leptonica to get ocr and parse it
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image("./scorep.png");

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


pub fn ocr_score() -> [u8;2] {
    // Use leptonica to get ocr and parse it
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image("./score.png");

    let mut result = lt.get_utf8_text().unwrap();

    let mut score_a = 0;
    let mut score_b = 0;

    if Regex::new(r#"(\d*-\d*)"#).unwrap().is_match(result.as_str()) {

        println!("{}", result.as_str());

        let a = result.substring(
            0,
            result.to_lowercase().rfind("-").unwrap_or(0),
        );

        let b = result.substring(
            result.to_lowercase().rfind("-").unwrap_or(0) + 1,
            result.len(),
        );

        println!("{}", a);

        println!("{}", b);

        score_a = a.replace("\n", "").replace(r#" "#, "").parse().unwrap();

        score_b = b.replace("\n", "").replace(r#" "#, "").parse().unwrap();
    }

    [score_a, score_b]
}
