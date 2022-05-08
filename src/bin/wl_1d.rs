use std::env::args;
use std::u32;

use rand::Rng;
use rand_pcg::Mcg128Xsl64;

use life_game_distribution::Universe;

struct WangLandau {
    n: usize,
    hist: Vec<u32>,
    s: Vec<f64>,
    f: f64,
    generations: usize,
}

impl WangLandau {
    fn new(n: usize, g: usize) -> WangLandau {
        WangLandau {
            n,
            hist: vec![0; n * n * 2 + 1],
            s: vec![0.0; n * n * 2 + 1],
            f: 1.0,
            generations: g,
        }
    }

    fn is_flatten(&self) -> bool {
        let mut max_h = self.hist[0];
        let mut min_h = u32::MAX;
        for &h in self.hist.iter() {
            if h > max_h {
                max_h = h;
            }
            if h > 0 && h < min_h {
                min_h = h;
            }
        }
        max_h * 8 / 10 < min_h
    }

    fn calc_energy(&self, u: &Universe) -> usize {
        let lives = u.lives();
        let mut u = u.next_generation();
        for _ in 1..self.generations {
            u = u.next_generation();
        }
        self.n * self.n + u.lives() - lives
    }

    fn lean<R: Rng>(&mut self, r: &mut R) {
        let mut u = Universe::new(self.n);
        let mut energy = self.calc_energy(&u);
        self.hist = vec![0; self.n * self.n * 2 + 1];
        for _ in 0..10000 {
            for _ in 0..self.n * self.n * 10 {
                let nex = u.one_cell_flip(r);
                let nex_energy = self.calc_energy(&nex);
                let ds = self.s[energy] - self.s[nex_energy];
                if ds > 0.0 || r.gen_bool(ds.exp()) {
                    u = nex;
                    energy = nex_energy;
                }
                self.hist[energy] += 1;
                self.s[energy] += self.f;
            }
            if self.is_flatten() {
                break;
            }
        }
        self.f *= 0.5;
    }
}

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    let n = args[0].parse().unwrap();
    let g = args[1].parse().unwrap();
    let mut wl = WangLandau::new(n, g);
    let mut random = Mcg128Xsl64::new(1);
    for i in 0..10 {
        wl.lean(&mut random);
        eprintln!("step={}", i);
    }

    for (i, &s) in wl.s.iter().enumerate() {
        if s > 0.0 {
            println!("{} {}", i as i32 - (n * n) as i32, s);
        }
    }
}
