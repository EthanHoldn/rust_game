use std::{thread, time};

use sdl2::{event::Event, mouse::MouseButton, keyboard::Keycode};

use crate::{display::WindowContext, world, ui::{mouse_click, UIScreens}};

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

    if let UIScreens::World = wc.screen{
        //camera movement
        if wc.im.key_states.contains(&Keycode::W){ // W  up
            wc.camera.y_offset += wc.camera.movement_speed
        }
        if wc.im.key_states.contains(&Keycode::A){ // A  up
            wc.camera.x_offset += wc.camera.movement_speed
        }
        if wc.im.key_states.contains(&Keycode::S) { // S  up
            wc.camera.y_offset -= wc.camera.movement_speed
        }
        if wc.im.key_states.contains(&Keycode::D) { // D  up
            wc.camera.x_offset -= wc.camera.movement_speed
        }

        //camera zoom
        
        if wc.im.key_states.contains(&Keycode::E) && wc.camera.zoom < 25.0{ // E  zoom in
            println!("{}", wc.camera.zoom);
            let relative_zoom_speed = wc.camera.zoom_speed * wc.camera.zoom;
            wc.camera.zoom += relative_zoom_speed;
            wc.camera.x_offset += (relative_zoom_speed * map.size as f32) * (((wc.camera.x_offset - (wc.camera.window_width / 2.0)) / wc.camera.zoom) / (map.size as f32));
            wc.camera.y_offset += (relative_zoom_speed * map.size as f32) * (((wc.camera.y_offset - (wc.camera.window_height / 2.0)) / wc.camera.zoom) / (map.size as f32));
        } 
        //let max_zoom_out = 
        if wc.im.key_states.contains(&Keycode::Q) && wc.camera.window_width < map.size as f32 * wc.camera.zoom{ // Q  zoom out
            println!("{}", wc.camera.zoom);
            let relative_zoom_speed = wc.camera.zoom_speed * wc.camera.zoom;
            wc.camera.zoom -= relative_zoom_speed;
            wc.camera.x_offset -= (relative_zoom_speed*map.size as f32) * (((wc.camera.x_offset-(wc.camera.window_width/2.0))/wc.camera.zoom)/(map.size as f32));
            wc.camera.y_offset -= (relative_zoom_speed*map.size as f32) * (((wc.camera.y_offset-(wc.camera.window_height/2.0))/wc.camera.zoom)/(map.size as f32));
        }
    }
    if wc.camera.window_width > map.size as f32 * wc.camera.zoom{
        wc.camera.zoom = wc.camera.window_width/map.size as f32
    }

    if wc.camera.x_offset > 0.0{
        wc.camera.x_offset = 0.0;
    }
    if wc.camera.y_offset > 0.0{
        wc.camera.y_offset = 0.0;
    }

    if wc.camera.y_offset +(map.size as f32*wc.camera.zoom) < wc.camera.window_height {
        wc.camera.y_offset = wc.camera.window_height - (map.size as f32*wc.camera.zoom);
    }
    if wc.camera.x_offset +(map.size as f32*wc.camera.zoom) < wc.camera.window_width {
        wc.camera.x_offset = wc.camera.window_width - (map.size as f32*wc.camera.zoom);
    }

    // Quit on esc or ctrl
    if wc.im.key_states.contains(&Keycode::Escape){return  true;}

    mouse_click(wc.im.mouse_x*2, wc.im.mouse_y*2, wc, map);

    return false;
}