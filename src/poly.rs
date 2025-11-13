use std::fmt;

use num::Num;

pub trait Ring = Num + Copy + Clone + fmt::Debug;

pub fn degree<R: Ring>(poly: &[R]) -> usize {
    poly.len() - 1
}

pub fn mul_schoolbook<R: Ring>(x: &[R], y: &[R]) -> Vec<R> {
    let mut result = vec![R::zero(); degree(x) + degree(y) + 1];
    for (i, &ai) in x.iter().enumerate() {
        for (j, &bj) in y.iter().enumerate() {
            result[i + j] = result[i + j] + (ai * bj);
        }
    }
    result
}

pub fn add<R: Ring>(x: &[R], y: &[R]) -> Vec<R> {
    let mut result = vec![R::zero(); degree(x).max(degree(y)) + 1];
    result.copy_from_slice(x);
    for (i, &bi) in y.iter().enumerate() {
        result[i] = result[i] + bi;
    }
    result
}

pub fn sub<R: Ring>(x: &[R], y: &[R]) -> Vec<R> {
    let mut result = vec![R::zero(); degree(x).max(degree(y)) + 1];
    result.copy_from_slice(x);
    for (i, &bi) in y.iter().enumerate() {
        result[i] = result[i] - bi;
    }
    result
}

pub fn mul_karatsuba<R: Ring>(x: &[R], y: &[R], threshold: usize) -> Vec<R> {
    assert!(x.len() == y.len());
    assert!(x.len().is_power_of_two());
    let n = x.len();
    if n == threshold {
        return mul_schoolbook(x, y);
    }
    let m = n / 2;
    let (x_lo, x_hi) = x.split_at(m);
    let (y_lo, y_hi) = y.split_at(m);
    let z0 = mul_karatsuba(x_lo, y_lo, threshold);
    let z2 = mul_karatsuba(x_hi, y_hi, threshold);
    let z3 = mul_karatsuba(&add(x_lo, x_hi), &add(y_lo, y_hi), threshold);
    let z1 = sub(&sub(&z3, &z0), &z2);
    let mut result = vec![R::zero(); degree(x) + degree(y) + 1];
    result[0..z0.len()].copy_from_slice(&z0);
    result[n..n + z2.len()].copy_from_slice(&z2);
    for (i, &ai) in z1.iter().enumerate() {
        result[m + i] = result[m + i] + ai;
    }
    result
}

/// How to display a polynomial (higher degree coefficient first)
fn print<R: Ring>(poly: &[R]) {
    let format_string = poly
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, coeff)| !coeff.is_zero())
        .map(|(i, &coeff)| match i {
            0 => format!("{coeff:?}"),
            1 => format!("{coeff:?}x"),
            _ => format!("{coeff:?}x^{i}"),
        })
        .collect::<Vec<String>>()
        .join(" + ");
    println!("{}", format_string)
}

fn eq<R: Ring>(x: &[R], y: &[R]) -> bool {
    x.iter()
        .zip(y.iter())
        .all(|(&a, &b)| (a == R::zero() && b == R::zero()) || a == b)
}
