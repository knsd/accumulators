use std::collections::{HashMap};
use std::collections::hash_state::DefaultState;
use std::default::Default;
use std::hash::Hasher;

pub trait Accumulator {
    fn add(&mut self, value: f64) -> ();
    fn mul(&mut self, value: &Accumulator);

    fn as_float(&self) -> Option<f64>;
}

// Numeric

pub struct Summ {
    pub inner: f64,
}

impl Accumulator for Summ {

    #[inline]
    fn add(&mut self, value: f64) {
        self.inner = self.inner + value
    }

    #[inline]
    fn mul(&mut self, value: &Accumulator) {
        value.as_float().map(|v| self.add(v));
    }

    fn as_float(&self) -> Option<f64> {
        Some(self.inner)
    }
}

pub struct SummNone {
    pub inner: Option<f64>,
}

impl Accumulator for SummNone {

    #[inline]
    fn add(&mut self, value: f64) {
        self.inner = match self.inner {
            None => Some(value),
            Some(inner) => Some(inner + value),
        }
    }

    #[inline]
    fn mul(&mut self, value: &Accumulator) {
        value.as_float().map(|v| self.add(v));
    }

    fn as_float(&self) -> Option<f64> {
        self.inner
    }
}

pub struct Last {
    pub inner: Option<f64>,
}

impl Accumulator for Last {

    #[inline]
    fn add(&mut self, value: f64) {
        self.inner = Some(value)
    }

    #[inline]
    fn mul(&mut self, value: &Accumulator) {
        value.as_float().map(|v| self.add(v));
    }

    fn as_float(&self) -> Option<f64> {
        self.inner
    }
}

pub struct Min {
    pub inner: Option<f64>,
}

impl Accumulator for Min {

    #[inline]
    fn add(&mut self, value: f64) {
        match self.inner {
            None => self.inner = Some(value),
            Some(inner) => if inner > value {
                self.inner = Some(value)
            },
        }
    }

    #[inline]
    fn mul(&mut self, value: &Accumulator) {
        value.as_float().map(|v| self.add(v));
    }

    fn as_float(&self) -> Option<f64> {
        self.inner
    }
}

pub struct Max {
    pub inner: Option<f64>,
}

impl Accumulator for Max {

    #[inline]
    fn add(&mut self, value: f64) {
        match self.inner {
            None => self.inner = Some(value),
            Some(inner) => if inner < value {
                self.inner = Some(value)
            },
        }
    }

    #[inline]
    fn mul(&mut self, value: &Accumulator) {
        value.as_float().map(|v| self.add(v));
    }

    fn as_float(&self) -> Option<f64> {
        self.inner
    }
}

pub struct Average {
    sum: f64,
    count: usize,
}

impl Accumulator for Average {

    fn add(&mut self, value: f64) {
        self.sum = self.sum + value;
        self.count = self.count + 1;
    }

    #[inline]
    fn mul(&mut self, value: &Accumulator) {
        value.as_float().map(|v| self.add(v));
    }

    fn as_float(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum / self.count as f64)
        }

    }
}
