use crate::my_structure::{List, SSet, USet};
pub struct MyList<T>(Vec<T>);

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

    fn set(&mut self, index: usize, x: T) {
        if let Some(elem) = self.0.get_mut(index) {
            *elem = x;
        }
    }

    fn add(&mut self, index: usize, x: T) {
        self.0.insert(index, x)
    }

    fn remove(&mut self, index: usize) -> T {
        self.0.remove(index)
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
        assert_eq!(my_list.remove(1), 2);
        assert_eq!(my_list.size(), 2);
        assert_eq!(my_list.get(1), Some(&3));
    }
}
