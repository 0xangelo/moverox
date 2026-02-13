pub(crate) trait BoxedIter<'a>: Iterator + 'a {
    fn boxed(self) -> Box<dyn Iterator<Item = Self::Item> + 'a>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl<'a, T: Iterator + 'a> BoxedIter<'a> for T {}
