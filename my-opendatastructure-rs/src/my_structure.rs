pub trait List<T> {
    fn size(&self) -> usize;
    fn get(&self, index: usize) -> Option<&T>;
    fn set(&mut self, index: usize, x: T) -> Option<T>;
    fn add(&mut self, index: usize, x: T);
    fn remove(&mut self, index: usize) -> Option<T>;
}

trait USet<T: Eq> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: &T) -> Option<T>;
    fn find(&self, x: &T) -> Option<&T>;
}

trait SSet<T: Eq + Ord> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: &T) -> Option<T>;
    fn find(&self, x: &T) -> Option<&T>;
}

pub mod naive_implementation;
