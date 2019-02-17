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

    pub fn next_generation(&self) -> Universe {
        let mut cell = Vec::with_capacity(self.n * self.n);
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
                cell.push(match (self.cell[y * self.n + x], count) {
                    (0, 3) => 1,
                    (1, 2) | (1, 3) => 1,
                    _ => 0,
                });
            }
        }
        Universe { n: self.n, cell }
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
