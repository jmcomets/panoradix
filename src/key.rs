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
