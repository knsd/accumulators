pub trait Add<Input> {
    fn add(&mut self, value: Input) -> ();
}

// Numeric

struct Summ {
    inner: f64,
}

impl Add<f64> for Summ {
    fn add(&mut self, value: f64) {
        self.inner = self.inner + value
    }
}

struct SummNone {
    inner: Option<f64>,
}

impl Add<f64> for SummNone {
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

impl Add<f64> for Last {
    fn add(&mut self, value: f64) {
        self.inner = Some(value)
    }
}

struct Min {
    inner: Option<f64>,
}

impl Add<f64> for Min {

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

impl Add<f64> for Max {

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

struct ListAvg {
    inner: Vec<f64>
}

impl Add<f64> for ListAvg {
    fn add(&mut self, value: f64) {
        self.inner.push(value)
    }
}
