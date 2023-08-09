pub mod display;
pub mod world;
pub mod fire;
pub mod ui;
pub mod common;
pub mod debug;
pub mod input_manager;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};

use ui::render;

pub const FIRE: bool = true;
pub const DEBUG: bool = true;


pub fn main() {
    //initialize the display
    let mut init = display::init();
    let window_context = &mut init.0;
    let mut map = init.1;

    //start the game loop
    loop {
        window_context.canvas.clear();
        let start = SystemTime::now();
        if !display::run( window_context, &mut map){break;}
        let after_display = SystemTime::now();
        render( window_context);
        let _ = window_context.canvas.present();
        thread::sleep(time::Duration::from_millis(100));


    }
}
