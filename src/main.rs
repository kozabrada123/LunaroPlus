/*

Main file with our loops and a controller.

*/

use std::{time::Instant, process::Output};
mod game;

fn main () {

    // Pings == sum of all measurments
    let mut pings: u32 = 0;

    // Pingnums == number of measurments
    let mut pingnums: u16 = 0;

    let mut out_p = String::new();
    let mut out = String::new();

    loop {

        let start = Instant::now();

        // Try to get ping and see if we can
        let pong =  game::get_ping();

        match pong {
            Ok(ping) => {
                // Yay! we can!
                pings += ping as u32;

                pingnums += 1;

                let avg = pings as f32 / pingnums as f32;
                

                out = format!("Cur: {}ms Avg: {:.1}ms Num: {}, Time: {:.2?}", ping, avg, pingnums, start.elapsed());
            },
            Err(_) => {
                // We can't
                // L rip bozo just wait
                out = "Can't get ocr..".to_string();
            }

        }

        if out_p != out {
            println!("{}", out);
        }

        out_p = out;

    }

}