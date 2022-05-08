use std::{fs::write, path::PathBuf};

use clap::Parser;
use rand_pcg::Mcg128Xsl64;

use life_game_distribution::{lives_count_dist::Dist, Universe};

#[derive(Parser)]
struct Args {
    #[clap(short)]
    n: usize,
    #[clap(short)]
    g: usize,
    #[clap(short)]
    c: usize,
    #[clap(short)]
    o: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let mut dist = Dist::new(args.g);
    let mut random = Mcg128Xsl64::new(1);
    for _ in tqdm::tqdm(0..args.c / 256) {
        for _ in 0..256 {
            let u = Universe::from_rng(args.n, &mut random);
            dist.count_up(&u);
        }
    }
    let mut line = format!("# n={} g={} c={}\n", args.n, args.g, args.c).into_bytes();
    dist.dump(args.n, &mut line).unwrap();
    write(args.o, &line).unwrap();
}
