use rand::prelude::*;
use noise::{NoiseFn, Perlin};
use sdl2::{pixels::PixelFormatEnum, render::{Canvas, Texture}, video::Window};

// Map struct 
pub(crate) struct Map {
    pub image: Vec<u8>,         // Array of pixels
    pub size: u32,              // Size of pixel array
    pub camera_x_offset: i32,   // Camera pos and zoom
    pub camera_y_offset: i32,
    pub camera_zoom: f32,       
}

// Generate the map
pub fn generate(s: u32) -> Vec<u8>{
    // Create noise instance, pixel array, and set scale
    let perlin = Perlin::new(1);
    let mut pixels: Vec<u8> = Vec::new();
    
    // Unnecessary?
    let scale:f64 = 30.0;

    // For each pixel
    for i in 0..(s*s){
        // Create perlin noise based on position
        let mut rng = rand::thread_rng();
        let x = i%s;
        let y = i/s;
        let mut pln = perlin.get([x as f64/scale, y as f64/scale, 0.0])+1.0;
        let pln_stable = pln;
        pln = pln + rng.gen::<f64>()*0.1;

        // Create pixel color values 
        let r: f32;
        let g: f32;
        let b: f32;

        // Set color based on noise value
        if pln_stable > 0.97 && pln_stable < 1.03 {//water
            r = 42.0;
            g = 147.0 + rng.gen::<f32>()*20.0;
            b = 173.0;
        } else if pln > 0.8 && pln < 1.2{//trees
            r = 83.0;
            g = 138.0 + rng.gen::<f32>()*20.0;
            b = 28.0;
        } else if pln > 0.6 && pln < 1.4{//brush
            r = 132.0;
            g = 181.0 + rng.gen::<f32>()*20.0;
            b = 83.0;
        } else if pln > 0.0 && pln < 2.0{//grass
            r = 167.0 + rng.gen::<f32>()*20.0;
            g = 199.0 ;
            b = 127.0;
        } else {//MAP_ERROR
            r = 255.0;
            g = 100.0;
            b = 100.0;
        }

        // Push to list as flattened array
        pixels.push((0.003921568627451*(r*r)) as u8);
        pixels.push((0.003921568627451*(g*g)) as u8);
        pixels.push((0.003921568627451*(b*b)) as u8);
        pixels.push(255);//a

    }


    return  pixels;

}
