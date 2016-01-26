#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # variance
//!
//! Helpers for establishing the variance of lifetimes and type parameters.
//!

use std::marker::PhantomData;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

/// A marker for forcing T to be considered invariant.
// *mut T is invariant in T
pub struct Invariant<T>(PhantomData<*mut T>);

// TODO: We have to explicitly list all OIBITS here, which is not really possible.
unsafe impl<T> Send for Invariant<T> {}
unsafe impl<T> Sync for Invariant<T> {}

impl<T> Invariant<T> {
    /// Create a new Invariant marker instance.
    ///
    /// All instances of Invariant with the same T are equivalent.
    #[inline]
    pub fn new() -> Self { Invariant(PhantomData) }
}

impl<T> Default for Invariant<T> {
    #[inline]
    fn default() -> Self { Invariant::new() }
}

impl<T> Copy for Invariant<T> {}

impl<T> Clone for Invariant<T> {
    #[inline]
    fn clone(&self) -> Self { Invariant::new() }
}

impl<T> fmt::Debug for Invariant<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Invariant Type Marker")
    }
}

impl<T> PartialEq<Invariant<T>> for Invariant<T> {
    #[inline]
    fn eq(&self, _: &Self) -> bool { true }

    #[inline]
    fn ne(&self, _: &Self) -> bool { false }
}

impl<T> PartialOrd<Invariant<T>> for Invariant<T> {
    #[inline]
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }

    #[inline]
    fn lt(&self, _: &Self) -> bool { false }

    #[inline]
    fn le(&self, _: &Self) -> bool { true }

    #[inline]
    fn gt(&self, _: &Self) -> bool { false }

    #[inline]
    fn ge(&self, _: &Self) -> bool { true }
}

impl<T> Eq for Invariant<T> {}

impl<T> Ord for Invariant<T> {
    #[inline]
    fn cmp(&self, _: &Self) -> Ordering { Ordering::Equal }
}

impl<T> Hash for Invariant<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        ().hash(state)
    }
}

/// A marker for forcing `'id` to be considered invariant.
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct InvariantLifetime<'id>(Invariant<&'id ()>);

impl<'id> InvariantLifetime<'id> {
    /// Create a new InvariantLifetime marker instance.
    ///
    /// All instances of InvariantLifetime with the same lifetime are equivalent.
    #[inline]
    pub fn new() -> Self { InvariantLifetime::default() }
}

fn _assert_bounds() {
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}
    fn is_derived<T: Copy + Clone + fmt::Debug + PartialEq + Eq + PartialOrd + Ord + Hash + Default>() {}

    struct Nothing;

    is_send::<Invariant<*mut ()>>();
    is_sync::<Invariant<*mut ()>>();
    is_derived::<Invariant<Nothing>>();

    fn lifetime<'a>() {
        is_send::<InvariantLifetime<'a>>();
        is_sync::<InvariantLifetime<'a>>();
        is_derived::<InvariantLifetime<'a>>();
    }
}

