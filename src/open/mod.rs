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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn add(&mut self, value: f64) {
        match self.inner {
            None => self.inner = Some(value),
            Some(inner) => if inner < value {
                self.inner = Some(value)
            },
        }
    }
}

struct Average {
    sum: f64,
    count: usize,
}

impl Accumulator for Average {
    fn new() -> Self {
        Average { sum: 0.0, count: 0 }
    }

    fn add(&mut self, value: f64) {
        self.sum = self.sum + value;
        self.count = self.count + 1;
    }
}

type AccContainer<A> = HashMap<String, A>;

struct Container {
    summ: AccContainer<Summ>,
    summ_none: AccContainer<SummNone>,
    last: AccContainer<Last>,
    min: AccContainer<Min>,
    max: AccContainer<Max>,
    average: AccContainer<Average>,
}

trait ContainerNotify<Acc> {
    fn notify(&mut self, name: &str, value: f64);
}

macro_rules! make_notify {
    ($acc: ty, $field: ident) => {
        impl ContainerNotify<$acc> for Container {

            #[inline]
            fn notify(&mut self, name: &str, value: f64) {
                let acc = self.$field.entry(name.to_string()).or_insert_with(|| Accumulator::new() );  // FIXME: excess clone
                acc.add(value)
            }
        }
    }
}

make_notify!(Summ, summ);
make_notify!(SummNone, summ_none);
make_notify!(Last, last);
make_notify!(Min, min);
make_notify!(Max, max);
make_notify!(Average, average);
