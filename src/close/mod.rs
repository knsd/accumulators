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

enum Accumulator {
    Summ(f64),
    SummNone(Option<f64>),
    Last(Option<f64>),
}

impl Accumulator {
    #[inline]
    fn add(&mut self, value: f64) {
        match self {
            &mut Accumulator::Summ(ref mut inner) => {
                *inner = *inner + value;
            },
            &mut Accumulator::SummNone(ref mut inner) => {
                *inner = Some(match inner {
                    &mut None => value,
                    &mut Some(inner) => inner + value
                })
            },
            &mut Accumulator::Last(ref mut inner) => {
                *inner = Some(value);
            }
        }
    }

    #[inline]
    fn as_float(&self) -> Option<f64> {
        match self {
            &Accumulator::Summ(ref inner) => Some(*inner),
            &Accumulator::SummNone(ref inner) => *inner,
            &Accumulator::Last(ref inner) => *inner,
        }
    }

    #[inline]
    fn accumulate(&mut self, other: &mut Accumulator) {
        match *self {
            Accumulator::Summ(_) => {
                other.as_float().map(|value| self.add(value) );
            },
            Accumulator::SummNone(_) => {
                other.as_float().map(|value| self.add(value) );
            },
            Accumulator::Last(inner) => {
                other.as_float().map(|value| self.add(value) );
            }
        }
    }
}

struct Container {
    accumulators: HashMap<String, Accumulator>
}

impl Container {

    #[inline]
    fn name_to_accumulator(name: &str) -> Option<Accumulator> {
        None
    }

    #[inline]
    fn notify(&mut self, name: &str, value: f64) {
        let shlould_insert = {
            let maybe_acc = self.accumulators.get_mut(name);   // FIXME: excess double hashing
            match maybe_acc {
                Some(acc) => {
                    acc.add(value);
                    false
                },
                None => true,
            }
        };

        if shlould_insert {
            match Self::name_to_accumulator(name) {
                None => (),
                Some(mut acc) => {
                    acc.add(value);
                    self.accumulators.insert(name.to_string(), acc);
                }
            }
        }
    }
}