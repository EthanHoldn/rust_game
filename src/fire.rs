use rand::{Rng};

use crate::world::{Map, TileType};
const BORDER: u32 = 5;
const MAX_BURNT: u8 = 6;

pub(crate) fn index (size:u32, x : u32, y : u32) -> Option<usize> {
    // Gaurd statement to check bounds
    if x >= size || y >= size { return None;}

    return Some((x * size + y) as usize);
}

pub(crate) fn simulation_update(map : &mut Map){
    let size = map.size;
    let fire_previous: Vec<u8> = map.fire.clone();
    let active_previous: Vec<bool> = map.active.clone();
    if map.modulator == 251 {
        map.modulator = 0
    };
    let m = map.modulator%41;
    map.modulator +=1;
    //for _ in 0..10{
        for x in BORDER..size-BORDER{
            for y in BORDER..size-BORDER{
                let i = index(size,x,y).unwrap();
                //if tile is on fire 
                //rand::thread_rng().gen_range(0..100) == 1
                if i %41 == m as usize && active_previous[i]{
                    map.fire[i] +=1;
                    if map.fire[i] > MAX_BURNT {
                        map.active[i] = false;
                        map.update_pixel(x, y, 50, 50, 50, 255);
                    }
                    let ox = rand::thread_rng().gen_range(0..3);
                    let oy = rand::thread_rng().gen_range(0..3);
                    let oi = index(size,x+ox-1,y+oy-1).unwrap();
                    if fire_previous[oi] == 0  && map.terrain[oi] != TileType::Mountain && map.terrain[oi] != TileType::Water{
                        map.fire[oi] += 1;
                        map.update_pixel(x+ox-1, y+oy-1, 255, 0, 0, 255);
                        map.active[oi] = true;
                    }

                }
            }
        }

    //}
    
}

pub(crate) fn spawn(map : &mut Map){
    let size = map.size;
    let x = map.size/2;
    let y = map.size/2;
    map.fire[index(size,x,y).unwrap()] = 1;
    map.update_pixel(x, y, 255, 0, 0, 255);
    map.active[index(size,x,y).unwrap()] = true;
}