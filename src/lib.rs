pub trait Accumulator {
    type Item;

    fn notify(&mut self, value: Self::Item) -> ();
}