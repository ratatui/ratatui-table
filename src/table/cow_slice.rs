use alloc::vec::Vec;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Deref;

/// A slice that the widget either borrows from the caller or owns.
///
/// This is not `alloc::borrow::Cow<'a, [T]>`. `Cow` stores `<[T] as ToOwned>::Owned`, and that
/// projection makes every type holding it invariant over `'a`. A `Table` with a shorter lifetime
/// then cannot borrow rows that an app keeps as `Vec<Row<'static>>`. Both variants here are
/// covariant, so a `Row<'static>` coerces to a `Row<'a>`.
#[derive(Clone)]
pub enum CowSlice<'a, T> {
    Borrowed(&'a [T]),
    Owned(Vec<T>),
}

impl<T> Deref for CowSlice<'_, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        match self {
            Self::Borrowed(items) => items,
            Self::Owned(items) => items,
        }
    }
}

impl<T> FromIterator<T> for CowSlice<'_, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::Owned(iter.into_iter().collect())
    }
}

impl<T> Default for CowSlice<'_, T> {
    fn default() -> Self {
        Self::Owned(Vec::new())
    }
}

impl<T: PartialEq> PartialEq for CowSlice<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T: Eq> Eq for CowSlice<'_, T> {}

impl<T: Hash> Hash for CowSlice<'_, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl<T: fmt::Debug> fmt::Debug for CowSlice<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{format, vec};

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn owned_and_borrowed_with_equal_items_are_equal() {
        let items = ["a", "b"];
        assert_eq!(CowSlice::Owned(vec!["a", "b"]), CowSlice::Borrowed(&items));
    }

    #[test]
    fn debug_matches_the_slice() {
        let items = ["a", "b"];
        let expected = format!("{:?}", &items[..]);
        assert_eq!(format!("{:?}", CowSlice::Owned(items.to_vec())), expected);
        assert_eq!(format!("{:?}", CowSlice::Borrowed(&items)), expected);
    }
}
