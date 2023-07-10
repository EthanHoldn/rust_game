pub mod display;
pub mod world;
pub mod fire;
pub mod ui;
pub mod common;
pub mod debug;
pub mod input_manager;



pub fn main() {
    //initialize the display
    let init = display::init();
    let mut window_context = init.0;
    let map = init.1;
    display::run(&mut window_context, map);
}
