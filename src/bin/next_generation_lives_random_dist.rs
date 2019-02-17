use std::collections::HashMap;
use std::env::args;

use rand_pcg::Mcg128Xsl64;

use back_life_game2::Universe;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let n = args[0].parse().unwrap();
    let g = args[1].parse::<usize>().unwrap();
    let c = args[2].parse::<usize>().unwrap();

    let mut gen = Vec::new();
    for _ in 0..=g {
        gen.push(HashMap::with_capacity(n * n));
    }
    let mut random = Mcg128Xsl64::new(1);
    for _ in 0..c {
        let u = Universe::from_rng(n, &mut random);

        *gen[0].entry(u.lives()).or_insert(0) += 1;
        let mut uu = u.next_generation();
        for i in 1..g {
            *gen[i].entry(uu.lives()).or_insert(0) += 1;
            uu = uu.next_generation();
        }
        *gen[g].entry(uu.lives()).or_insert(0) += 1;
    }
    for i in 0..n * n {
        let mut line = format!("{} {}", i, gen[0].get(&i).or(Some(&0)).unwrap());
        for j in 1..=g {
            line += &format!(" {}", gen[j].get(&i).or(Some(&0)).unwrap());
        }
        println!("{}", line);
    }
}
