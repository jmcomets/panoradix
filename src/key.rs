pub trait KeyComponent: Ord + Eq + Clone {}
impl<T: Ord + Eq + Clone> KeyComponent for T {}

pub trait Key: ToOwned {
    type Component: KeyComponent;

    fn as_slice<'a>(&'a self) -> &'a [Self::Component];

    fn from_vec(v: Vec<Self::Component>) -> Self::Owned;
}

impl Key for str {
    type Component = u8;

    fn as_slice<'a>(&'a self) -> &'a [u8] {
        self.as_bytes()
    }

    fn from_vec(v: Vec<u8>) -> String {
        unsafe {
            String::from_utf8_unchecked(v)
        }
    }
}

impl<T: KeyComponent> Key for [T] {
    type Component = T;

    fn as_slice<'a>(&'a self) -> &'a [T] {
        self
    }

    fn from_vec(v: Vec<T>) -> Vec<T> {
        v
    }
}

/// A key that can be inserted in both [`RadixSet`] and [`RadixMap`].
///
/// These keys should be equivalent to slices of `T: Ord + Eq + Clone`.
///
/// [`RadixSet`]: struct.RadixSet.html
/// [`RadixMap`]: struct.RadixMap.html
pub trait ExtensibleKey: ToOwned {
    /// A single component of the key. Note that it should be `Ord + Eq + Clone`.
    type Component: KeyComponent;

    /// Get a slice of key components to integrate the key in a radix tree.
    fn as_slice<'a>(&'a self) -> &'a [Self::Component];

    /// Given a merged vec of components, build an owned.
    fn from_vec(v: Vec<Self::Component>) -> Self::Owned;
}

impl<T: ExtensibleKey> Key for T {
    type Component = T::Component;

    fn as_slice<'a>(&'a self) -> &'a [Self::Component] {
        self.as_slice()
    }

    fn from_vec(v: Vec<Self::Component>) -> Self::Owned {
        T::from_vec(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_be_extended() {
        #[derive(Clone)]
        struct Wrapper(Vec<()>);

        impl ExtensibleKey for Wrapper {
            type Component = ();

            fn as_slice<'a>(&'a self) -> &'a [()] {
                &self.0
            }

            fn from_vec(v: Vec<Self::Component>) -> Self::Owned {
                Wrapper(v)
            }
        }

        use set::RadixSet;

        let mut s = RadixSet::<Wrapper>::new();
        s.insert(&Wrapper(vec![()]));
    }
}
