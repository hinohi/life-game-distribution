use std::env::args;

use life_game_distribution::{lives_count_dist::Dist, Universe};

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let n = args[0].parse().unwrap();
    let g = args[1].parse::<usize>().unwrap();
    let mut u = Universe::new(n);
    let mut dist = Dist::new(g);

    loop {
        dist.count_up(&u);
        if !u.next_permutation() {
            break;
        }
    }
    dist.dump(n);
}
