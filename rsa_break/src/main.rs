use std::fs::File;
use std::io::{self, BufRead};
use std::time::{Duration, Instant};

use num_bigint::{BigUint, ToBigUint};
use num_traits::cast::ToPrimitive;

fn main() -> io::Result<()> {
    let data = io::BufReader::new(File::open("./data_all")?)
        .lines()
        .map(|x| {
            x.unwrap()
                .split(',')
                .map(|num| num.parse::<BigUint>().unwrap())
                .collect::<Vec<BigUint>>()
        })
        .collect::<Vec<Vec<BigUint>>>();

    println!("NBits,Time,p,q, Closeness");
    for line in data {
        let nbits = &line[0];
        let n = &line[1];
        let perc = &line[4];

        let start = Instant::now();
        let (p, q) = fermat_factor(&n);
        let duration = start.elapsed();
        println!(
            "{},{},{},{},{}",
            nbits,
            duration.as_millis(),
            &p,
            &q,
            100u32 - perc.to_u32().unwrap()
        );
    }

    Ok(())
}

fn fermat_factor(n: &BigUint) -> (BigUint, BigUint) {
    let mut a = n.sqrt() + 1u32;
    let mut b2 = a.pow(2) - n;

    while !is_square(&b2) {
        a += 1u32;
        b2 = a.pow(2) - n
    }
    let b = b2.sqrt();

    let q = &a + b;
    let p = 2u32 * a - &q;

    return (p, q);
}

fn is_square(no: &BigUint) -> bool {
    let n = no.clone();
    if (&n & BigUint::from(2u32) != 0u32.into())
        || (&n & BigUint::from(7u32)) == 5u32.into()
        || (&n & BigUint::from(11u32)) == 8u32.into()
    {
        return false;
    }
    return match (&n & BigUint::from(0xFu32)).to_u32().unwrap() {
        0 | 1 | 4 | 9 => {
            let sq = n.sqrt();
            sq.pow(2) == n
        }
        _ => false,
    };
}
