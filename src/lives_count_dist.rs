use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::Universe;

pub struct Dist {
    g: usize,
    gen: Vec<HashMap<usize, u32>>,
}

impl Dist {
    pub fn new(g: usize) -> Dist {
        let mut gen = Vec::with_capacity(g + 1);
        for _ in 0..=g {
            gen.push(HashMap::new());
        }
        Dist { g, gen }
    }

    pub fn count_up(&mut self, u: &Universe) {
        *self.gen[0].entry(u.lives()).or_insert(0) += 1;
        let mut uu = u.next_generation();
        for g in self.gen.iter_mut().skip(1).take(self.g - 1) {
            *g.entry(uu.lives()).or_insert(0) += 1;
            uu = uu.next_generation();
        }
        *self.gen[self.g].entry(uu.lives()).or_insert(0) += 1;
    }

    pub fn dump<W: Write>(&self, n: usize, w: &mut W) -> io::Result<()> {
        for i in 0..=n * n {
            let mut line = format!("{}", i);
            for g in self.gen.iter() {
                line += &format!(" {}", g.get(&i).or(Some(&0)).unwrap());
            }
            line.push('\n');
            w.write_all(line.as_bytes())?;
        }
        Ok(())
    }
}
