extern crate sdl2;
use crate::ui::{Button, render};
use crate::world::{self, TileType, Map};
//use crate::debug::debug;
use crate::input_manager::inputs;

use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::{Duration, Instant};


pub struct Camera {
    pub x_offset: f32, // Camera pos and zoom
    pub y_offset: f32,
    pub zoom: f32,
    pub movement_speed: f32,
    pub zoom_speed: f32,
    pub window_width: f32,
    pub window_height: f32,
    pub target_fps: u32,
    pub target_frame_time: Duration,
}
pub struct WindowContext {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub ttf_context: Sdl2TtfContext,
    pub camera: Camera,
    pub im: IM,
    pub buttons: Vec<Button>,
    pub scale: i32,
}
pub struct IM {
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub left_click: bool,

    pub key_states: Vec<Keycode>,
    pub clicks: Vec<(u32,u32)>,
}

pub(crate) fn init() -> (WindowContext, Map){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem
        .window("game", 800, 600)
        .resizable()
        .build()
        .unwrap();
    let mut wc = WindowContext{
        canvas: window.into_canvas().build().unwrap(),
        event_pump: sdl_context.event_pump().unwrap(),
        ttf_context: sdl2::ttf::init().unwrap(),
        camera: Camera {
            x_offset: 0.0,
            y_offset: 0.0,
            zoom: 2.0,
            zoom_speed: 0.02,
            movement_speed: 5.0,
            window_width: 1600.0,
            window_height: 1200.0,
            target_fps: 60,
            target_frame_time:  Duration::from_secs(1) / 60,
        },
        im: IM { 
            mouse_x: 0,
            mouse_y: 0, 
            left_click: false,
            key_states: Vec::<Keycode>::new(),
            clicks: Vec::<(u32,u32)>::new(), 
        },
        buttons: Vec::<Button>::new(),
        scale: 4,
        
    };
    let map = world::Map {
        size: 0,
        terrain: Vec::<TileType>::new(),
        image: Vec::<u8>::new(),
        marsh_thresh: 0.03,
        plain_thresh: 0.2,
        grass_thresh: 0.5,
        fire: Vec::<u8>::new(),
        active: Vec::<bool>::new(),
        simulating: false
    };

    //main menu buttons

    wc.buttons.push(Button { name: "exit".to_owned(), text: "Exit".to_owned(), x: 0, y: 0, x_align: 0.25, y_align: 0.25, width: 200, height: 50, color: Color::RGB(100, 100, 100) });
    wc.buttons.push(Button { name: "new world".to_owned(), text: "New World".to_owned(), x: 0, y: 100, x_align: 0.25, y_align: 0.25, width: 200, height: 50, color: Color::RGB(100, 100, 100) });

    return (wc, map);
}

pub fn run(wc: &mut WindowContext, mut map: &mut Map) -> bool {
    
    //used to generate textures from a Vec<u8>
    let texture_creator = wc.canvas.texture_creator();
    // Scale for correct window size
    wc.camera.window_width = wc.canvas.window().size().0 as f32;
    wc.camera.window_height = wc.canvas.window().size().1 as f32;
    //get user inputs
    if inputs(wc, &mut map) {
        return false;
    }
    //if map is initialized
    if map.size != 0{
        let mut map_texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA32, map.size, map.size)
        .unwrap();
        //display map
        texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32, map.size, map.size).unwrap();
        map_texture.update(None, &map.image, map.size as usize * 4).unwrap();
        wc.canvas.copy( &map_texture, None, Rect::new( wc.camera.x_offset as i32, wc.camera.y_offset as i32, (wc.camera.zoom * map.size as f32) as u32, (wc.camera.zoom * map.size as f32) as u32,),).unwrap();
    }
    
    
    //show debug info 
    //if DEBUG {debug(wc, None, &mut map);}
    
    return true;

}

pub(crate) fn display_text(wc: &mut WindowContext, x:i32, y: i32, text: &str){
    let font: Font = wc.ttf_context.load_font("assets/fonts/Avenir Regular.ttf", 20 ).unwrap();

    let text_surface = font
        .render(text)
        .blended(Color::BLACK)
        .unwrap();

        let text_texture_creator: TextureCreator<_> = wc.canvas.texture_creator();
        let text_texture: Texture = text_texture_creator
            .create_texture_from_surface(&text_surface)
            .unwrap();

        // Calculate the position of the text
        let text_width = text_surface.width();
        let text_height = text_surface.height();

        // Draw the text
        wc.canvas.copy(&text_texture, None, Rect::new(x, y, text_width, text_height))
            .unwrap();
}

