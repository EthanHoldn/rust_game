use sdl2::{rect::Rect, pixels::Color, ttf::Font, render::{TextureCreator, Texture}};

use crate::{display::WindowContext, world::Map, fire};

#[derive(Clone)]
pub struct Button{
    pub name: String,
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width:  f32,
    pub height: f32,
    pub color: Color,

}

pub fn to_scrn(pos : f32, scale : f32) -> i32 {
    return (pos * scale) as i32;
}

pub(crate) fn render(wc: &mut WindowContext) {
    let font: Font = wc.ttf_context.load_font("assets/fonts/FiraSans-Bold.ttf", 50 ).unwrap();
    let wid = wc.camera.window_width;
    let hei = wc.camera.window_height;

    for button in wc.buttons.clone(){
        wc.canvas.set_draw_color(button.color);
        //background rectangle
        let _ = wc.canvas.fill_rect(Rect::new(to_scrn(button.x, wid), to_scrn(button.y, hei),
                                          to_scrn(button.width, wid) as u32, to_scrn(button.height, hei) as u32));
    
        // text rendering
        let text_surface = font.render(&button.text).blended(Color::BLACK).unwrap();
        let text_texture_creator: TextureCreator<_> = wc.canvas.texture_creator();
        let text_texture: Texture = text_texture_creator.create_texture_from_surface(&text_surface).unwrap();

        let text_width = text_surface.width();
        let text_height = text_surface.height();

        let middle_x = (to_scrn(button.width, wid) as u32 - text_width)/2;
        let middle_y = (to_scrn(button.height, hei) as u32 - text_height)/2;
        println!("--{},{}--",to_scrn(button.x, wid), to_scrn(button.y, hei));
        // Draw the text
        wc.canvas.copy(&text_texture, None, Rect::new(to_scrn(button.x, wid) + middle_x as i32, to_scrn(button.y, hei) + middle_y as i32, text_width, text_height))
            .unwrap();
    }
    wc.canvas.set_draw_color(Color::RGB(0, 0, 0));
        
}

pub fn mouse_click(x: i32, y: i32, wc: &mut WindowContext, map: &mut Map) {
    for button in wc.buttons.clone() {
        //println!("{},{} {},{}-{},{}", x,y,to_scrn(button.x, wc.camera.window_width),to_scrn(button.y, wc.camera.window_height),to_scrn(button.x + button.width, wc.camera.window_width) as i32,to_scrn(button.y + button.height, wc.camera.window_height) as i32);
        if x >= to_scrn(button.x, wc.camera.window_width) as i32 && y >= to_scrn(button.y, wc.camera.window_height) 
        && x <= to_scrn(button.x + button.width, wc.camera.window_width) as i32 && y <= to_scrn(button.y + button.height, wc.camera.window_height) as i32 {
            ui_distributor(&button.name, wc, map);
        }
    }
    wc.im.left_click = false
}

fn remove_button(name: &str, wc: &mut WindowContext){
    for i in 0..wc.buttons.len(){
        if wc.buttons[i].name == name{
            wc.buttons.remove(i);
        }
    }
}

fn ui_distributor(name: &str, wc: &mut WindowContext, map: &mut Map){
    println!("{}", name);

    match name {
        "exit" => {
            std::process::exit(1);
        }
        "new world" => {
            map.size = 500;
            //generate map data
            map.generate_layers();
            map.create_image();
            //start fire
            fire::spawn(map);
            remove_button("new world", wc)
        }
        _ => {
            
        }
    }
}
