#![feature(trait_alias)]
mod poly;

use std::time::Instant;

use poly::Polynomial;

fn main() {
    let a = Polynomial::new(Vec::from_iter(0..5));
    let b = Polynomial::new(Vec::from_iter(0..5));

    println!("# Dumb multiplication");
    let now = Instant::now();
    let c_schoolbook = &a.multiply_schoolbook(&b);
    let elapsed = now.elapsed();
    println!("   ({:?})\n x ({:?})\n = {:?}", a, b, c_schoolbook);
    println!("Elapsed time: {:?}", elapsed);

    println!();

    println!("# Fancy schmancy multiplication");
    let now = Instant::now();
    let c_karatsuba = &a.multiply_karatsuba(&b);
    let elapsed = now.elapsed();
    println!("   ({:?})\n x ({:?})\n = {:?}", a, b, c_karatsuba);
    println!("Elapsed time: {:?}", elapsed);

    assert_eq!(c_karatsuba, c_schoolbook);
}
