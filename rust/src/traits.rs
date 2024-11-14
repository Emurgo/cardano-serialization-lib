pub trait NoneOrEmpty {
    fn is_none_or_empty(&self) -> bool;
}

pub trait EmptyToNone: Sized {
   fn empty_to_none(self) -> Option<Self>;
}

impl<T: NoneOrEmpty> NoneOrEmpty for &T {
    fn is_none_or_empty(&self) -> bool {
        (*self).is_none_or_empty()
    }
}

impl<T: NoneOrEmpty> NoneOrEmpty for Option<T> {
    fn is_none_or_empty(&self) -> bool {
        match &self {
            Some(x) => x.is_none_or_empty(),
            None => true,
        }
    }
}

impl<T: NoneOrEmpty> EmptyToNone for T {
    fn empty_to_none(self) -> Option<Self> {
        if self.is_none_or_empty() {
            None
        } else {
            Some(self)
        }
    }
}