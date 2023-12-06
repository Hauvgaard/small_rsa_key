use num_bigint::RandBigInt;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
use rand::Rng;

fn main() {
    //println!("nbits: ");

    //let mut line = String::new();
    //std::io::stdin().read_line(&mut line).unwrap();
    //let nbits = match line.trim().parse::<u32>() {
    //    Ok(x) => x,
    //    Err(_) => continue,
    //};

    println!("nbits,n,p,q,%");
    for nbits in 30..40 {
        //let nbits = 40;
        let mut modulus = BigUint::from(1u32);
        let mut primes = vec![];
        for i in 0..100 {
            primes.push(gen_nbit_random(nbits));
        }

        'perc: for p in 0..10 {
            for i in 0..100 {
                for j in i..100 {
                    let ratio = &primes[i] * 100u32 / &primes[j];
                    if ratio < (100u32 - p * 5).into() && ratio > (100u32 - (p + 1) * 5).into() {
                        println!(
                            "{},{},{},{},{}",
                            nbits,
                            &primes[i] * &primes[j],
                            &primes[i],
                            &primes[j],
                            ratio
                        );
                        continue 'perc;
                    }
                }
            }
        }
    }
    /*
    for i in &['p', 'q'] {
        let prime = gen_nbit_random(nbits);
        modulus *= &prime;
        println!("{}: {}", i, &prime);
    }*/
    //println!("modulus: {}", modulus);
    //println!("Key Size: {}\n", 2 * nbits);
}

pub struct RsaKey {
    pub p: u128,
    pub q: u128,
    pub e: u128,
}

fn gen_nbit_random(nbits: u32) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut rand = BigUint::from(1u32);
    rand |= rng.gen_biguint_range(&Zero::zero(), (&(BigUint::from(1u32) << (nbits - 2)))) << 1u32;
    rand |= BigUint::from(1u32) << (nbits - 1);
    while !miller_rabin(&rand) {
        rand += 2u32;
    }
    return rand;
}

fn miller_rabin(num: &BigUint) -> bool {
    let mut rng = rand::thread_rng();
    if (num & BigUint::from(1u32)) == Zero::zero() {
        return false;
    }
    let k = 50; // Sandsynlighed for fejl er 4^(-k) approx 1e-30z

    // Factor out 2 of n-1
    let mut s = 0;
    let mut nm1: BigUint = num - BigUint::from(1u32);
    while &nm1 & BigUint::from(1u32) == Zero::zero() {
        s += 1;
        nm1 /= BigUint::from(2u32);
    }
    let mut d = nm1;

    for _ in 0..k {
        let numm2 = num - BigUint::from(2u32);
        let a: BigUint = rng.gen_biguint_range(&2u32.into(), &numm2);
        let mut x = a.modpow(&d, &num);
        for _ in 0..s {
            let y = x.modpow(&2_u32.into(), &num);
            if y == One::one() && x != One::one() && x != (num - BigUint::from(1u32)) {
                return false;
            }
            x = y;
        }
        if x != One::one() {
            return false;
        }
    }
    return true;
}
