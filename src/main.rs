/*

Main file with our loops and a controller.

*/

use std::{process::Output, time::Instant};
mod game;

use std::{thread::sleep, time::Duration};

fn main() {
    // Clear the screen immediately
    //print!("\x1B[2J\x1B[1;1H");


    // Pings == sum of all measurments
    let mut pings: u32 = 0;

    // Pingnums == number of measurments
    let mut pingnums: u16 = 0;

    let mut out_p = String::new();
    let mut out = String::new();
    
    // Main loop
    loop {
        let start = Instant::now();

        // Try to get ping and see if we can
        let pong = game::get_ping();

        match pong {
            Ok(ping) => {
                // Yay! we can!
                pings += ping as u32;

                pingnums += 1;

                let avg = pings as f32 / pingnums as f32;

                out = format!(
                    "Cur: {}ms Avg: {:.1}ms Num: {}, Time: {:.2?}",
                    ping,
                    avg,
                    pingnums,
                    start.elapsed()
                );
            }
            Err(_) => {
                // We can't
                // L rip bozo just wait
                out = "Can't get ocr..".to_string();
            }
        }


        // Only print if it isnt the exact same
        if out_p != out {

            // Clear if it isn't "Can't get ocr.."
            //if out != "Can't get ocr..".to_string() {print!("\x1B[2J\x1B[1;1H");}
            println!("{}", out);
        }

        out_p = out;
    }
}


fn get_players() {
    println!("getting players..");

    let players = game::get_primary_players();

    //Get screenshot
    println!("Sun: {}", players);
}