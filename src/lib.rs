pub trait Accumulator {
    type Input;
    type Output;

    fn notify(&mut self, value: Self::Input) -> ();
    fn result(&self) -> Self::Output;
}

// Numeric

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

struct Last {
    inner: Option<f64>,
}

impl Accumulator for Last {
    type Input = f64;
    type Output = Option<f64>;

    fn notify(&mut self, value: Self::Input) {
        self.inner = Some(value)
    }

    fn result(&self) -> Self::Output {
        self.inner
    }
}

// List

struct ListAvg {
    inner: Vec<f64>
}

impl Accumulator for ListAvg {
    type Input = f64;
    type Output = Option<f64>;

    fn notify(&mut self, value: Self::Input) {
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
