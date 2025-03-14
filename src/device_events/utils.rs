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
        if self.len() == 0 {
            return;
        }
        let mut i = self.len() - 1;
        while i > 0 {
            if filter(&mut self[i]) {
                self.remove(i);
            } else {
                i -= 1;
            }
        }
    }
}
