use sdl2::{
    pixels::Color,
    rect::{Rect, Point},
    render::{Texture, TextureCreator},
    ttf::Font,
};

use crate::{display::WindowContext, fire, world::Map};
const BORDER: f32 = 4.0;
pub enum UIScreens {
    MainMenu,
    World,
}

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

#[derive(Clone)]
pub struct UIBox {
    pub name: String, //name used to identify the box
    pub x: f32, //X and Y position relative to the window
    pub y: f32, 
    pub width: u32, //width of the button in pixels
    pub height: u32, //heigh of the button in pixels
    pub color: Color, //color of the button
}

pub fn to_scrn(pos: f32, scale: f32) -> i32 {
    return (pos * scale) as i32;
}

pub(crate) fn render(wc: &mut WindowContext) {
    if let UIScreens::World = wc.screen {
        in_world_ui_render(wc);
    }

    ui_box_render(wc);

    button_render(wc);


    

}

fn in_world_ui_render(wc: &mut WindowContext){
    wc.canvas.set_draw_color(Color::RGB(80, 80, 80));
    let hight = (80.0 * wc.ui_scale) as u32;
    let _ = wc.canvas.fill_rect(Rect::new(0, wc.camera.window_height as i32 - hight as i32 , wc.camera.window_width as u32, hight));
    wc.canvas.set_draw_color(Color::RGB(90, 90, 90));
    let border = (BORDER*wc.ui_scale) as i32;
    let _ = wc.canvas.fill_rect(Rect::new(0, wc.camera.window_height as i32 - hight as i32 - border, wc.camera.window_width as u32, border as u32));

}

fn ui_box_render(wc: &mut WindowContext){
    for ui_box in wc.ui_box.clone() {
        let window_width = wc.camera.window_width;
        let window_height = wc.camera.window_height;
        let middle_x: i32 = (ui_box.x * window_width) as i32;
        let middle_y: i32 = (ui_box.y * window_height) as i32;
        let top_left_x = middle_x - (ui_box.width as f32 * wc.ui_scale) as i32/2;
        let top_left_y = middle_y - (ui_box.height as f32 * wc.ui_scale) as i32/2;
        let scaled_width = (ui_box.width as f32 * wc.ui_scale) as u32;
        let scaled_height = (ui_box.height as f32 * wc.ui_scale) as u32;
        let border = (BORDER * wc.ui_scale) as u32;
        let light = Color::RGB(ui_box.color.r +10, ui_box.color.g+10, ui_box.color.b+10);
        let dark =Color::RGB(ui_box.color.r-10, ui_box.color.g-10, ui_box.color.b-10);


        // top left and bottom right
        for x in 0..border as i32{
            for y in 0..border as i32{
                if  x - y > 0{
                    wc.canvas.set_draw_color(light)
                } else if x - y < 0 {
                    wc.canvas.set_draw_color(dark)
                } else if x == y {
                    wc.canvas.set_draw_color(ui_box.color);
                }
                let _ = wc.canvas.draw_point(Point::new(top_left_x + x as i32, top_left_y + y as i32));
                let _ = wc.canvas.draw_point(Point::new(top_left_x + (scaled_width - border)as i32 + x as i32, top_left_y + (scaled_height - border) as i32 + y as i32));
            }
        }

        //sides
        wc.canvas.set_draw_color(light);
        let _ = wc.canvas.fill_rect(Rect::new(top_left_x+border as i32,top_left_y,scaled_width - border,border));
        
        wc.canvas.set_draw_color(dark);
        let _ = wc.canvas.fill_rect(Rect::new(top_left_x,top_left_y+border as i32,border,scaled_height - border*2));

        wc.canvas.set_draw_color(light);
        let _ = wc.canvas.fill_rect(Rect::new(top_left_x+(scaled_width - border) as i32 ,top_left_y+border as i32,border,scaled_height - border*2));

        wc.canvas.set_draw_color(dark);
        let _ = wc.canvas.fill_rect(Rect::new(top_left_x ,top_left_y +(scaled_height - border) as i32,scaled_width - border,border));

        //middle
        wc.canvas.set_draw_color(ui_box.color);
        let _ = wc.canvas.fill_rect(Rect::new(top_left_x+border as i32,top_left_y+border as i32,scaled_width - border*2,scaled_height - border*2));

    }
}

fn button_render(wc: &mut WindowContext){
    let font: Font = wc.ttf_context.load_font("assets/fonts/Avenir Regular.ttf", (20.0 * wc.ui_scale) as u16).unwrap();
    let window_width = wc.camera.window_width;
    let window_height = wc.camera.window_height;

    for button in wc.buttons.clone() {

        let middle_x: i32 = (button.x_align * window_width) as i32 + (button.x as f32 * wc.ui_scale) as i32;
        let middle_y: i32 = (button.y_align * window_height) as i32 + (button.y as f32 * wc.ui_scale) as i32;

        let top_left_x = middle_x - (button.width as f32 * wc.ui_scale) as i32/2;
        let top_left_y = middle_y - (button.height as f32 * wc.ui_scale) as i32/2;
        let scaled_width = (button.width as f32 * wc.ui_scale) as u32;
        let scaled_height = (button.height as f32 * wc.ui_scale) as u32;
        let border = (BORDER * wc.ui_scale) as u32;
        let light = Color::RGB(button.color.r +10, button.color.g+10, button.color.b+10);
        let dark = Color::RGB(button.color.r-10, button.color.g-10, button.color.b-10);

        for x in 0..border as i32{
            for y in 0..border as i32{
                if  x - y > 0{
                    wc.canvas.set_draw_color(light)
                } else if x - y < 0 {
                    wc.canvas.set_draw_color(dark)
                } else if x == y {
                    wc.canvas.set_draw_color(button.color);
                }
                let _ = wc.canvas.draw_point(Point::new(top_left_x + x as i32, top_left_y + y as i32));
                let _ = wc.canvas.draw_point(Point::new(top_left_x + (scaled_width - border)as i32 + x as i32, top_left_y + (scaled_height - border) as i32 + y as i32));
            }
        }

        //sides
        wc.canvas.set_draw_color(light);
        let _ = wc.canvas.fill_rect(Rect::new(top_left_x+border as i32,top_left_y,scaled_width - border,border));
        
        wc.canvas.set_draw_color(dark);
        let _ = wc.canvas.fill_rect(Rect::new(top_left_x,top_left_y+border as i32,border,scaled_height - border*2));

        wc.canvas.set_draw_color(light);
        let _ = wc.canvas.fill_rect(Rect::new(top_left_x+(scaled_width - border) as i32 ,top_left_y+border as i32,border,scaled_height - border*2));

        wc.canvas.set_draw_color(dark);
        let _ = wc.canvas.fill_rect(Rect::new(top_left_x ,top_left_y +(scaled_height - border) as i32,scaled_width - border,border));

        //middle
        wc.canvas.set_draw_color(button.color);
        let _ = wc.canvas.fill_rect(Rect::new(top_left_x+border as i32,top_left_y+border as i32,scaled_width - border*2,scaled_height - border*2));

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
}

pub fn mouse_click(x: i32, y: i32, wc: &mut WindowContext, map: &mut Map) {
    let window_width = wc.camera.window_width;
    let window_height = wc.camera.window_height;
    for button in wc.buttons.clone() {
        let middle_x: i32 = (button.x_align * window_width) as i32 + (button.x as f32 * wc.ui_scale) as i32;
        let middle_y: i32 = (button.y_align * window_height) as i32 + (button.y as f32 * wc.ui_scale) as i32;

        if x >= middle_x - ((button.width as f32 * wc.ui_scale) / 2.0) as i32
            && x <= middle_x + ((button.width as f32 * wc.ui_scale) / 2.0) as i32
            && y >= middle_y - ((button.height as f32 * wc.ui_scale) / 2.0) as i32
            && y <= middle_y + ((button.height as f32 * wc.ui_scale) / 2.0) as i32
        {
            ui_distributor(&button.name, wc, map);
        }
    }
    wc.im.left_click = false
}

fn remove_button(name: &str, wc: &mut WindowContext) {
    for i in 0..wc.buttons.len() {
        if wc.buttons[i].name == name {
            wc.buttons.remove(i);
            break;
        }
    }
}
fn remove_box(name: &str, wc: &mut WindowContext) {
    for i in 0..wc.ui_box.len() {
        if wc.ui_box[i].name == name {
            wc.ui_box.remove(i);
            break;
        }
    }
}

fn ui_distributor(name: &str, wc: &mut WindowContext, map: &mut Map) {

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
            wc.camera.zoom = 5.0;
            wc.screen = UIScreens::World;
            remove_button("new world", wc);
            remove_button("load world", wc);
            remove_button("settings", wc);
            remove_button("exit", wc);
            remove_box("main_menu", wc);


        }
        _ => {}
    }
}
