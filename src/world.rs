use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use sdl2::{
    pixels::PixelFormatEnum,
    render::{Canvas, Texture},
    video::Window,
};

pub enum TileType {
    Invalid,
    Water,
    Grass,
    Brush,
    Tree,
    Mountain,
}

// Map struct
pub(crate) struct Map {
    pub size: u32, // Size of pixel array
    pub terrain: Vec<TileType>,
    pub image: Vec<u8>,
    pub plain_thresh: f32,
    pub mountain_thresh: f32,
}

// Class methods for Map
impl Map {
    //TODO: implement map data layer generations
    pub fn generate_layers(&mut self) {
        // Create noise instance, pixel array, and set scale
        let perlin: Perlin = Perlin::new(1);

        // scale factor for perlin noise
        // bigger number means the changes in terrain are more spread out
        let scale: f64 = 30.0;

        // For each pixel
        for i in 0..(self.size * self.size) {
            // Create perlin noise based on position
            let mut rng: ThreadRng = rand::thread_rng();
            let x: u32 = i % self.size;
            let y: u32 = i / self.size;
            let mut pln: f64 = perlin.get([x as f64 / scale, y as f64 / scale, 0.0]) + 1.0;
            let pln_stable: f64 = pln;
            pln = pln + rng.gen::<f64>() * 0.1;
            //pln = pln + rng.gen::<f64>()*0.1;

            // Set color based on noise value
            if pln_stable >= 0.0 && pln_stable < 0.2 {
                //water
                self.terrain.push(TileType::Water);
            } else if pln >= 0.2 && pln < 0.25 {// marsh
                self.terrain.push(TileType::Brush);
            } else if pln >= 0.25 && pln < 1.5 {// grass
                self.terrain.push(TileType::Grass);
            } else if pln >= 1.6 {// grass
                self.terrain.push(TileType::Mountain);
            } else {//MAP_ERROR
                self.terrain.push(TileType::Invalid);
            }
        }
    }

    // Generate the map image
    pub fn create_image(&mut self) {
        let mut rng: ThreadRng = rand::thread_rng();
        self.image.clear();
        // Set color based on noise value

        for tile in &self.terrain {
            //let tile = self.terrain.get(i);
            // Create pixel color values
            let r: f32;
            let g: f32;
            let b: f32;

            match tile {
                TileType::Water => {
                    r = 42.0;
                    g = 147.0 + rng.gen::<f32>() * 20.0;
                    b = 173.0;
                }
                TileType::Tree => {
                    //trees
                    r = 83.0;
                    g = 138.0 + rng.gen::<f32>() * 20.0;
                    b = 28.0;
                }
                TileType::Brush => {
                    //brush
                    r = 132.0;
                    g = 181.0 + rng.gen::<f32>() * 20.0;
                    b = 83.0;
                }
                TileType::Grass => {
                    //grass
                    r = 167.0 + rng.gen::<f32>() * 20.0;
                    g = 199.0;
                    b = 127.0;
                } 
                TileType::Mountain => {
                    let a = 160 + rng.gen::<f32>() * 20.0;
                    r = a;
                    g = a;
                    b = a;
                }
                _ => {//MAP_ERROR
                    r = 255.0;
                    g = 100.0;
                    b = 100.0;
                }
            }

            // Push to list as flattened array
            self.image.push((0.003921568627451 * (r * r)) as u8);
            self.image.push((0.003921568627451 * (g * g)) as u8);
            self.image.push((0.003921568627451 * (b * b)) as u8);
            self.image.push(255); //a
        }
    }
}
