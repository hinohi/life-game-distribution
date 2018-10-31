use std::collections::HashMap;

fn main() {
    let units = vec![
        0b11001_u32,
        0b11100_u32,
        0b01110_u32,
        0b00111_u32,
        0b10011_u32,
    ];
    let mut filters = Vec::new();
    for a in &units {
        for b in &units {
            let mut f = 0;
            let mut mask = 0b10000_u32;
            for _ in 0..5 {
                f <<= 5;
                if a & mask > 0 {
                    f |= *b;
                }
                mask >>= 1;
            }
            filters.push(f);
        }
    }

    let mut dist = HashMap::new();
    for a in 0..2_u32.pow(25) {
        let mut b = 0_u32;
        for f in &filters {
            b <<= 1;
            let alive = (a & f).count_ones();
            if alive == 3 || alive == 4 {
                b |= 1;
            }
        }
        *dist.entry(b).or_insert(0) += 1;
    }

    let mut keys: Vec<&u32> = dist.keys().collect();
    keys.sort_unstable_by_key(|k| (-dist[k], **k));
    for key in keys {
        println!("{:025b} {}", key, dist[key]);
    }
}
