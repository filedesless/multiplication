use std::{fmt, ops::Mul};

use num::Num;

pub trait Ring = Num + Copy + Clone + fmt::Debug;

/// Polynomial over a ring R represented by its vector of coefficients.
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

    fn multiply(&self, other: &Polynomial<R>) -> Polynomial<R> {
        let mut result = vec![R::zero(); self.degree() + other.degree() + 1];
        for (i, &ai) in self.0.iter().enumerate() {
            for (j, &bj) in other.0.iter().enumerate() {
                result[i + j] = result[i + j] + (ai * bj);
            }
        }
        Polynomial(result)
    }
}

impl<R: Ring> fmt::Debug for Polynomial<R> {
    /// How to display a polynomial (higher degree coefficient first)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &coeff) in self.0.iter().enumerate().rev() {
            if !coeff.is_zero() {
                if i == 0 {
                    write!(f, "{:?}", coeff)?;
                } else if i == 1 {
                    write!(f, "{:?}x", coeff)?;
                } else {
                    write!(f, "{:?}x^{}", coeff, i)?;
                }
                if i != 0 {
                    write!(f, " + ")?;
                }
            }
        }
        Ok(())
    }
}

impl<R: Ring> Mul for &Polynomial<R> {
    type Output = Polynomial<R>;

    fn mul(self, other: &Polynomial<R>) -> Self::Output {
        self.multiply(&other)
    }
}
