pub mod common;
pub mod debug;
pub mod display;
pub mod fire;
pub mod input_manager;
pub mod ui;
pub mod world;
pub mod apparatus;

use std::time::SystemTime;
use std::thread;

//X is left and right (width)
//Y is up and down (height)

pub const FIRE: bool = true;
pub const DEBUG: bool = true;

pub fn main() {
    //initialize the display
    let mut init = display::init();
    let window_context = &mut init.0;
    let mut map = init.1;

    //start the game loop
    loop {

        let start_time = SystemTime::now();
        let mut previous_time = SystemTime::now();
        window_context.canvas.clear();

    //display
        if !display::run(window_context, &mut map) {
            break;
        }
        let after_display = previous_time.elapsed().unwrap().as_nanos();

    //UI
        previous_time = SystemTime::now();
        ui::render(window_context);
        ui::mouse_icons(window_context,&mut map);
        let after_ui = previous_time.elapsed().unwrap().as_nanos();

    //Map
        previous_time = SystemTime::now();
        map.update();
        let after_map = previous_time.elapsed().unwrap().as_nanos();

    //Apparatus
        previous_time = SystemTime::now();
        apparatus::update(window_context, &mut map);

    //Debug
        if DEBUG{
            debug::debug(window_context,&mut map, start_time.elapsed().unwrap().as_nanos(), after_display,after_ui,after_map);
        } 
        let _ = window_context.canvas.present();

    // slow framerate to 60 fps
        if start_time.elapsed().unwrap() < window_context.camera.target_frame_time{
            let remaining = window_context.camera.target_frame_time - start_time.elapsed().unwrap();
            thread::sleep(remaining);
        }


    }
}


