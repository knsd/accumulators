use std::collections::{HashMap};

pub trait Accumulator {
    fn new() -> Self;
    fn add(&mut self, value: f64) -> ();
}

// Numeric

pub struct Summ {
    inner: f64,
}

impl Accumulator for Summ {
    fn new() -> Self {
        Summ { inner: 0.0 }
    }

    fn add(&mut self, value: f64) {
        self.inner = self.inner + value
    }
}

struct SummNone {
    inner: Option<f64>,
}

impl Accumulator for SummNone {
    fn new() -> Self {
        SummNone { inner: None }
    }

    fn add(&mut self, value: f64) {
        self.inner = match self.inner {
            None => Some(value),
            Some(inner) => Some(inner + value),
        }
    }
}

struct Last {
    inner: Option<f64>,
}

impl Accumulator for Last {
    fn new() -> Self {
        Last { inner: None }
    }
    fn add(&mut self, value: f64) {
        self.inner = Some(value)
    }
}

struct Min {
    inner: Option<f64>,
}

impl Accumulator for Min {
    fn new() -> Self {
        Min { inner: None }
    }

    fn add(&mut self, value: f64) {
        match self.inner {
            None => self.inner = Some(value),
            Some(inner) => if inner > value {
                self.inner = Some(value)
            },
        }
    }
}

struct Max {
    inner: Option<f64>,
}

impl Accumulator for Max {
    fn new() -> Self {
        Max { inner: None }
    }

    fn add(&mut self, value: f64) {
        match self.inner {
            None => self.inner = Some(value),
            Some(inner) => if inner < value {
                self.inner = Some(value)
            },
        }
    }
}
