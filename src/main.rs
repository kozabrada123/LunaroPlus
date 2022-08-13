/*

Main file with our loops and a controller.

*/

use std::{process::Output, time::Instant};
mod game;

use std::{thread::sleep, time::Duration};

fn main() {
    // Clear the screen immediately
    //print!("\x1B[2J\x1B[1;1H");

    // Game's data as a struct
    let mut game = game::types::GameData::new();

    let mut out_p = String::new();
    let mut out = String::new();
    
    // Main loop
    loop {
        let start = Instant::now();

        // Try to get ping and see if we can
        let pong = game::get_ping(&mut game);

        match pong {
            Ok(ping) => {
                // Yay! we can!
                game._all_ping += ping as u32;

                game.measurments += 1;

                game.curr_ping = ping;

                game.average_ping =  game._all_ping as f32 / game.measurments as f32;

                out = format!(
"Cur: {}ms
Avg: {:.1}ms",
                    game.curr_ping,
                    game.average_ping,
                );
            }
            Err(_) => {
                match game.status {
                    game::types::GameStatus::StatusGameHost {} => {out = "Cur: 0ms".to_string();},
                    game::types::GameStatus::StatusInGame {} => {out = "Can't get ocr..".to_string(); },
                    game::types::GameStatus::StatusNoGame {} => {out = out_p.clone(); },
                }
            }
        }

        //println!("{:?}", game::get_score());

        // Only print if it isnt the exact same
        if out_p != out {

            // Clear if it isn't "Can't get ocr.."
            //if out != "Can't get ocr..".to_string() {print!("\x1B[2J\x1B[1;1H");}
            println!("{}", &out);
        }

        out_p = out.clone();
    }
}


fn get_players() {
    println!("getting players..");

    let players = game::get_primary_players();

    //Get screenshot
    println!("Sun: {}", players);
}