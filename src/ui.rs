use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator},
    ttf::Font,
};

use crate::{display::WindowContext, fire, world::Map};

#[derive(Clone)]
pub struct Button {
    pub name: String, //name used to identify the button
    pub text: String, //Text displayed on button
    pub x: i32, //X and Y position relative to the x and y align
    pub y: i32, 
    pub x_align: f32, //Where to orient the coordinate 
    pub y_align: f32, 
        //Example:
        // x = 100; y = 100; x_align = 0.5; y_align = 0.0;
        // Multiply the alignments by the size of the window
        // x_align * 800 = 400; y_align * 600 = 0;
        // Add the X and Y to the alignment calculations
        // 800 + x = 900; 0 + y = 100;
        // The center of the button will be at 900 and 100;
    pub width: u32, //width of the button in pixels
    pub height: u32, //heigh of the button in pixels
    pub color: Color, //color of the button
}

pub fn to_scrn(pos: f32, scale: f32) -> i32 {
    return (pos * scale) as i32;
}

pub(crate) fn render(wc: &mut WindowContext) {
    let font: Font = wc
        .ttf_context
        .load_font("assets/fonts/Avenir Regular.ttf", 20)
        .unwrap();
    let window_width = wc.camera.window_width;
    let window_height = wc.camera.window_height;

    for button in wc.buttons.clone() {
        wc.canvas.set_draw_color(button.color);

        let middle_x = (button.x_align * window_width) as i32 + button.x;

        let middle_y = (button.y_align * window_height) as i32 + button.y;

        //background rectangle
        let _ = wc
            .canvas
            .fill_rect(Rect::new(middle_x - button.width as i32/2, middle_y - button.height as i32/2, button.width, button.height));

        // text rendering
        let text_surface = font.render(&button.text).blended(Color::BLACK).unwrap();
        let text_texture_creator: TextureCreator<_> = wc.canvas.texture_creator();
        let text_texture: Texture = text_texture_creator
            .create_texture_from_surface(&text_surface)
            .unwrap();

        let text_width = text_surface.width();
        let text_height = text_surface.height();

        // Draw the text
        wc.canvas
            .copy(
                &text_texture,
                None,
                Rect::new(
                    middle_x - (text_width / 2) as i32,
                    middle_y - (text_height / 2) as i32,
                    text_width,
                    text_height,
                ),
            )
            .unwrap();
    }
    wc.canvas.set_draw_color(Color::RGB(0, 0, 0));
}

pub fn mouse_click(x: i32, y: i32, wc: &mut WindowContext, map: &mut Map) {
    let window_width = wc.camera.window_width;
    let window_height = wc.camera.window_height;
    for button in wc.buttons.clone() {
        let middle_x = (button.x_align * window_width) as i32 + button.x;
        let middle_y = (button.y_align * window_height) as i32 + button.y;

        if x >= middle_x - (button.width / 2) as i32
            && x <= middle_x + (button.width / 2) as i32
            && y >= middle_y - (button.height / 2) as i32
            && y <= middle_y + (button.height / 2) as i32
        {
            println!("slgdkhj");
            ui_distributor(&button.name, wc, map);
        }
    }
    wc.im.left_click = false
}

fn remove_button(name: &str, wc: &mut WindowContext) {
    for i in 0..wc.buttons.len() {
        if wc.buttons[i].name == name {
            wc.buttons.remove(i);
        }
    }
}

fn ui_distributor(name: &str, wc: &mut WindowContext, map: &mut Map) {
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
        _ => {}
    }
}
