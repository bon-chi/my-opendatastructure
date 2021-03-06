#[allow(unused_assignments)]
use crate::my_structure::{List, SSet, USet};
#[derive(Debug)]
pub struct MyList<T>(Vec<T>);
#[derive(Debug)]
pub struct MyUSet<T: Eq>(Vec<T>);
#[derive(Debug)]
pub struct MySSet<T: Eq + Ord>(Vec<T>);

impl<T> MyList<T> {
    pub fn new(vec: Vec<T>) -> Self {
        MyList(vec)
    }
}

impl<T> List<T> for MyList<T> {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    fn set(&mut self, index: usize, x: T) -> Option<T> {
        Some(std::mem::replace(&mut self.0[index], x))
    }

    fn add(&mut self, index: usize, x: T) {
        self.0.insert(index, x)
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        Some(self.0.remove(index))
    }
}

impl<T: Eq> MyUSet<T> {
    pub fn new(vec: Vec<T>) -> Self {
        let mut unique_vec = Vec::with_capacity(vec.len());
        for e in vec {
            if !unique_vec.contains(&e) {
                unique_vec.push(e);
            }
        }
        MyUSet(unique_vec)
    }
}

impl<T: Eq> USet<T> for MyUSet<T> {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn add(&mut self, x: T) -> bool {
        if self.0.contains(&x) {
            return false;
        }
        self.0.push(x);
        true
    }

    fn remove(&mut self, x: &T) -> Option<T> {
        if let Some(index) = self.0.iter().position(|e| e == x) {
            Some(self.0.remove(index))
        } else {
            None
        }
    }
    fn find(&self, x: &T) -> Option<&T> {
        if let Some(index) = self.0.iter().position(|e| e == x) {
            self.0.get(index)
        } else {
            None
        }
    }
}

impl<T: Eq + Ord> MySSet<T> {
    pub fn new(vec: Vec<T>) -> Self {
        let unique_vec = Vec::with_capacity(vec.len());
        let mut my_set = MySSet(unique_vec);
        for e in vec {
            my_set.add(e);
        }
        my_set
    }
}

impl<T: Eq + Ord> SSet<T> for MySSet<T> {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn add(&mut self, x: T) -> bool {
        if self.0.contains(&x) {
            false
        } else {
            if let Some(index) = self.0.iter().position(|e| e <= &x) {
                self.0.insert(index, x)
            } else {
                self.0.push(x);
            }
            true
        }
    }

    fn remove(&mut self, x: &T) -> Option<T> {
        if let Some(index) = self.0.iter().position(|e| e == x) {
            Some(self.0.remove(index))
        } else {
            None
        }
    }
    fn find(&self, x: &T) -> Option<&T> {
        if let Some(index) = self.0.iter().position(|e| e <= x) {
            self.0.get(index)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::my_structure::naive_implementation::MyList;
    use crate::my_structure::List;

    #[test]
    fn list_size() {
        let my_list = MyList::new(vec![1, 2, 3]);
        assert_eq!(my_list.size(), 3);
    }

    #[test]
    fn list_get() {
        let my_list = MyList::new(vec![1, 2, 3]);
        assert_eq!(my_list.get(1), Some(&2));
    }

    #[test]
    fn list_set() {
        let mut my_list = MyList::new(vec![1, 2, 3]);
        my_list.set(2, 4);
        assert_eq!(my_list.get(2), Some(&4));
    }

    #[test]
    fn list_add() {
        let mut my_list = MyList::new(vec![1, 2, 3]);
        my_list.add(1, 4);
        assert_eq!(my_list.size(), 4);
        assert_eq!(my_list.get(1), Some(&4));
        assert_eq!(my_list.get(2), Some(&2));
    }

    #[test]
    fn list_remove() {
        let mut my_list = MyList::new(vec![1, 2, 3]);
        assert_eq!(my_list.remove(1), Some(2));
        assert_eq!(my_list.size(), 2);
        assert_eq!(my_list.get(1), Some(&3));
    }

    use crate::my_structure::naive_implementation::MyUSet;
    use crate::my_structure::USet;
    #[derive(Debug)]
    struct Tuple(usize, usize);
    impl PartialEq for Tuple {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl Eq for Tuple {}

    #[test]
    fn uset_size() {
        let my_uset = MyUSet::new(vec![1, 2, 3]);
        assert_eq!(my_uset.size(), 3);
    }

    #[test]
    fn uset_add() {
        let mut my_uset = MyUSet::new(vec![1, 2, 3]);
        assert_eq!(my_uset.add(4), true);
        assert_eq!(my_uset.add(4), false);
        assert_eq!(my_uset.size(), 4);
        let mut my_uset = MyUSet::new(vec![Tuple(1, 11), Tuple(2, 22), Tuple(3, 33)]);
        assert_eq!(my_uset.add(Tuple(4, 44)), true);
        assert_eq!(my_uset.add(Tuple(4, 40)), false);
    }

    #[test]
    fn uset_remove() {
        let mut my_uset = MyUSet::new(vec![1, 2, 3]);
        assert_eq!(my_uset.remove(&2), Some(2));
        assert_eq!(my_uset.size(), 2);
        assert_eq!(my_uset.remove(&4), None);
        assert_eq!(my_uset.size(), 2);
        let mut my_uset = MyUSet::new(vec![Tuple(1, 11), Tuple(2, 22), Tuple(3, 33)]);
        assert_eq!(my_uset.remove(&Tuple(2, 0)), Some(Tuple(2, 22)));
        assert_eq!(my_uset.size(), 2);
        assert_eq!(my_uset.remove(&Tuple(4, 40)), None);
    }

    #[test]
    fn uset_find() {
        let my_uset = MyUSet::new(vec![Tuple(1, 11), Tuple(2, 22), Tuple(3, 33)]);
        assert_eq!(my_uset.find(&Tuple(2, 0)), Some(&Tuple(2, 22)));
        assert_eq!(my_uset.find(&Tuple(4, 40)), None);
    }

    use crate::my_structure::naive_implementation::MySSet;
    use crate::my_structure::SSet;
    #[derive(Debug)]
    struct OTuple(usize, usize);
    impl PartialEq for OTuple {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl Eq for OTuple {}
    use std::cmp::Ordering;
    impl Ord for OTuple {
        fn cmp(&self, other: &OTuple) -> Ordering {
            self.0.cmp(&other.0)
        }
    }
    impl PartialOrd for OTuple {
        fn partial_cmp(&self, other: &OTuple) -> Option<Ordering> {
            Some(self.0.cmp(&other.0))
        }
    }
    #[test]
    fn sset_test() {
        let mut my_sset = MySSet::new(vec![OTuple(2, 22), OTuple(5, 55), OTuple(1, 11)]);
        my_sset.add(OTuple(4, 4));
        assert_eq!(my_sset.find(&OTuple(2, 0)), Some(&OTuple(2, 22)));
        assert_eq!(my_sset.find(&OTuple(4, 40)), Some(&OTuple(4, 4)));
        assert_eq!(my_sset.find(&OTuple(5, 5)), Some(&OTuple(5, 55)));
    }
}
