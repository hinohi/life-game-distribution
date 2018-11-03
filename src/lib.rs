extern crate num_traits;

use num_traits::PrimInt;

pub struct Mover<T>
where
    T: PrimInt,
{
    moves: Vec<T>,
}

macro_rules! int_impl {
    ($T: ty, $S: expr) => {
        impl Mover<$T> {
            pub fn new(size: usize) -> Result<Mover<$T>, ()> {
                if size > $S {
                    return Err(());
                }
                let mut units = Vec::new();
                for i in (0..size).rev() {
                    let u: $T = (1 << i) + (1 << (i + 1) % size) + (1 << (i + size - 1) % size);
                    units.push(u);
                }
                let mut moves = Vec::new();
                for a in &units {
                    for b in &units {
                        let mut m: $T = 0;
                        let mut mask: $T = 1 << (size - 1);
                        for _ in 0..size {
                            m <<= size;
                            if a & mask > 0 {
                                m |= *b;
                            }
                            mask >>= 1;
                        }
                        moves.push(m);
                    }
                }
                Ok(Mover { moves })
            }

            pub fn apply(&self, board: $T) -> $T {
                let mut b: $T = 0;
                for m in &self.moves {
                    b <<= 1;
                    let alive = (board & m).count_ones();
                    if alive == 3 || alive == 4 {
                        b |= 1;
                    }
                }
                b
            }
        }
    };
}

int_impl!(u32, 5);
int_impl!(u64, 8);
int_impl!(u128, 11);

#[cfg(test)]
mod tests {
    use super::Mover;

    #[test]
    fn smoke_l4() {
        let m = Mover::<u32>::new(4).unwrap();
        assert_eq!(0, m.apply(0));
        assert_eq!(0, m.apply(1));
    }

    #[test]
    fn smoke_l5() {
        let m = Mover::<u32>::new(5).unwrap();
        assert_eq!(0, m.apply(0));
        assert_eq!(0, m.apply(1));
    }

    #[test]
    fn smoke_l6() {
        let m = Mover::<u64>::new(6).unwrap();
        assert_eq!(0, m.apply(0));
        assert_eq!(0, m.apply(1));
    }

    #[test]
    fn smoke_l8() {
        let m = Mover::<u64>::new(8).unwrap();
        assert_eq!(0, m.apply(0));
        assert_eq!(0, m.apply(1));
    }

    #[test]
    fn smoke_l11() {
        let m = Mover::<u128>::new(11).unwrap();
        assert_eq!(0, m.apply(0));
        assert_eq!(0, m.apply(1));
    }
}
