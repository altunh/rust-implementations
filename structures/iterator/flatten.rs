pub trait Flattener
where
    Self: Iterator + Sized,
    Self::Item: IntoIterator,
{
    fn flatten(self) -> Flatten<Self> {
        Flatten::new(self)
    }
}

impl<I> Flattener for I
where
    I: Iterator + Sized,
    I::Item: IntoIterator,
{
}

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    iter: FlattenCompat<I, <I::Item as IntoIterator>::IntoIter>,
}

impl<I> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    pub fn new(iter: I) -> Self {
        Flatten {
            iter: FlattenCompat::new(iter),
        }
    }
}

impl<I, U> Iterator for Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator<IntoIter = U, Item = U::Item>,
    U: Iterator,
{
    type Item = U::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.iter.count()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.iter.last()
    }
}

impl<I, U> DoubleEndedIterator for Flatten<I>
where
    I: DoubleEndedIterator,
    I::Item: IntoIterator<IntoIter = U, Item = U::Item>,
    U: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<I> Default for Flatten<I>
where
    I: Default + Iterator,
    I::Item: IntoIterator,
{
    fn default() -> Self {
        Flatten::new(Default::default())
    }
}

impl<I, U> Clone for Flatten<I>
where
    I: Clone + Iterator,
    I::Item: IntoIterator<IntoIter = U, Item = U::Item>,
    U: Clone + Iterator,
{
    fn clone(&self) -> Self {
        Flatten {
            iter: self.iter.clone(),
        }
    }
}

#[derive(Clone)]
pub struct FlattenCompat<I, U> {
    iter: I,
    frontiter: Option<U>,
    backiter: Option<U>,
}

impl<I, U> FlattenCompat<I, U>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        FlattenCompat {
            iter,
            frontiter: None,
            backiter: None,
        }
    }
}

impl<I, U> Iterator for FlattenCompat<I, U>
where
    I: Iterator,
    I::Item: IntoIterator<IntoIter = U, Item = U::Item>,
    U: Iterator,
{
    type Item = U::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let element @ Some(_) = and_then_or_clear(&mut self.frontiter, Iterator::next) {
                return element;
            }
            match self.iter.next() {
                Some(inner) => self.frontiter = Some(inner.into_iter()),
                None => return and_then_or_clear(&mut self.backiter, Iterator::next),
            }
        }
    }
}

impl<I, U> DoubleEndedIterator for FlattenCompat<I, U>
where
    I: DoubleEndedIterator,
    I::Item: IntoIterator<IntoIter = U, Item = U::Item>,
    U: DoubleEndedIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let elt @ Some(_) = and_then_or_clear(&mut self.backiter, DoubleEndedIterator::next_back) {
                return elt;
            }
            match self.iter.next_back() {
                Some(inner) => self.backiter = Some(inner.into_iter()),
                None => return and_then_or_clear(&mut self.frontiter, DoubleEndedIterator::next_back),
            }
        }
    }
}

#[inline]
fn and_then_or_clear<T, U>(opt: &mut Option<T>, f: impl FnOnce(&mut T) -> Option<U>) -> Option<U> {
    let x = f(opt.as_mut()?);
    if x.is_none() {
        *opt = None;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let iter = std::iter::empty::<Vec<()>>();
        assert_eq!(flatten(iter).count(), 0);
    }

    #[test]
    fn empty_wide() {
        let iter: Vec<Vec<()>> = vec![vec![], vec![], vec![]];
        assert_eq!(flatten(iter).count(), 0);
    }

    #[test]
    fn one() {
        let iter = std::iter::once(vec![0]);
        assert_eq!(flatten(iter).count(), 1);
    }

    #[test]
    fn one_wide() {
        let iter = vec![vec![0, 1, 2]];
        assert_eq!(flatten(iter).count(), 3);
    }

    #[test]
    fn two() {
        let iter = std::iter::once(vec![0, 1]);
        assert_eq!(flatten(iter).count(), 2);
    }

    #[test]
    fn two_wide() {
        let iter = vec![vec![1, 2], vec![3, 4, 5]];
        assert_eq!(flatten(iter).count(), 5);
    }

    #[test]
    fn reverse() {
        let iter = std::iter::once(vec![1, 2]);
        assert_eq!(flatten(iter).rev().collect::<Vec<_>>(), vec![2, 1]);
    }

    #[test]
    fn reverse_wide() {
        let iter = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(flatten(iter).rev().collect::<Vec<_>>(), vec![4, 3, 2, 1]);
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn inf() {
        let mut iter = flatten((0..).map(|i| 0..i));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn deep() {
        assert_eq!(flatten(flatten(vec![vec![vec![0, 1]]])).count(), 2);
    }

    #[test]
    fn flattener() {
        let flattener = Flattener::flatten(vec![vec![0, 1], vec![2, 3]].into_iter());
        assert_eq!(flattener.count(), 4);
    }
}
