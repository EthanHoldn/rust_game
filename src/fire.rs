use rand::random;

use crate::world::Map;
pub(crate) fn simulation_update(map : &mut Map){
    let x = random::<u32>() % map.size;
    let y = random::<u32>() % map.size;
    let i = (x*map.size)+y;
    map.fire[i as usize] = 1;
    map.image[(i*4) as usize] = 255;
}