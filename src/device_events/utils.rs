//! Utils.

/// This is a placeholder for the unstable feature.
pub trait DrainFilter<T> {
    fn drain_filter<F>(&mut self, filter: F)
    where
        F: FnMut(&mut T) -> bool;
}

impl<T> DrainFilter<T> for Vec<T> {
    fn drain_filter<F>(&mut self, filter: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let mut filter = filter;
        let mut i = 0;
        while i < self.len() {
            if filter(&mut self[i]) {
                self.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
