pub trait Accumulator {
    type Input;
    type Output;

    fn notify(&mut self, value: Self::Input) -> ();
    fn result(&self) -> Self::Output;
}

struct Summ {
    inner: f64,
}

impl Accumulator for Summ {
    type Input = f64;
    type Output = f64;

    fn notify(&mut self, value: Self::Input) {
        self.inner = self.inner + value
    }

    fn result(&self) -> Self::Output {
        self.inner
    }
}

struct SummNone {
    inner: Option<f64>,
}

impl Accumulator for SummNone {
    type Input = f64;
    type Output = Option<f64>;

    fn notify(&mut self, value: Self::Input) {
        self.inner = match self.inner {
            None => Some(value),
            Some(inner) => Some(inner + value),
        }
    }

    fn result(&self) -> Self::Output {
        self.inner
    }
}