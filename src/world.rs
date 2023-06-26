use rand::prelude::*;
use noise::{NoiseFn, Perlin};
use sdl2::{pixels::PixelFormatEnum, render::{Canvas, Texture}, video::Window};

#[derive(Copy, Clone)]
pub enum tile_type {
    invalid,
    water,
    grass,
    brush,
    tree,
    mountain
}

// Map struct 
#[derive(Clone)]
pub(crate) struct Map {
    pub size: u32,              // Size of pixel array
    pub terrain: Vec<tile_type>,
    pub plain_thresh: f32,
    pub mountain_thresh: f32 
}

// Class methods for Map
impl Map {
    //TODO: implement map data layer generations
    pub fn generate_layers(&mut self) {
        // Create noise instance, pixel array, and set scale
        let perlin: Perlin = Perlin::new(1);
        
        // scale factor for perlin noise 
        // bigger number means the changes in terrain are more spread out
        let scale:f64 = 30.0;

        // For each pixel
        for i in 0 .. (self.size * self.size){
            // Create perlin noise based on position
            let mut rng: ThreadRng = rand::thread_rng();
            let x: u32 = i % self.size;
            let y: u32 = i / self.size;
            let mut pln: f64 = perlin.get([x as f64 / scale, y as f64 / scale, 0.0]) + 1.0;
            let pln_stable: f64 = pln;
            pln = pln + rng.gen::<f64>()*0.1;

            // Set color based on noise value
            if pln_stable >= 0.0 && pln_stable < 0.2 {//water
                self.terrain.push(tile_type::water);
            } else if pln >= 0.2 && pln < 0.6 {//trees
                self.terrain.push(tile_type::tree);
            } else if pln >= 0.6 && pln < 0.8 {//brush
                self.terrain.push(tile_type::brush);
            } else if pln >= 0.8 {//grass
                self.terrain.push(tile_type::grass);
            } else {//MAP_ERROR
                self.terrain.push(tile_type::invalid);
            }

        }
    }

    // Generate the map image
    pub fn create_image(&mut self) -> Vec<u8> {
        let mut image: Vec<u8> = Vec::<u8>::new();         // Array of pixels
        let mut rng: ThreadRng = rand::thread_rng();

        // Set color based on noise value
        for i in 0..self.terrain.len(){
            println!("{i}");
            let tile = self.terrain.get(i);

            // Create pixel color values 
            let r: f32;
            let g: f32;
            let b: f32;

            match tile {
                Some(tile_type::water) => {
                    r = 42.0;
                    g = 147.0 + rng.gen::<f32>() * 20.0;
                    b = 173.0;
                } 
                Some(tile_type::tree) => {//trees
                    r = 83.0;
                    g = 138.0 + rng.gen::<f32>() * 20.0;
                    b = 28.0;
                } 
                Some(tile_type::brush) => {//brush
                    r = 132.0;
                    g = 181.0 + rng.gen::<f32>() * 20.0;
                    b = 83.0;
                }
                Some(tile_type::grass) => {//grass
                    r = 167.0 + rng.gen::<f32>() * 20.0;
                    g = 199.0;
                    b = 127.0;
                } 
                _ => {//MAP_ERROR
                    r = 255.0;
                    g = 100.0;
                    b = 100.0;
                }
            }

            // Push to list as flattened array
            image.push((0.003921568627451*(r*r)) as u8);
            image.push((0.003921568627451*(g*g)) as u8);
            image.push((0.003921568627451*(b*b)) as u8);
            image.push(255);//a

        }

        return  image;

    }
}