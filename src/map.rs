use rand::prelude::*;
use noise::{NoiseFn, Perlin};
use sdl2::{pixels::PixelFormatEnum, render::{Canvas, Texture}, video::Window};

pub fn generate(s: u32) -> Vec<u8>{

    let perlin = Perlin::new(1);
    let mut pixels: Vec<u8> = Vec::new();
    let scale:f64 = 30.0;
    for i in 0..(s*s){
        let mut rng = rand::thread_rng();

        let x = i%s;
        let y = i/s;
        let mut pln = perlin.get([x as f64/scale, y as f64/scale, 0.0])+1.0;
        let pln_stable = pln;
        pln = pln + rng.gen::<f64>()*0.1;
        let r: f32;
        let g: f32;
        let b: f32;
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

        pixels.push((0.003921568627451*(r*r)) as u8);
        pixels.push((0.003921568627451*(g*g)) as u8);
        pixels.push((0.003921568627451*(b*b)) as u8);
        pixels.push(255);//a

    }


    return  pixels;

}