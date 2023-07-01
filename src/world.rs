use noise::{NoiseFn, Perlin, utils::Color};
use rand::prelude::*;
use sdl2::{
    pixels::PixelFormatEnum,
    render::{Canvas, Texture},
    video::Window,
};

use crate::fire;


#[derive(Copy, Clone)]
pub struct Point {
    pub x : i32,
    pub y : i32
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum TileType {
    Invalid,
    Water,
    Grass,
    Brush,
    Tree,
    Mountain
}

// Map struct 
#[derive(Clone)]
pub(crate) struct Map {
    pub size: u32, // Size of pixel array
    pub terrain: Vec<TileType>,
    pub fire: Vec<u8>,
    pub active: Vec<bool>,
    pub image: Vec<u8>,
    pub plain_thresh: f32,
    pub mountain_thresh: f32,
}

impl Point {
    // Add a point to this point and return NEW point
    fn add (&mut self, p : Point) -> Point {
        return Point { x : self.x + p.x, y : self.y + p.y };
    }
    
    // Add a point to this point IN PLACE (modify it)
    fn add_i (&mut self, p : Point) {
        self.x += p.x;
        self.y += p.y;
    }

    // Multiply by scalar and return NEW point
    fn scalar_mult (&mut self, a : i32) -> Point {
        return  Point { x: self.x * a, y: self.y * a};
    }

    // Multiply by scalar IN PLACE
    fn scalar_mult_i (&mut self, a : i32) {
        self.x *= a;
        self.y *= a;
    }

    // Print position
    fn print(&mut self){
        println!("Point: {:},{:}", self.x, self.y);
    }
}

// Class methods for Map
impl Map {
    // Get index into image

    // Get index into terrain map
    pub fn index (&mut self, x : i32, y : i32) -> usize {
        return (y * self.size as i32 + x) as usize;
    }

    // Check if x
    fn check_bounds (&mut self, x : i32, y : i32) -> bool {
        return x >= 0 && x < self.size as i32 && y >= 0 && y < self.size as i32;
    }

    // Generate mountains
    pub fn generate_mountains(&mut self, v_0_range : u32, v_0_min : u32, max_len : u32, max_accel : u32) {
        // Generate a seed point for the fault line and generate initial velocity
        let mut fault_seed : Point = Point { x: (random::<u32>() % self.size) as i32, y: (random::<u32>() % self.size) as i32 };
        let mut fault_velocity : Point = Point { x: ((random::<u32>() % v_0_range) + v_0_min) as i32 * (((random::<u32>() % 2) * 2) as i32 - 1), 
                                                 y: ((random::<u32>() % v_0_range) + v_0_min) as i32 * (((random::<u32>() % 2) * 2) as i32 - 1)};
        let mut next_point : Point = fault_seed;

        // Set black pixel for debugging and print
        self.update_pixel(fault_seed.x as u32, fault_seed.y as u32, 0, 0, 0, 255);
        print!("Fault seed: {:},{:}\nv_0: ", fault_seed.x, fault_seed.y);
        fault_velocity.print();

        // NOTE: Could have it follow perlin noise field instead
        // Create each line segment
        for i in 0..(random::<u32>() % max_len) {
            // Introduce randomized acceleration to fault path
            let mut fault_acceleration : Point = Point { x: (random::<u32>() % max_accel) as i32 * (((random::<u32>() % 2) * 2) as i32 - 1), 
                                                         y: (random::<u32>() % max_accel) as i32 * (((random::<u32>() % 2) * 2) as i32 - 1)};
            fault_velocity = fault_velocity.add(fault_acceleration);
            print!("Acc: ");
            fault_acceleration.print();
            print!("Vel: ");
            fault_velocity.print();
            
            // Add the velocity vector to previous point to get new point
            next_point = next_point.add(fault_velocity);
            print!("Next: ");
            next_point.print();
            
            if !self.check_bounds(next_point.x, next_point.y) {
                break;
            }
            
            self.update_pixel(next_point.x as u32, next_point.y as u32, 0, 0, 0, 255);
            
        }
    }


    pub fn update_pixel(&mut self, x:u32, y:u32, r:u8, g:u8, b:u8, a:u8){
        let i = (x*self.size)+y;
        self.image[(i*4) as usize] = r;
        self.image[(i*4) as usize + 1] = g;
        self.image[(i*4) as usize + 2] = b;
        self.image[(i*4) as usize + 3] = a;


    }
    //TODO: implement map data layer generations
    pub fn generate_layers(&mut self) {
        self.fire = vec![0; (self.size*self.size).try_into().unwrap()];
        self.active = vec![false; (self.size*self.size).try_into().unwrap()];
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
            if pln_stable >= 0.0 && pln_stable < 0.3 {
                //water
                self.terrain.push(TileType::Water);
            } else if pln >= 0.3 && pln < 0.4 {// marsh
                self.terrain.push(TileType::Brush);
            } else if pln >= 0.4 && pln < 1.3 {// grass
                self.terrain.push(TileType::Grass);
            } else if pln >= 1.3 {// grass
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
                    let a = 160.0 + rng.gen::<f32>() * 20.0;
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
        self.generate_mountains(15,10, 25, 15);
    }

    pub fn update(&mut self){
        fire::simulation_update( self);
    }
}
