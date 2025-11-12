use std::{
    fmt,
    ops::{Add, Mul, Sub},
};

use num::Num;

pub trait Ring = Num + Copy + Clone + fmt::Debug;

/// Polynomial over a ring R represented by its vector of coefficients (lowest degree first).
#[derive(Clone)]
pub(crate) struct Polynomial<R: Ring>(Vec<R>);

impl<R: Ring> Polynomial<R> {
    pub fn new(coefficients: Vec<R>) -> Self {
        Self(coefficients)
    }

    /// Returns the degree of the polynomial.
    /// That is the highest of the degrees of its monomials.
    pub fn degree(&self) -> usize {
        self.0.len() - 1
    }

    pub fn multiply_schoolbook(&self, other: &Polynomial<R>) -> Polynomial<R> {
        let mut result = vec![R::zero(); self.degree() + other.degree() + 1];
        for (i, &ai) in self.0.iter().enumerate() {
            for (j, &bj) in other.0.iter().enumerate() {
                result[i + j] = result[i + j] + (ai * bj);
            }
        }
        Polynomial(result)
    }

    pub fn multiply_karatsuba(&self, other: &Polynomial<R>) -> Polynomial<R> {
        let n = self.0.len().max(other.0.len()).next_power_of_two();
        let mut a = self.clone();
        let mut b = other.clone();
        a.0.resize(n, R::zero());
        b.0.resize(n, R::zero());
        a.multiply_karatsuba_rec(&b)
    }

    fn multiply_karatsuba_rec(&self, other: &Polynomial<R>) -> Polynomial<R> {
        let n = self.0.len();
        assert_eq!(n, other.0.len());
        assert!(n.is_power_of_two());
        // base case - TODO: find a good threshold for schoolbook fallback
        if n == 1 {
            return self.multiply_schoolbook(other);
        }
        let m = n / 2;

        // TODO: maybe try to reduce the crazy heap allocations
        let x0 = Polynomial::from(&self.0[..m]);
        let x1 = Polynomial::from(&self.0[m..]);
        let y0 = Polynomial::from(&other.0[..m]);
        let y1 = Polynomial::from(&other.0[m..]);

        let z0 = x0.multiply_karatsuba_rec(&y0);
        let z2 = x1.multiply_karatsuba_rec(&y1);
        let z1 = &(&(&x0 + &x1).multiply_karatsuba_rec(&(&y0 + &y1)) - &z0) - &z2;

        let z1 = z1.multiply_by_monic_monomial(m);
        let z2 = z2.multiply_by_monic_monomial(n);

        &(&z0 + &z1) + &z2
    }

    pub fn multiply_by_monomial(&self, degree: usize, coefficient: R) -> Polynomial<R> {
        let zeros = vec![R::zero(); degree];
        let result = Vec::from_iter(
            zeros
                .into_iter()
                .chain(self.0.iter().map(|&coeff| coeff * coefficient)),
        );
        Polynomial::from(result)
    }

    pub fn multiply_by_monic_monomial(&self, degree: usize) -> Polynomial<R> {
        self.multiply_by_monomial(degree, R::one())
    }
}

impl<R: Ring> fmt::Debug for Polynomial<R> {
    /// How to display a polynomial (higher degree coefficient first)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format_string = self
            .0
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
        write!(f, "{}", format_string)
    }
}

impl<R: Ring> Add for &Polynomial<R> {
    type Output = Polynomial<R>;

    fn add(self, other: &Polynomial<R>) -> Self::Output {
        let mut result = vec![R::zero(); self.degree().max(other.degree()) + 1];
        for (i, &ai) in self.0.iter().enumerate() {
            result[i] = result[i] + ai;
        }
        for (i, &bi) in other.0.iter().enumerate() {
            result[i] = result[i] + bi;
        }
        Polynomial(result)
    }
}

impl<R: Ring> Sub for &Polynomial<R> {
    type Output = Polynomial<R>;

    fn sub(self, other: &Polynomial<R>) -> Self::Output {
        let mut result = vec![R::zero(); self.degree().max(other.degree()) + 1];
        for (i, &ai) in self.0.iter().enumerate() {
            result[i] = result[i] + ai;
        }
        for (i, &bi) in other.0.iter().enumerate() {
            result[i] = result[i] - bi;
        }
        Polynomial(result)
    }
}

impl<R: Ring> Mul for &Polynomial<R> {
    type Output = Polynomial<R>;

    fn mul(self, other: &Polynomial<R>) -> Self::Output {
        // self.multiply_schoolbook(&other)
        self.multiply_karatsuba(&other)
    }
}

impl<R: Ring> From<&[R]> for Polynomial<R> {
    fn from(coefficients: &[R]) -> Self {
        Self(coefficients.to_vec())
    }
}

impl<R: Ring> From<Vec<R>> for Polynomial<R> {
    fn from(coefficients: Vec<R>) -> Self {
        Self(coefficients)
    }
}

impl<R: Ring> PartialEq for Polynomial<R> {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(&a, &b)| (a == R::zero() && b == R::zero()) || a == b)
    }
}
