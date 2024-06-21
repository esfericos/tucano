use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElideDebug<T>(pub T);

impl<T> From<T> for ElideDebug<T> {
    fn from(inner: T) -> Self {
        ElideDebug(inner)
    }
}

impl<T> ElideDebug<T> {
    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn into(self) -> T {
        self.0
    }
}

impl<T> fmt::Debug for ElideDebug<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<...>")
    }
}
