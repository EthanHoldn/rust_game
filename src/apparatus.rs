use sdl2::{image::LoadTexture, rect::Rect};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{display::WindowContext, world::Map};

#[derive(Clone)]
pub struct Apparatus{
    pub x:f32,
    pub y:f32,
    pub angel: f32,
    pub name: ApparatusType,

}

#[derive(Clone)]
pub enum ApparatusType {
    Bell205,
    Bulldozer,
}

pub fn update(wc: &mut WindowContext, mut map: &mut Map) {
    render(wc, map)
}
fn simulate(wc: &mut WindowContext, mut map: &mut Map){

}

fn render(wc: &mut WindowContext, mut map: &mut Map){
    let texture_creator = wc.canvas.texture_creator();
    let bell205 = match texture_creator.load_texture("assets/Bell 205.png"){
        Ok(texture) => texture,
        Err(error) => {
            eprintln!("Error loading texture: {}", error);
            return;
        }};

    for a in &map.apparatuses{
        match a.name {
            ApparatusType::Bell205 => {
                let _ = wc.canvas.copy_ex(
                    &bell205,
                    None,
                    Some(Rect::new(((a.x*wc.camera.zoom) + wc.camera.x_offset) as i32 -60, ((a.y*wc.camera.zoom) + wc.camera.y_offset) as i32 -60, (40.0*wc.ui_scale*1.5)as u32, (60.0*wc.ui_scale*1.5)as u32)),
                    a.angel as f64,   
                    None,
                    false,
                    false,
                );
            },
            ApparatusType::Bulldozer => {
                
            },
        }
    }
}