#![feature(trait_alias)]
mod poly;

use std::time::Instant;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::poly::{mul_karatsuba, mul_schoolbook};

fn main() {
    println!("Degree,Schoolbook,Karatsuba,Karatsuba2,Karatsuba32,Karatsuba64");
    (5..20).into_par_iter().for_each(|n| {
        let x = Vec::from_iter(0..2u64.pow(n));
        let y = Vec::from_iter(0..2u64.pow(n));

        let now = Instant::now();
        let c_schoolbook = mul_schoolbook(&x, &y);
        let elapsed_schoolbook = now.elapsed().as_micros();

        let now = Instant::now();
        let c_karatsuba = mul_karatsuba(&x, &y, 1);
        let elapsed_karatsuba = now.elapsed().as_micros();

        let now = Instant::now();
        let c_karatsuba2 = mul_karatsuba(&x, &y, 2);
        let elapsed_karatsuba2 = now.elapsed().as_micros();

        let now = Instant::now();
        let c_karatsuba32 = mul_karatsuba(&x, &y, 32);
        let elapsed_karatsuba32 = now.elapsed().as_micros();

        assert_eq!(c_karatsuba, c_schoolbook);
        assert_eq!(c_karatsuba2, c_schoolbook);
        assert_eq!(c_karatsuba32, c_schoolbook);
        println!(
            "{},{elapsed_schoolbook},{elapsed_karatsuba},{elapsed_karatsuba2},{elapsed_karatsuba32}",
            2u64.pow(n)
        )
    });
}
