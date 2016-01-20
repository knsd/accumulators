pub trait Accumulator {
    type Input;
    type Output;

    fn notify(&mut self, value: &Self::Input) -> ();
    fn result(&self) -> Self::Output;
}
}

