use std::collections::{HashMap};
use std::collections::hash_state::DefaultState;
use std::default::Default;
use std::hash::Hasher;

pub struct FnvHasher(u64);

impl Default for FnvHasher {

    #[inline]
    fn default() -> FnvHasher {
        FnvHasher(0xcbf29ce484222325)
    }
}

impl Hasher for FnvHasher {

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let FnvHasher(mut hash) = *self;

        for byte in bytes.iter() {
            hash = hash ^ (*byte as u64);
            hash = hash.wrapping_mul(0x100000001b3);
        }

        *self = FnvHasher(hash);
    }
}

pub trait Accumulator {
    fn add(&mut self, value: f64) -> ();
}

// Numeric

pub struct Summ {
    inner: f64,
}

impl Accumulator for Summ {

    #[inline]
    fn add(&mut self, value: f64) {
        self.inner = self.inner + value
    }
}

struct SummNone {
    inner: Option<f64>,
}

impl Accumulator for SummNone {

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

    #[inline]
    fn add(&mut self, value: f64) {
        self.inner = Some(value)
    }
}

struct Min {
    inner: Option<f64>,
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
}

struct Max {
    inner: Option<f64>,
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
}

struct Average {
    sum: f64,
    count: usize,
}

impl Accumulator for Average {

    fn add(&mut self, value: f64) {
        self.sum = self.sum + value;
        self.count = self.count + 1;
    }
}

trait Container {
    #[inline]
    fn add_data(&mut self, data: &HashMap<String, f64>);
}

struct WrappedAccumulator {
    accumulator: Box<Accumulator>,
    updated_in_last_iteration: bool,
}

impl Accumulator for WrappedAccumulator {
    #[inline]
    fn add(&mut self, value: f64) {
        self.accumulator.add(value);
        self.updated_in_last_iteration = true;
    }
}

struct SimpleContainer {
    accumulators: HashMap<String, WrappedAccumulator, DefaultState<FnvHasher>>,
}

impl SimpleContainer {
    fn new() -> Self {
        let fnv = DefaultState::<FnvHasher>::default();
        SimpleContainer {
            accumulators: HashMap::with_hash_state(fnv),
        }
    }
}

impl Container for SimpleContainer {

    #[inline]
    fn add_data(&mut self, data: &HashMap<String, f64>) {
        for (name, value) in data {
            let shlould_insert = {
                let maybe_acc = self.accumulators.get_mut(name);   // FIXME: excess double hashing
                match maybe_acc {
                    Some(acc) => {
                        acc.add(*value);
                        false
                    },
                    None => true,
                }
            };

            if shlould_insert {
                let acc: Box<Accumulator> = match name.as_bytes()[name.len() - 1] {
                    b's' => Box::new(Summ {inner: 0.0} ),
                    b'n' => Box::new(SummNone {inner: None} ),
                    b'l' => Box::new(Last {inner: None} ),
                    _ => continue,
                };
                let mut wrapped_acc = WrappedAccumulator { accumulator: acc, updated_in_last_iteration: false };
                wrapped_acc.add(*value);
                self.accumulators.insert(name.to_string(), wrapped_acc);
            }
        }
    }
}
