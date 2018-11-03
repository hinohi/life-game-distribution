extern crate back_life_game2;
use back_life_game2::Mover;
use std::collections::HashMap;

fn main() {
    const L: usize = 5;
    let mover = Mover::<u128>::new(L).unwrap();
    let mut count = HashMap::new();
    for a in 0..2_u128.pow((L * L) as u32) {
        let b = mover.apply(a);
        *count.entry(b).or_insert(0) += 1;
    }
    let mut dist = HashMap::new();
    for c in count.values() {
        *dist.entry(*c).or_insert(0) += 1;
    }

    let mut keys: Vec<&u128> = dist.keys().collect();
    keys.sort_unstable_by_key(|k| (**k, dist[k]));
    for key in keys {
        println!("{} {}", key, dist[key]);
    }
}
