use std::ops::Range;

/// Types:
/// Type 1 - an adapter that returns a single constant, such as a u64
/// Type 2 - an adapter that returns a single value of the same type as the iterator
/// Type 3 - an adapter that returns another iterator.


// Step 1: Define a trait.
pub trait IteratorAdapters: Iterator {
    /// Create an InRange structure which can be used as an iterator.
    /// Based on itertools.step().
    /// This is a Type 3 iterator adapter (it returns another iterator).
    fn in_range(self, r: Range<u64>) -> InRange<Self>
        where Self: Iterator<Item = u64> + Sized
    {
        InRange { iter: self, r, have_skipped: false }
    }
}

// Step 2: Use that trait to add extension methods to the Iterator trait.
impl<T: ?Sized> IteratorAdapters for T where T: Iterator { }


// Step 3: Define the structs required by our adapters (if any, these are only
// required if the adapters need to manage state).
pub struct InRange<I>
    where I:Iterator<Item=u64>
{
    iter: I,
    r: Range<I::Item>,
    have_skipped: bool
}

/// Step 4: The InRange struct is itself an iterator (it returns multiple items) so we need
/// to `impl Iterator` for it.
impl<I> Iterator for InRange<I>
    where I:Iterator<Item=u64>
{
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        // We have to return a single item on each call.
        let start = self.r.start;
        let end = self.r.end;

        if !self.have_skipped {
            let i = self.iter.find(|&p| p >= start);
            self.have_skipped = true;
            return i;
        }

        if let Some(n) = self.iter.next() {
            if n < end {
                return Some(n);
            }
        }
        None
    }
}




#[cfg(test)]
mod tests {
    use super::IteratorAdapters;

    #[test]
    fn in_range_works() {
        let v = (0..10).collect::<Vec<_>>();
        let result = v.into_iter().in_range(0..3).collect::<Vec<_>>();
        assert_eq!(result, vec![0, 1, 2]);

        let v = (0..10).collect::<Vec<_>>();
        let result = v.into_iter().in_range(4..8).collect::<Vec<_>>();
        assert_eq!(result, vec![4, 5, 6, 7]);

        // TODO: This doesn't work.
//        let v = (0..10).collect::<Vec<_>>();
//        let result = v.iter().in_range(4..8).collect::<Vec<_>>();
//        assert_eq!(result, vec![4, 5, 6, 7]);
    }
}