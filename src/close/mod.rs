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
}