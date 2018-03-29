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
    fn in_range<T>(self, r: Range<T>) -> InRange<Self>
        where Self: Iterator<Item = T> + Sized
    {
        InRange { iter: self, r, have_skipped: false }
    }
}

// Step 2: Use that trait to add extension methods to the Iterator trait.
// The impl block itself is empty because we defaulted all the method definitions
// in the trait definition itself.
impl<T> IteratorAdapters for T where T: Iterator { }


// Step 3: Define the structs required by our adapters, if any: these are only
// required if the adapters need to manage state.
pub struct InRange<I>
    where I: Iterator
{
    iter: I,
    r: Range<I::Item>,
    have_skipped: bool
}

/// Step 4: The InRange struct is itself an iterator (it returns multiple items) so we need
/// to `impl Iterator` for it.
impl<I> Iterator for InRange<I>
    where I:Iterator,
          I::Item: PartialOrd
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if !self.have_skipped {
            self.have_skipped = true;

            while let Some(x) = self.iter.next() {
                if x >= self.r.start {
                    return Some(x);
                }
            }

            return None;
        }

        if let Some(n) = self.iter.next() {
            if n < self.r.end {
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
        let v= vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = v.into_iter().in_range(0..3).collect::<Vec<_>>();
        assert_eq!(result, vec![0, 1, 2]);

        let v= vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = v.into_iter().in_range(4..8).collect::<Vec<_>>();
        assert_eq!(result, vec![4, 5, 6, 7]);

        // TODO: This is ugly...
        let v= vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = v.iter().in_range(&4..&8).collect::<Vec<_>>();
        assert_eq!(result, vec![&4, &5, &6, &7]);
    }
}