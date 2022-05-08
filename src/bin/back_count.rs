use std::collections::HashMap;
use std::env::args;

use life_game_distribution::Universe;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let n = args[0].parse().unwrap();
    let g = args[1].parse::<usize>().unwrap();

    let mut u = Universe::new(n);
    let mut count = HashMap::new();
    loop {
        *count.entry(u.n_generation(g)).or_insert(0) += 1;
        if !u.next_permutation() {
            break;
        }
    }
    let mut hist = HashMap::new();
    for c in count.values() {
        *hist.entry(*c).or_insert(0) += 1;
    }
    let mut keys = hist.keys().collect::<Vec<_>>();
    keys.sort();
    for key in keys {
        println!("{} {}", key, hist[key]);
    }
}
