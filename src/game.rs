/*

Game file that contains our funtions that give us game data
Pls no sue DE

*/
use image::{Rgb, Rgba, ImageError};
use image::io::Reader as ImageReader;
use rgb::*;
use screenshots::{DisplayInfo, Screen};
use std::{fs, num::ParseIntError, time::Instant};
use std::error::Error;
use substring::Substring;
use palette::{ Hsv, Srgb, FromColor, LinSrgb, Hsl, Srgba, IntoColor};

pub mod ocr;
pub mod types;

pub fn get_ping(gamedata: &mut types::GameData) -> Result<u16, ParseIntError> {
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

    let image = pscreen
        .clone()
        .capture_area(
            0,
            (pscreen.clone().display_info.height - 100)
                .try_into()
                .unwrap(),
            500,
            100,
        )
        .unwrap();


    let buffer = image.buffer();
    fs::write("ping.png", &buffer).unwrap();

    // Convert the u8 buffer to rgb and then make a mask
    // Put the mask in an new image

    /*let mut new_img = image::RgbaImage::new(image.width(), image.height());

    // Crippling depressurizatuion
    let better_image = ImageReader::open("ping.png").unwrap().decode().unwrap();
    let pixels = better_image.as_rgba8().unwrap().as_rgba();
    
    for y in 1..image.height() { 
        for x in 1..image.width() -1 {

            let index = (((y - 1) * image.width()) + x);

            // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
            // HElp
            if index >= pixels.len() as u32 {break;}

            let mut pixel = pixels[index as usize];

            //println!("{:?}", pixel);

            // Convert to hsv so we can see brightness
            let hsvcolor: Hsv = Srgb::new(pixel.r as f32 / 250 as f32, pixel.g as f32 /250 as f32, pixel.b as f32 / 250 as f32).into_color();

            // Make slight changes
            let hsvchanged = Hsv::new(hsvcolor.hue, 0.0, hsvcolor.value.powf(2.0));

            // back to rgb
            let rgbcolor: Srgba = hsvchanged.clone().into_color();

            //assert!(Srgb::new(pixel.r as f32, pixel.g as f32, pixel.b as f32).red == pixel.r as f32 && Srgb::new(pixel.r as f32, pixel.g as f32, pixel.b as f32).green == pixel.g as f32 && Srgb::new(pixel.r as f32, pixel.g as f32, pixel.b as f32).blue == pixel.b as f32);

            // If not bright enough, make it black
            if hsvcolor.value < 0.4 {
                new_img.put_pixel(x, y, Rgba([0, 0, 0, pixel.a]));
            }

            else {
                new_img.put_pixel(x, y, Rgba([(rgbcolor.red * 255.0) as u8, (rgbcolor.green * 255.0) as u8, (rgbcolor.blue * 255.0) as u8, (rgbcolor.alpha * 255.0) as u8]));
            }
            //new_img.put_pixel(x, y, Rgba([(rgbcolor.red * 255.0) as u8, (rgbcolor.green * 255.0) as u8, (rgbcolor.blue * 255.0) as u8, (rgbcolor.alpha * 255.0) as u8]));
            //new_img.put_pixel(x, y, image::Rgba([pixel.r, pixel.g, pixel.b, pixel.a]));
            
        }
    }

    

    // We need to write it aaaaaaaaaaa
    new_img.save("./pingp.png");*/

    
    // Give it to leptonica to get the cocaine

    let mut ocrresult = ocr::ocr_ping();

    // see what our status is
    match ocrresult {
        Ok(0) => {
            gamedata.status = types::GameStatus::StatusGameHost{}; 
            // Raise an error because aaaa our main func is kinda garbage
            ocrresult = "ligma".parse::<u16>();
        },
        Ok(_num) => gamedata.status = types::GameStatus::StatusInGame{},
        Err(ref ParseIntError) => gamedata.status = types::GameStatus::StatusNoGame{},
    }

    // Return what we get
    ocrresult
}

pub fn get_primary_players() -> String {
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

    /*

    Here we are working with screen percentages
    So like for example 800 / 1920 = 41.6% of the screen width
    So that is how we can then hopefully scale properly from ultrawide, 4k and other resolutions

     */
    let sun_left = pscreen.clone().display_info.width as f32 / 2 as f32 - (800 as f32 / 1920 as f32) * pscreen.clone().display_info.width as f32; //x
    let sun_upper = (450 as f32 / 1080 as f32) * pscreen.clone().display_info.height as f32; //y
    let sun_right = sun_left as f32 + (250 as f32 / 1920 as f32) * pscreen.clone().display_info.width as f32; //x + width
    let sun_lower = sun_upper as f32 + (250 as f32 / 1080 as f32) * pscreen.clone().display_info.height as f32; //y + height

    println!("{}", sun_left);
    println!("{}", sun_upper);
    println!("{}", sun_right);
    println!("{}", sun_lower);

    let image = pscreen
        .clone()
        .capture_area(
            sun_left as i32,
            sun_upper as i32,
            sun_right as u32,
            sun_lower as u32
        )
        .unwrap();

    let buffer = image.buffer();
    // We need to write it aaaaaaaaaaa
    fs::write("./ping.png", &buffer).unwrap();

    // Give it to leptonica to get the cocaine

    let ocrresult = ocr::ocr_text();

    // Return what we get
    ocrresult
}

pub fn get_score() -> [u8; 2] {
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

    /*

    Here we are working with screen percentages
    So like for example 800 / 1920 = 41.6% of the screen width
    So that is how we can then hopefully scale properly from ultrawide, 4k and other resolutions

     */
    let sun_left = pscreen.clone().display_info.width as f32 / 2 as f32 - (25 as f32 / 1920 as f32) * pscreen.clone().display_info.width as f32; //x
    let sun_upper = (62 as f32 / 1080 as f32) * pscreen.clone().display_info.height as f32; //y
    let sun_right = (50 as f32 / 1920 as f32) * pscreen.clone().display_info.width as f32; //width
    let sun_lower = (28 as f32 / 1080 as f32) * pscreen.clone().display_info.height as f32; //height

    let image = pscreen
        .clone()
        .capture_area(
            sun_left as i32,
            sun_upper as i32,
            sun_right as u32,
            sun_lower as u32
        )
        .unwrap();

    let buffer = image.buffer();
    // We need to write it aaaaaaaaaaa
    fs::write("./score.png", &buffer).unwrap();

    // Convert the u8 buffer to rgb and then make a mask
    // Put the mask in an new image

    /*let mut new_img = image::RgbaImage::new(image.width(), image.height());

    // Crippling depressurizatuion
    let better_image = ImageReader::open("score.png").unwrap().decode().unwrap();
    let pixels = better_image.as_rgba8().unwrap().as_rgba();
    
    for y in 1..image.height() { 
        for x in 1..image.width() -1 {

            let index = (((y - 1) * image.width()) + x);

            // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
            // HElp
            if index >= pixels.len() as u32 {break;}

            let mut pixel = pixels[index as usize];

            //println!("{:?}", pixel);

            // Convert to hsv so we can see brightness
            let hsvcolor: Hsv = Srgb::new(pixel.r as f32 / 250 as f32, pixel.g as f32 /250 as f32, pixel.b as f32 / 250 as f32).into_color();

            // Make slight changes
            let hsvchanged = Hsv::new(hsvcolor.hue, hsvcolor.saturation, hsvcolor.value);

            // back to rgb
            let rgbcolor: Srgba = hsvchanged.clone().into_color();

            //assert!(Srgb::new(pixel.r as f32, pixel.g as f32, pixel.b as f32).red == pixel.r as f32 && Srgb::new(pixel.r as f32, pixel.g as f32, pixel.b as f32).green == pixel.g as f32 && Srgb::new(pixel.r as f32, pixel.g as f32, pixel.b as f32).blue == pixel.b as f32);

            // If not bright enough, make it black
            if  calculate_color_similarity(
                // Pain of a line but essentially if its sun or moon or the - then delete it
                &rgbcolor.into_color(), // Sun / orange
                &Srgb::new(0.898, 0.561, 0.224)
                ) < 0.5
                ||
                calculate_color_similarity(
                &rgbcolor.into_color(), // Moon / blue
                &Srgb::new(0.176, 0.784, 0.878)
                ) < 0.5
                ||
                calculate_color_similarity(
                &rgbcolor.into_color(), // White
                &Srgb::new(1.0, 1.0, 1.0)
                ) < 0.5
                 
                 {
                new_img.put_pixel(x, y, Rgba([0, 0, 0, pixel.a]));
            }

            else {
                new_img.put_pixel(x, y, Rgba([(rgbcolor.red * 255.0) as u8, (rgbcolor.green * 255.0) as u8, (rgbcolor.blue * 255.0) as u8, (rgbcolor.alpha * 255.0) as u8]));
            }
            //new_img.put_pixel(x, y, Rgba([(rgbcolor.red * 255.0) as u8, (rgbcolor.green * 255.0) as u8, (rgbcolor.blue * 255.0) as u8, (rgbcolor.alpha * 255.0) as u8]));
            //new_img.put_pixel(x, y, image::Rgba([pixel.r, pixel.g, pixel.b, pixel.a]));
            
        }
    }

    new_img.save("./scorep.png");*/

    // Give it to leptonica to get the cocaine

    let ocrresult = ocr::ocr_score();

    // Return what we get
    ocrresult
}


// Calculates the similarity of two rgb colors.
// returns 0 - 1 float
fn calculate_color_similarity(a: &Srgb, b: &Srgb) -> f32 {
    let mut similarity = 0.0;

    similarity = ((a.red - b.red).powi(2) + (a.green - b.green).powi(2) + (a.blue - b.blue).powi(2)).sqrt();

    return similarity;
}