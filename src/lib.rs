use rand::Rng;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Universe {
    n: usize,
    cell: Vec<u8>,
}

impl Universe {
    pub fn new(n: usize) -> Universe {
        assert!(n >= 3);
        Universe {
            n,
            cell: vec![0; n * n],
        }
    }

    pub fn from_rng<R: Rng>(n: usize, r: &mut R) -> Universe {
        Universe {
            n,
            cell: (0..n * n).map(|_| r.gen_range(0, 2)).collect::<Vec<_>>(),
        }
    }

    pub fn next_generation(&self) -> Universe {
        let mut cell = Vec::with_capacity(self.n * self.n);
        // x=0 y=0
        let count = self.cell[(self.n - 1) * self.n + self.n - 1]
            + self.cell[(self.n - 1) * self.n + 0]
            + self.cell[(self.n - 1) * self.n + 1]
            + self.cell[0 + self.n - 1]
            + self.cell[0 + 1]
            + self.cell[self.n + self.n - 1]
            + self.cell[self.n + 0]
            + self.cell[self.n + 1];
        cell.push(match (self.cell[0], count) {
            (0, 3) => 1,
            (1, 2) | (1, 3) => 1,
            _ => 0,
        });
        // y=0
        for x in 1..self.n - 1 {
            let x0 = x - 1;
            let x1 = x + 1;
            let count = self.cell[(self.n - 1) * self.n + x0]
                + self.cell[(self.n - 1) * self.n + x]
                + self.cell[(self.n - 1) * self.n + x1]
                + self.cell[x0]
                + self.cell[x1]
                + self.cell[self.n + x0]
                + self.cell[self.n + x]
                + self.cell[self.n + x1];
            cell.push(match (self.cell[x], count) {
                (0, 3) => 1,
                (1, 2) | (1, 3) => 1,
                _ => 0,
            });
        }
        // x=n-1 y=0
        let count = self.cell[(self.n - 1) * self.n + self.n - 2]
            + self.cell[(self.n - 1) * self.n + self.n - 1]
            + self.cell[(self.n - 1) * self.n]
            + self.cell[0 + self.n - 2]
            + self.cell[0 + 0]
            + self.cell[self.n + self.n - 2]
            + self.cell[self.n + self.n - 1]
            + self.cell[self.n + 0];
        cell.push(match (self.cell[self.n - 1], count) {
            (0, 3) => 1,
            (1, 2) | (1, 3) => 1,
            _ => 0,
        });
        for y in 1..self.n - 1 {
            let count = self.cell[(y - 1) * self.n + self.n - 1]
                + self.cell[(y - 1) * self.n + 0]
                + self.cell[(y - 1) * self.n + 1]
                + self.cell[y * self.n + self.n - 1]
                + self.cell[y * self.n + 1]
                + self.cell[(y + 1) * self.n + self.n - 1]
                + self.cell[(y + 1) * self.n + 0]
                + self.cell[(y + 1) * self.n + 1];
            cell.push(match (self.cell[y * self.n], count) {
                (0, 3) => 1,
                (1, 2) | (1, 3) => 1,
                _ => 0,
            });
            for x in 1..self.n - 1 {
                let x0 = x - 1;
                let x1 = x + 1;
                let y0 = y - 1;
                let y1 = y + 1;
                let count = self.cell[y0 * self.n + x0]
                    + self.cell[y0 * self.n + x]
                    + self.cell[y0 * self.n + x1]
                    + self.cell[y * self.n + x0]
                    + self.cell[y * self.n + x1]
                    + self.cell[y1 * self.n + x0]
                    + self.cell[y1 * self.n + x]
                    + self.cell[y1 * self.n + x1];
                cell.push(match (self.cell[y * self.n + x], count) {
                    (0, 3) => 1,
                    (1, 2) | (1, 3) => 1,
                    _ => 0,
                });
            }
            let count = self.cell[(y - 1) * self.n + self.n - 2]
                + self.cell[(y - 1) * self.n + self.n - 1]
                + self.cell[(y - 1) * self.n]
                + self.cell[y * self.n + self.n - 2]
                + self.cell[y * self.n + 0]
                + self.cell[(y + 1) * self.n + self.n - 2]
                + self.cell[(y + 1) * self.n + self.n - 1]
                + self.cell[(y + 1) * self.n + 0];
            cell.push(match (self.cell[y * self.n + self.n - 1], count) {
                (0, 3) => 1,
                (1, 2) | (1, 3) => 1,
                _ => 0,
            });
        }
        // x=0 y=n-1
        let count = self.cell[(self.n - 2) * self.n + self.n - 1]
            + self.cell[(self.n - 2) * self.n + 0]
            + self.cell[(self.n - 2) * self.n + 1]
            + self.cell[(self.n - 1) * self.n + self.n - 1]
            + self.cell[(self.n - 1) * self.n + 1]
            + self.cell[self.n - 1]
            + self.cell[0]
            + self.cell[1];
        cell.push(match (self.cell[(self.n - 1) * self.n], count) {
            (0, 3) => 1,
            (1, 2) | (1, 3) => 1,
            _ => 0,
        });
        // y=n-1
        for x in 1..self.n - 1 {
            let x0 = x - 1;
            let x1 = x + 1;
            let count = self.cell[(self.n - 2) * self.n + x0]
                + self.cell[(self.n - 2) * self.n + x]
                + self.cell[(self.n - 2) * self.n + x1]
                + self.cell[(self.n - 1) * self.n + x0]
                + self.cell[(self.n - 1) * self.n + x1]
                + self.cell[x0]
                + self.cell[x]
                + self.cell[x1];
            cell.push(match (self.cell[(self.n - 1) * self.n + x], count) {
                (0, 3) => 1,
                (1, 2) | (1, 3) => 1,
                _ => 0,
            });
        }
        // x=n-1 y=n-1
        let count = self.cell[(self.n - 2) * self.n + self.n - 2]
            + self.cell[(self.n - 2) * self.n + self.n - 1]
            + self.cell[(self.n - 2) * self.n]
            + self.cell[(self.n - 1) * self.n + self.n - 2]
            + self.cell[(self.n - 1) * self.n + 0]
            + self.cell[self.n - 2]
            + self.cell[self.n - 1]
            + self.cell[0];
        cell.push(
            match (self.cell[(self.n - 1) * self.n + self.n - 1], count) {
                (0, 3) => 1,
                (1, 2) | (1, 3) => 1,
                _ => 0,
            },
        );
        Universe { n: self.n, cell }
    }

    pub fn lives(&self) -> usize {
        let mut ret = 0;
        for c in self.cell.iter() {
            ret += *c as usize;
        }
        ret
    }

    pub fn next_generation_lives(&self) -> usize {
        let mut lives = 0;
        for y in 0..self.n {
            for x in 0..self.n {
                let x0 = (x + self.n - 1) % self.n;
                let x1 = (x + 1) % self.n;
                let y0 = (y + self.n - 1) % self.n;
                let y1 = (y + 1) % self.n;
                let count = self.cell[y0 * self.n + x0]
                    + self.cell[y0 * self.n + x]
                    + self.cell[y0 * self.n + x1]
                    + self.cell[y * self.n + x0]
                    + self.cell[y * self.n + x1]
                    + self.cell[y1 * self.n + x0]
                    + self.cell[y1 * self.n + x]
                    + self.cell[y1 * self.n + x1];
                match (self.cell[y * self.n + x], count) {
                    (0, 3) => lives += 1,
                    (1, 2) | (1, 3) => lives += 1,
                    _ => (),
                }
            }
        }
        lives
    }

    pub fn next_permutation(&mut self) -> bool {
        for i in 0..self.n * self.n {
            if self.cell[i] == 0 {
                self.cell[i] = 1;
                return true;
            } else {
                self.cell[i] = 0;
            }
        }
        false
    }

    pub fn one_cell_flip<R: Rng>(&self, r: &mut R) -> Universe {
        let i = r.gen_range(0, self.n * self.n);
        let mut u = self.clone();
        u.cell[i] = 1 - u.cell[i];
        u
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    fn from_str(s: &str) -> Vec<u8> {
        Vec::from_iter(s.chars().filter_map(|c| match c {
            '0' => Some(0),
            '1' => Some(1),
            _ => None,
        }))
    }

    #[test]
    fn oscillator_clock() {
        let mut u = Universe::new(6);
        u.cell = from_str(
            r#"
            000000
            001000
            000110
            011000
            000100
            000000
        "#,
        );
        let u = u.next_generation();
        assert_eq!(
            u.cell,
            from_str(
                r#"
            000000
            000100
            010100
            001010
            001000
            000000
        "#
            )
        );
    }

    #[test]
    fn glider() {
        let mut u = Universe::new(5);
        u.cell = from_str(
            r#"
            01000
            00100
            11100
            00000
            00000
        "#,
        );
        let u = u.next_generation();
        assert_eq!(
            u.cell,
            from_str(
                r#"
            00000
            10100
            01100
            01000
            00000
        "#
            )
        );
        let u = u.next_generation();
        assert_eq!(
            u.cell,
            from_str(
                r#"
            00000
            00100
            10100
            01100
            00000
        "#
            )
        );
        let u = u.next_generation();
        let u = u.next_generation();
        assert_eq!(
            u.cell,
            from_str(
                r#"
            00000
            00100
            00010
            01110
            00000
        "#
            )
        );
        let u = u.next_generation();
        let u = u.next_generation();
        let u = u.next_generation();
        let u = u.next_generation();
        assert_eq!(
            u.cell,
            from_str(
                r#"
            00000
            00000
            00010
            00001
            00111
        "#
            )
        );
        let u = u.next_generation();
        let u = u.next_generation();
        let u = u.next_generation();
        let u = u.next_generation();
        assert_eq!(
            u.cell,
            from_str(
                r#"
            10011
            00000
            00000
            00001
            10000
        "#
            )
        );
    }

    #[test]
    fn lives() {
        let mut u = Universe::new(4);
        loop {
            let n = u.next_generation();
            assert_eq!(n.lives(), u.next_generation_lives());
            if !u.next_permutation() {
                break;
            }
        }
    }

    #[test]
    fn next_permutation() {
        let mut u = Universe::new(3);
        assert_eq!(u.cell, vec![0; 9]);
        assert!(u.next_permutation());
        assert_eq!(u.cell, vec![1, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(u.next_permutation());
        assert_eq!(u.cell, vec![0, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert!(u.next_permutation());
        assert_eq!(u.cell, vec![1, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert!(u.next_permutation());
        assert_eq!(u.cell, vec![0, 0, 1, 0, 0, 0, 0, 0, 0]);
        u.cell = vec![1; 9];
        assert!(!u.next_permutation());
    }
}
