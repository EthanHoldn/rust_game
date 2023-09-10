use std::time::Duration;

use sdl2::rect::Rect;

use crate::{display::{WindowContext, display_text}, world::Map, fire};

pub(crate) fn debug(wc: &mut WindowContext, map: &mut Map, total_time: u128, after_display: u128, after_ui: u128, after_map: u128){

    let display_time = (after_display) as f32 / 1_000_000.0;
    let ui_time = (after_ui) as f32 / 1_000_000.0;
    let map_time = (after_map) as f32 / 1_000_000.0;
    
    //FPS display
    display_text(wc, 10,10, &format!("Display: {:.2}",display_time));
    display_text(wc, 10,30, &format!("UI: {:.2}",ui_time));
    display_text(wc, 10,50, &format!("Map: {:.2}",map_time));
    display_text(wc, 10,70, &format!("Active fire: {:.2}",map.active.iter().filter(|&n| *n == true).count()));

    //show map data
    let middle_x = (wc.camera.window_width/2.0) as i32;
    let middle_y = (wc.camera.window_height/2.0) as i32;

    let _ = wc.canvas.draw_rect(Rect::new(middle_x, middle_y, 10, 10));

    let y = ((((wc.camera.x_offset - (wc.camera.window_width / 2.0)) / wc.camera.zoom))*-1.0) as u32;
    let x = ((((wc.camera.y_offset - (wc.camera.window_height / 2.0)) / wc.camera.zoom))*-1.0) as u32;

    // Crashes here. index error. No bounds checking for bottom or right of screen.
    let active_index = fire::index(map.size, x, y);
    if active_index.is_some() {
        let active:bool = map.active[active_index.unwrap()];
        display_text(wc, middle_x, middle_y, active.to_string().as_str());
    }

    let fire_index = fire::index(map.size, x, y);
    if fire_index.is_some() {
        let fire: u8 = map.fire[fire_index.unwrap()];
        display_text(wc, middle_x, middle_y+20, fire.to_string().as_str());
    }



}
