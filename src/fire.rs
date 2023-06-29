use rand::{random, Rng};

use crate::world::Map;

fn index (size:u32, x : u32, y : u32) -> usize {
    return (y * size + x) as usize;
}
pub(crate) fn simulation_update(map : &mut Map){
    let size = map.size;
    for x in 1..size-1{
        for y in 1..size-1{
            if map.fire[index(size,x,y)] != 0 && rand::thread_rng().gen_range(0..100) == 1 {

                map.fire[index(size,x-1,y)] = 1;
                map.update_pixel(x-1, y, 255, 0, 0, 255);

                map.fire[index(size,x+1,y)] = 1;
                map.update_pixel(x+1, y, 255, 0, 0, 255);

                map.fire[index(size,x,y-1)] = 1;
                map.update_pixel(x, y-1, 255, 0, 0, 255);

                map.fire[index(size,x,y+1)] = 1;
                map.update_pixel(x, y+1, 255, 0, 0, 255);
            }
        }
    }
}

pub(crate) fn spawn(map : &mut Map){
    let size = map.size;
    let x = map.size/2;
    let y = map.size/2;
    map.fire[index(size,x,y)] = 1;
    map.update_pixel(x, y, 255, 0, 0, 255)
}