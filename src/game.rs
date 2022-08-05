/*

Game file that contains our funtions that give us game data
Pls no sue DE

*/

use screenshots::{Screen, DisplayInfo};
use std::{fs, time::Instant, num::ParseIntError};
use substring::Substring;

pub mod ocr;

pub fn get_ping() -> Result<u16, ParseIntError> {
    // Get screenshot from monitor
    
    let screens = Screen::all().unwrap();

    // Dummy screen is first one in vector
    let mut pscreen = screens.clone().into_iter().nth(0).unwrap();

    // Look for the primary one
    for screen in screens {
        if screen.display_info.is_primary {
            // that one is now our primary one
            pscreen = screen.clone();
        }
    }

    // And take a screenshot on that one

    let image = pscreen.clone().capture_area(0, (pscreen.clone().display_info.height - 20).try_into().unwrap(), 500, 20).unwrap();
    let buffer = image.buffer();
    // We need to write it aaaaaaaaaaa
    fs::write("/tmp/ping.png", &buffer).unwrap();

    // Give it to leptonica to get the cocaine

    let mut ocrresult = ocr::ocr_ping();

    // Return what we get
    ocrresult
    
    


}