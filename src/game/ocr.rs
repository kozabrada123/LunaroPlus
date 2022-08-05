/*

Ocr file that has direct links to leptess for ocr.
called by game

*/


use substring::Substring;

pub fn ocr_ping() -> u16 {
    // Use leptonica to get ocr and parse it
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image("/tmp/ping.png");

    let result = lt.get_utf8_text().unwrap();
    println!("{}", lt.get_utf8_text().unwrap());

    // We now have raw utf8, get pingeth
    let ping = result.substring(result.to_lowercase().find("ping").unwrap() + 3 + 2, result.to_lowercase().rfind("ms").unwrap());

    // Return result as u8
    return ping.parse::<u16>().unwrap();
}