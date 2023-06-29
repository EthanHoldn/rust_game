use rand::{random, Rng};

use crate::world::{Map, TileType};
const BORDER: u32 = 5;
const MAX_BURNT: u8 = 10;

fn index (size:u32, x : u32, y : u32) -> usize {
    return (x * size + y) as usize;
}
pub(crate) fn simulation_update(map : &mut Map){
    let size = map.size;
    let mut update: Vec<u8> = map.fire.clone();
    //for _ in 0..10{
        for x in BORDER..size-BORDER{
            for y in BORDER..size-BORDER{
                if map.fire[index(size,x,y)] >= MAX_BURNT{
                    map.update_pixel(x, y, 255, 255, 255, 255);
                }
                //if tile is on fire 
                //rand::thread_rng().gen_range(0..100) == 1
                if 
                    map.fire[index(size,x,y)] != 0 &&
                    map.fire[index(size,x,y)] <= MAX_BURNT
                    {
                    map.fire[index(size,x,y)] +=1;
                    let ox = rand::thread_rng().gen_range(0..3);
                    let oy = rand::thread_rng().gen_range(0..3);

                    if map.fire[index(size,x+ox-1,y+oy-1)] <= MAX_BURNT.into() && map.terrain[index(size,x+ox-1,y+oy-1)] == TileType::Grass{
                        update[index(size,x+ox-1,y+oy-1)] += 1;
                        map.update_pixel(x+ox-1, y+oy-1, 255, 0, 0, 255);
                    }

                }
            }
        }
        map.fire = update
    //}
    
}

pub(crate) fn spawn(map : &mut Map){
    let size = map.size;
    let x = map.size/2;
    let y = map.size/2;
    map.fire[index(size,x,y)] = 1;
    map.update_pixel(x, y, 255, 0, 0, 255)
}