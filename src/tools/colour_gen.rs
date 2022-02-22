#[derive(Clone)]
pub struct ColourIter<'a, C>
{
    current: usize,
    colours: &'a[C],
}

impl<'a, C: 'a> ColourIter<'a, C> {
    pub fn new(colours: &'a[C]) -> Self {
        ColourIter {
            current: 0,
            colours,
        }
    }
}

impl<'a, C: 'a> Iterator for ColourIter<'a, C>
where
    C: Clone,
{
    type Item = &'a C;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_index = self.current + 1;

        if next_index >= self.colours.len() {
            next_index = 0;
        }

        self.current = next_index;

        Some(&self.colours[next_index])
    }
}
