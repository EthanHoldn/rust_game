extern crate sdl2;
use crate::world;

use std::time::{Duration, Instant};
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub(crate) fn init() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("game", 800, 600).allow_highdpi().resizable()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
 
    run(&mut canvas, &mut event_pump);
}


fn run(canvas: &mut Canvas<Window>, event_pump: &mut EventPump){
    let target_fps = 60;
    let target_frame_time = Duration::from_secs(1) / target_fps;

    //map data
    let mut map = world::Map{
        size: 750,
        image: Vec::<u8>::new(),
        camera_x_offset: 0,
        camera_y_offset: 0,
        camera_zoom: 0.0,
    };

    map.generate_image();

    let mut i = 0;

    //used to generate textures from a Vec<u8>
    let texture_creator = canvas.texture_creator();

    //map image texture
    let mut map_texture = texture_creator
    .create_texture_streaming(PixelFormatEnum::RGBA32, map.size, map.size)
    .unwrap();

    map_texture.update(None, &map.image, map.size as usize * 4).unwrap();
    
    //frame rate calculation
    let mut previous_frame_start = Instant::now();

    //main  window rendering loop
    //all window related operations need to be done in here
    'running: loop {
        
        i = (i + 1) % 255;
        canvas.clear();

        //get user inputs
        if inputs(event_pump, &mut map){break 'running}


        texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA32, map.size, map.size)
        .unwrap();
        map_texture.update(None, &map.image, map.size as usize * 4).unwrap();

        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.copy(&map_texture, None, Rect::new(map.camera_x_offset, map.camera_y_offset, 2000, 2000)).unwrap();

        canvas.present();
        let elapsed =  previous_frame_start.elapsed();
        println!("{}",(elapsed.as_nanos() as f64 )/1_000_000.0);
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        }
        previous_frame_start = Instant::now();
    }
}


fn inputs(event_pump: &mut EventPump, map: &mut world::Map) -> bool{
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return true;
                
            },
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                map.camera_x_offset +=1;
            },
            _ => {}
        }
    }
    return false;
}