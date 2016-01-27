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
                let shlould_insert = {
                    let maybe_acc = self.$field.get_mut(name);   // FIXME: excess double hashing
                    match maybe_acc {
                        Some(acc) => {
                            acc.add(value);
                            false
                        },
                        None => true,
                    }
                };

                if shlould_insert {
                    let mut acc: $acc = Accumulator::new();
                    acc.add(value);
                    self.$field.insert(name.to_string(), acc);
                }
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

impl Container {
    fn new() -> Self {
        Container {
            summ: HashMap::new(),
            summ_none: HashMap::new(),
            last: HashMap::new(),
            min: HashMap::new(),
            max: HashMap::new(),
            average: HashMap::new(),
        }
    }
}
