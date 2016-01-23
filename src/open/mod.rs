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

// List

#[derive(Debug)]
struct ListAvg {
    inner: Vec<f64>
}

impl Accumulator for ListAvg {
    type Input = f64;
    type Output = Option<f64>;

    fn add(&mut self, value: Self::Input) {
        self.inner.push(value)
    }

    fn result(&self) -> Self::Output {
        if self.inner.is_empty() {
            return None
        } else {
            return Some(self.inner.iter().fold(0.0, |a, b| a + b) / self.inner.len() as f64)
        }
    }
}

type AccContainer<A> = HashMap<String, A>;

struct Container {
    summ: AccContainer<Summ>,
    summ_none: AccContainer<SummNone>,
    last: AccContainer<Last>,
    min: AccContainer<Min>,
    max: AccContainer<Max>,
    list_avg: AccContainer<ListAvg>,
}

impl Container {
    fn notify_summ(&mut self, name: &str, value: f64) {
        let maybe_acc = self.summ.get_mut(name);
        let acc = match maybe_acc {
            Some(acc) => acc,
            None => {
                let acc = Summ { inner: 0.0 };
                self.summ.insert(name.to_string(), acc);
                panic!("foo");
            },
        };
        // acc.notify(value);
    }
}