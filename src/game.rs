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

    let image = pscreen
        .clone()
        .capture_area(
            0,
            (pscreen.clone().display_info.height - 50)
                .try_into()
                .unwrap(),
            500,
            50,
        )
        .unwrap();


    let buffer = image.buffer();
    fs::write("ping.png", &buffer).unwrap();

    // Convert the u8 buffer to rgb and then make a mask
    // Put the mask in an new image

    let mut new_img = image::RgbaImage::new(image.width(), image.height());

    // Crippling depressurizatuion
    let better_image = ImageReader::open("ping.png").unwrap().decode().unwrap();
    let pixels = better_image.as_rgba8().unwrap().as_rgb();
    
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

            //assert!(Srgb::new(pixel.r as f32, pixel.g as f32, pixel.b as f32).red == pixel.r as f32 && Srgb::new(pixel.r as f32, pixel.g as f32, pixel.b as f32).green == pixel.g as f32 && Srgb::new(pixel.r as f32, pixel.g as f32, pixel.b as f32).blue == pixel.b as f32);

            // If not bright enough, make it black
            //if hsvcolor.value < 0.8 {
            //    new_img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            //}

            //else {
            //    new_img.put_pixel(x, y, Rgba([pixel.r, pixel.g, pixel.b, 255]));
            //}
            new_img.put_pixel(x, y, image::Rgba([pixel.r, pixel.g, pixel.b, 255]));
            
        }
    }

    

    // We need to write it aaaaaaaaaaa
    new_img.save("./pingp.jpg");

    // Give it to leptonica to get the cocaine

    let mut ocrresult = ocr::ocr_ping();

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
