use sdl2::{rect::Rect, pixels::Color, ttf::Font, render::{TextureCreator, Texture}};

use crate::display::WindowContext;

#[derive(Clone)]
pub(crate) struct Button{
    pub name: String,
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: Color,

}

impl Button {
    pub fn render(wc: &mut WindowContext) {
        let font: Font = wc.ttf_context.load_font("assets/fonts/FiraSans-Bold.ttf", 50 ).unwrap();

        for button in wc.buttons.clone(){
            wc.canvas.set_draw_color(button.color);

            //background rectangle
            let _ = wc.canvas.fill_rect(Rect::new(button.x, button.y, button.width, button.height));

            // text rendering
            let text_surface = font.render(&button.text).blended(Color::BLACK).unwrap();
            let text_texture_creator: TextureCreator<_> = wc.canvas.texture_creator();
            let text_texture: Texture = text_texture_creator.create_texture_from_surface(&text_surface).unwrap();
    
            let text_width = text_surface.width();
            let text_height = text_surface.height();
    
            let middle_x = (button.width-text_width)/2;
            let middle_y = (button.height-text_height)/2;

            // Draw the text
            wc.canvas.copy(&text_texture, None, Rect::new(button.x+middle_x as i32, button.y+middle_y as i32, text_width, text_height))
                .unwrap();

        }
        wc.canvas.set_draw_color(Color::RGB(0, 0, 0));
        
    }
    // every time the display input function detects a mouse click it calls this function
    pub fn mouse_click(x: i32, y: i32, wc: &mut WindowContext) {
        for button in &mut wc.buttons {
            if x >= button.x && y >= button.y && x <= button.x + button.width as i32 && y <= button.y + button.height as i32 {
                Self::ui_distributor(&button.name, wc);
            }
        }
    }

    pub fn ui_distributor(name: &str, wc: &mut WindowContext){
        match name {
            "exit" => {
                std::process::exit;
            }
            _ => {
                
            }
        }
    }
}
