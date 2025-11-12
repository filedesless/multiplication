#![feature(trait_alias)]
mod poly;

use poly::Polynomial;

fn main() {
    let a = Polynomial::new(vec![2, 1]);
    let b = Polynomial::new(vec![4, 3, 2]);
    let c = &a * &b;
    println!("Hello polynomial world: ({:?}) * ({:?}) = {:?}", a, b, c);
}
