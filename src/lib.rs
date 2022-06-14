extern crate rand;

use rand::thread_rng;
use rand::Rng;

pub fn gen_2d_list() -> [[u8; 100]; 100] {
    let mut arrays = [[0; 100]; 100];
    for i in 0..100 {
        arrays[i] = random_hundred();
    }
    arrays
} 

fn random_hundred() -> [u8; 100] {
    let mut rng = thread_rng();
    let mut numbers = [0; 100];
    for i in 0..100 {
        numbers[i] = rng.gen_range(1..=100);
    }
    numbers
}