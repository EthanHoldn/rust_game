use std::time::Duration;

use sdl2::rect::Rect;

use crate::{display::{WindowContext, display_text}, world::Map, fire};

pub(crate) fn debug(wc: &mut WindowContext, elapsed: Duration, map: &mut Map){

    //calculate framerate
    let elapsed_sec = elapsed.as_secs_f64();
    let remaining_sec = wc.camera.target_frame_time.as_secs_f64()-elapsed_sec;
    let actual_framerate = 1.0/elapsed_sec;
    //FPS display
    display_text(wc, 10,10, &format!("{:.2}",elapsed_sec*1000.0));
    display_text(wc, 10,110, &format!("{:.2}",remaining_sec*1000.0));
    display_text(wc, 10,210, &format!("{:.2}",actual_framerate));

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

    map.update_pixel(x, y, 0, 255, 0, 255);


}
