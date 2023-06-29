use rand::{random, Rng};

use crate::world::{Map, TileType};
const BORDER: u32 = 5;
const MAX_BURNT: u8 = 3;

fn index (size:u32, x : u32, y : u32) -> usize {
    return (x * size + y) as usize;
}
pub(crate) fn simulation_update(map : &mut Map){
    let size = map.size;
    let mut fire_update: Vec<u8> = map.fire.clone();
    let mut active_update: Vec<bool> = map.active.clone();
    let mut active_count = 0;
    //for _ in 0..10{
        for x in BORDER..size-BORDER{
            for y in BORDER..size-BORDER{
                let i = index(size,x,y);
                //if tile is on fire 
                //rand::thread_rng().gen_range(0..100) == 1
                if map.active[i]{
                    active_count += 1;
                    map.fire[i] +=1;
                    if map.fire[i] > MAX_BURNT {
                        map.active[i] = false;
                        map.update_pixel(x, y, 255, 255, 255, 255);
                    }
                    let ox = rand::thread_rng().gen_range(0..3);
                    let oy = rand::thread_rng().gen_range(0..3);
                    let oi = index(size,x+ox-1,y+oy-1);
                    if map.fire[oi] == 0  && map.terrain[oi] == TileType::Grass{
                        fire_update[oi] += 1;
                        map.update_pixel(x+ox-1, y+oy-1, 255, 0, 0, 255);
                        active_update[oi] = true;
                    }

                }
            }
        }
        println!("{}", active_count);
        map.fire = fire_update;
        map.active = active_update;

    //}
    
}

pub(crate) fn spawn(map : &mut Map){
    let size = map.size;
    let x = map.size/2;
    let y = map.size/2;
    map.fire[index(size,x,y)] = 1;
    map.update_pixel(x, y, 255, 0, 0, 255);
    map.active[index(size,x,y)] = true;
}