use std::{thread, time};

use sdl2::{event::Event, mouse::MouseButton, keyboard::Keycode};

use crate::{display::WindowContext, world, ui::mouse_click};

pub(crate) fn inputs(wc: &mut WindowContext, map: &mut world::Map,) -> bool {
    //updates the array of all the keys that are currently held down
    for event in wc.event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return true,
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                // Key is pressed
                wc.im.key_states.push(key);
                wc.im.key_states.dedup();
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                // Key is released
                wc.im.key_states.retain(|x| *x != key);
            }
            Event::MouseButtonUp {mouse_btn, x, y, .. } => {
                println!("LMB UP {},{}", x, y);
                if MouseButton::Left == mouse_btn {
                    wc.im.mouse_x = x;
                    wc.im.mouse_y = y
                }
            }
            _ => {}
        }
    }
    
    
    if wc.im.key_states.contains(&Keycode::Tab){ // ` 
        thread::sleep(time::Duration::from_millis(1000));
    }

    //camera movement
    if wc.im.key_states.contains(&Keycode::W) { // W  up
        wc.camera.y_offset += wc.camera.movement_speed
    }
    if wc.im.key_states.contains(&Keycode::A) { // A  up
        wc.camera.x_offset += wc.camera.movement_speed
    }
    if wc.im.key_states.contains(&Keycode::S) { // S  up
        wc.camera.y_offset -= wc.camera.movement_speed
    }
    if wc.im.key_states.contains(&Keycode::D) { // D  up
        wc.camera.x_offset -= wc.camera.movement_speed
    }
    //camera zoom
    
    if wc.im.key_states.contains(&Keycode::E) { // E  zoom in
        let relative_zoom_speed = wc.camera.zoom_speed * wc.camera.zoom;
        wc.camera.zoom += relative_zoom_speed;
        wc.camera.x_offset += (relative_zoom_speed * map.size as f32) * (((wc.camera.x_offset - (wc.camera.window_width / 2.0)) / wc.camera.zoom) / (map.size as f32));
        wc.camera.y_offset += (relative_zoom_speed * map.size as f32) * (((wc.camera.y_offset - (wc.camera.window_height / 2.0)) / wc.camera.zoom) / (map.size as f32));
    } 
    if wc.im.key_states.contains(&Keycode::Q) { // Q  zoom out
        

        let relative_zoom_speed = wc.camera.zoom_speed * wc.camera.zoom;
        wc.camera.zoom -= relative_zoom_speed;
        wc.camera.x_offset -= (relative_zoom_speed*map.size as f32) * (((wc.camera.x_offset-(wc.camera.window_width/2.0))/wc.camera.zoom)/(map.size as f32));
        wc.camera.y_offset -= (relative_zoom_speed*map.size as f32) * (((wc.camera.y_offset-(wc.camera.window_height/2.0))/wc.camera.zoom)/(map.size as f32));



    }

    // Quit on esc or ctrl
    if wc.im.key_states.contains(&Keycode::Escape){return  true;}

    mouse_click(wc.im.mouse_x, wc.im.mouse_y, wc, map);

    return false;
}