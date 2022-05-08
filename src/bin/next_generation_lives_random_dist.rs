use std::env::args;

use rand_pcg::Mcg128Xsl64;

use life_game_distribution::{lives_count_dist::Dist, Universe};

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let n = args[0].parse().unwrap();
    let g = args[1].parse::<usize>().unwrap();
    let c = args[2].parse::<usize>().unwrap();

    let mut dist = Dist::new(g);
    let mut random = Mcg128Xsl64::new(1);
    for _ in 0..c {
        let u = Universe::from_rng(n, &mut random);
        dist.count_up(&u);
    }
    println!("# n={} g={} c={}", n, g, c);
    dist.dump(n);
}
