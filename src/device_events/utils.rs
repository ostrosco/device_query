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
        if self.is_empty() {
            return;
        }

        let mut i = self.len();
        while i > 0 {
            i -= 1;
            if filter(&mut self[i]) {
                self.remove(i);
            }
        }
    }
}
