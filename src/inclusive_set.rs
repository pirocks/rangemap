use core::fmt::{self, Debug};
use core::iter::FromIterator;
use core::ops::RangeInclusive;

#[cfg(feature = "serde1")]
use core::marker::PhantomData;
#[cfg(feature = "serde1")]
use serde::{
    de::{Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};

use crate::std_ext::*;
use crate::RangeInclusiveMap;

#[derive(Clone)]
/// A set whose items are stored as ranges bounded
/// inclusively below and above `(start..=end)`.
///
/// See [`RangeInclusiveMap`]'s documentation for more details.
///
/// [`RangeInclusiveMap`]: struct.RangeInclusiveMap.html
pub struct RangeInclusiveSet<T, StepFnsT = T> {
    rm: RangeInclusiveMap<T, (), StepFnsT>,
}

impl<T> Default for RangeInclusiveSet<T, T>
where
    T: Ord + Clone + StepLite,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> RangeInclusiveSet<T, T>
where
    T: Ord + Clone + StepLite,
{
    /// Makes a new empty `RangeInclusiveSet`.
    pub fn new() -> Self {
        Self::new_with_step_fns()
    }
}

impl<T, StepFnsT> RangeInclusiveSet<T, StepFnsT>
where
    T: Ord + Clone,
    StepFnsT: StepFns<T>,
{
    /// Makes a new empty `RangeInclusiveSet`, specifying successor and
    /// predecessor functions defined separately from `T` itself.
    ///
    /// This is useful as a workaround for Rust's "orphan rules",
    /// which prevent you from implementing `StepLite` for `T` if `T`
    /// is a foreign type.
    ///
    /// **NOTE:** This will likely be deprecated and then eventually
    /// removed once the standard library's [Step](core::iter::Step)
    /// trait is stabilised, as most crates will then likely implement [Step](core::iter::Step)
    /// for their types where appropriate.
    ///
    /// See [this issue](https://github.com/rust-lang/rust/issues/42168)
    /// for details about that stabilization process.
    pub fn new_with_step_fns() -> Self {
        Self {
            rm: RangeInclusiveMap::new_with_step_fns(),
        }
    }

    /// Returns a reference to the range covering the given key, if any.
    pub fn get(&self, value: &T) -> Option<&RangeInclusive<T>> {
        self.rm.get_key_value(value).map(|(range, _)| range)
    }

    /// Returns `true` if any range in the set covers the specified value.
    pub fn contains(&self, value: &T) -> bool {
        self.rm.contains_key(value)
    }

    /// Gets an ordered iterator over all ranges,
    /// ordered by range.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.rm.iter(),
        }
    }

    /// Insert a range into the set.
    ///
    /// If the inserted range either overlaps or is immediately adjacent
    /// any existing range, then the ranges will be coalesced into
    /// a single contiguous range.
    ///
    /// # Panics
    ///
    /// Panics if range `start > end`.
    pub fn insert(&mut self, range: RangeInclusive<T>) {
        self.rm.insert(range, ());
    }

    /// Removes a range from the set, if all or any of it was present.
    ///
    /// If the range to be removed _partially_ overlaps any ranges
    /// in the set, then those ranges will be contracted to no
    /// longer cover the removed range.
    ///
    /// # Panics
    ///
    /// Panics if range `start > end`.
    pub fn remove(&mut self, range: RangeInclusive<T>) {
        self.rm.remove(range);
    }

    /// Gets an iterator over all the maximally-sized ranges
    /// contained in `outer_range` that are not covered by
    /// any range stored in the set.
    ///
    /// The iterator element type is `RangeInclusive<T>`.
    pub fn gaps<'a>(&'a self, outer_range: &'a RangeInclusive<T>) -> Gaps<'a, T, StepFnsT> {
        Gaps {
            inner: self.rm.gaps(outer_range),
        }
    }
}

/// An iterator over the ranges of a `RangeInclusiveSet`.
///
/// This `struct` is created by the [`iter`] method on [`RangeInclusiveSet`]. See its
/// documentation for more.
///
/// [`iter`]: RangeInclusiveSet::iter
pub struct Iter<'a, T> {
    inner: super::inclusive_map::Iter<'a, T, ()>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a RangeInclusive<T>;

    fn next(&mut self) -> Option<&'a RangeInclusive<T>> {
        self.inner.next().map(|(range, _)| range)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// An owning iterator over the ranges of a `RangeInclusiveSet`.
///
/// This `struct` is created by the [`into_iter`] method on [`RangeInclusiveSet`]
/// (provided by the `IntoIterator` trait). See its documentation for more.
///
/// [`into_iter`]: IntoIterator::into_iter
pub struct IntoIter<T> {
    inner: super::inclusive_map::IntoIter<T, ()>,
}

impl<T> IntoIterator for RangeInclusiveSet<T> {
    type Item = RangeInclusive<T>;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.rm.into_iter(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = RangeInclusive<T>;
    fn next(&mut self) -> Option<RangeInclusive<T>> {
        self.inner.next().map(|(range, _)| range)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// We can't just derive this automatically, because that would
// expose irrelevant (and private) implementation details.
// Instead implement it in the same way that the underlying BTreeSet does.
impl<T: Debug> Debug for RangeInclusiveSet<T>
where
    T: Ord + Clone + StepLite,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T> FromIterator<RangeInclusive<T>> for RangeInclusiveSet<T>
where
    T: Ord + Clone + StepLite,
{
    fn from_iter<I: IntoIterator<Item = RangeInclusive<T>>>(iter: I) -> Self {
        let mut range_set = RangeInclusiveSet::new();
        range_set.extend(iter);
        range_set
    }
}

impl<T> Extend<RangeInclusive<T>> for RangeInclusiveSet<T>
where
    T: Ord + Clone + StepLite,
{
    fn extend<I: IntoIterator<Item = RangeInclusive<T>>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |range| {
            self.insert(range);
        })
    }
}

#[cfg(feature = "serde1")]
impl<T> Serialize for RangeInclusiveSet<T>
where
    T: Ord + Clone + StepLite + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.rm.btm.len()))?;
        for range in self.iter() {
            seq.serialize_element(&(&range.start(), &range.end()))?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde1")]
impl<'de, T> Deserialize<'de> for RangeInclusiveSet<T>
where
    T: Ord + Clone + StepLite + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RangeInclusiveSetVisitor::new())
    }
}

#[cfg(feature = "serde1")]
struct RangeInclusiveSetVisitor<T> {
    marker: PhantomData<fn() -> RangeInclusiveSet<T>>,
}

#[cfg(feature = "serde1")]
impl<T> RangeInclusiveSetVisitor<T> {
    fn new() -> Self {
        RangeInclusiveSetVisitor {
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "serde1")]
impl<'de, T> Visitor<'de> for RangeInclusiveSetVisitor<T>
where
    T: Ord + Clone + StepLite + Deserialize<'de>,
{
    type Value = RangeInclusiveSet<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("RangeInclusiveSet")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut range_inclusive_set = RangeInclusiveSet::new();
        while let Some((start, end)) = access.next_element()? {
            range_inclusive_set.insert(start..=end);
        }
        Ok(range_inclusive_set)
    }
}

/// An iterator over all ranges not covered by a `RangeInclusiveSet`.
///
/// This `struct` is created by the [`gaps`] method on [`RangeInclusiveSet`]. See its
/// documentation for more.
///
/// [`gaps`]: RangeInclusiveSet::gaps
pub struct Gaps<'a, T, StepFnsT> {
    inner: crate::inclusive_map::Gaps<'a, T, (), StepFnsT>,
}

// `Gaps` is always fused. (See definition of `next` below.)
impl<'a, T, StepFnsT> core::iter::FusedIterator for Gaps<'a, T, StepFnsT>
where
    T: Ord + Clone,
    StepFnsT: StepFns<T>,
{
}

impl<'a, T, StepFnsT> Iterator for Gaps<'a, T, StepFnsT>
where
    T: Ord + Clone,
    StepFnsT: StepFns<T>,
{
    type Item = RangeInclusive<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{format, vec, vec::Vec};

    trait RangeInclusiveSetExt<T> {
        fn to_vec(&self) -> Vec<RangeInclusive<T>>;
    }

    impl<T> RangeInclusiveSetExt<T> for RangeInclusiveSet<T>
    where
        T: Ord + Clone + StepLite,
    {
        fn to_vec(&self) -> Vec<RangeInclusive<T>> {
            self.iter().cloned().collect()
        }
    }

    #[test]
    fn empty_set_is_empty() {
        let range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
        assert_eq!(range_set.to_vec(), vec![]);
    }

    #[test]
    fn insert_into_empty_map() {
        let mut range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
        range_set.insert(0..=50);
        assert_eq!(range_set.to_vec(), vec![0..=50]);
    }

    #[test]
    fn remove_partially_overlapping() {
        let mut range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
        range_set.insert(0..=50);
        range_set.remove(25..=75);
        assert_eq!(range_set.to_vec(), vec![0..=24]);
    }

    #[test]
    fn gaps_between_items_floating_inside_outer_range() {
        let mut range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ●-● ◌ ◌ ◌
        range_set.insert(5..=6);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ●-● ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(2..=3);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------------◆ ◌
        let outer_range = 1..=8;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield gaps at start, between items,
        // and at end.
        assert_eq!(gaps.next(), Some(1..=1));
        assert_eq!(gaps.next(), Some(4..=4));
        assert_eq!(gaps.next(), Some(7..=8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    ///
    /// impl Debug
    ///

    #[test]
    fn set_debug_repr_looks_right() {
        let mut set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();

        // Empty
        assert_eq!(format!("{:?}", set), "{}");

        // One entry
        set.insert(2..=5);
        assert_eq!(format!("{:?}", set), "{2..=5}");

        // Many entries
        set.insert(7..=8);
        set.insert(10..=11);
        assert_eq!(format!("{:?}", set), "{2..=5, 7..=8, 10..=11}");
    }

    // impl Serialize

    #[cfg(feature = "serde1")]
    #[test]
    fn serialization() {
        let mut range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆---◆ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(1..=3);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆---◆ ◌ ◌
        range_set.insert(5..=7);
        let output = serde_json::to_string(&range_set).expect("Failed to serialize");
        assert_eq!(output, "[[1,3],[5,7]]");
    }

    // impl Deserialize

    #[cfg(feature = "serde1")]
    #[test]
    fn deserialization() {
        let input = "[[1,3],[5,7]]";
        let range_set: RangeInclusiveSet<u32> =
            serde_json::from_str(input).expect("Failed to deserialize");
        let reserialized = serde_json::to_string(&range_set).expect("Failed to re-serialize");
        assert_eq!(reserialized, input);
    }
}
