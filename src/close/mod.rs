use std::collections::{HashMap};

enum Accumulator {
    Summ(f64),
    SummNone(Option<f64>),
    Last(Option<f64>),
}

impl Accumulator {
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

    fn as_float(&self) -> Option<f64> {
        match self {
            &Accumulator::Summ(ref inner) => Some(*inner),
            &Accumulator::SummNone(ref inner) => *inner,
            &Accumulator::Last(ref inner) => *inner,
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