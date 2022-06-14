use std::time::Instant;
use grid_search::gen_2d_list;

fn main() {
    let grid = gen_2d_list();

    let target = 50;
    let mut counter = 0;
    let now = Instant::now();

    for row in grid {
        if row.contains(&target) {
            counter += 1;
        }
    }

    println!("There were {} 50s", counter);
    println!("{:?}", now.elapsed());
}