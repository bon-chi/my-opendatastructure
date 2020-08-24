use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

fn main() {
    // 通常のUSet
    impl ToKey for i32 {
        type Key = i32;
        fn compare_to_key(&self, key: &i32) -> bool {
            *self == *key
        }
    }
    let mut bag: Bag<i32> = Bag::new();
    println!("add 1: {:?}", bag.add(1));
    println!("find: {:?}, size: {:?}", bag.find(&1), bag.size());
    println!("add 2: {:?}", bag.add(2));
    println!("find: {:?}, size: {:?}", bag.find(&2), bag.size());
    println!("add 3: {:?}", bag.add(3));
    println!("find: {:?}, size: {:?}", bag.find(&3), bag.size());

    println!("find: {:?}", bag.find_all(&1));
    println!("find: {:?}", bag.find_all(&2));
    println!("find: {:?}", bag.find_all(&3));

    println!("remove 1: {:?}", bag.remove(&1));
    println!("find: {:?}, size: {:?}", bag.find(&1), bag.size());
    println!("remove 2: {:?}", bag.remove(&2));
    println!("find: {:?}, size: {:?}", bag.find(&2), bag.size());
    println!("remove 3: {:?}", bag.remove(&3));
    println!("find: {:?}, size: {:?}", bag.find(&3), bag.size());

    println!("add 4: {:?}", bag.add(4));
    println!("add 4: {:?}", bag.add(4));
    println!(
        "find: {:?}, find_all: {:?}, size: {:?}",
        bag.find(&4),
        bag.find_all(&4),
        bag.size()
    );

    println!("");
    // 四捨五入して一致する同じ整数値になる値はすべて同値とみなす
    impl ToKey for f32 {
        type Key = f32;
        fn compare_to_key(&self, key: &f32) -> bool {
            (*self).round() == (*key).round()
        }
    }

    let mut bag: Bag<f32> = Bag::new();
    println!("add 1: {:?}", bag.add(1.3));
    println!("add 1: {:?}", bag.add(1.4));
    println!("add 2: {:?}", bag.add(1.5));
    println!(
        "find1: {:?}, find_all1: {:?}",
        bag.find(&1.2),
        bag.find_all(&1.2)
    );
    println!("add 2: {:?}", bag.add(2.4));
    println!("add 3: {:?}", bag.add(2.5));
    println!(
        "find2: {:?}, find_all2: {:?}",
        bag.find(&2.2),
        bag.find_all(&2.2),
    );
    println!("remove 1: {:?}", bag.remove(&1.2));
    println!("remove 2: {:?}", bag.remove(&1.6));
    println!(
        "find_all1: {:?}, find_all2: {:?}",
        bag.find(&1.4),
        bag.find_all(&1.5),
    );
}

trait ToKey {
    type Key;
    fn compare_to_key(&self, key: &Self::Key) -> bool;
}

struct USet<T: ToKey>(Vec<T>);

impl<T: ToKey<Key = T>> USet<T> {
    fn new() -> USet<T> {
        USet(Vec::new())
    }
    fn size(&self) -> usize {
        self.0.len()
    }

    fn add(&mut self, x: T) -> bool {
        if self.0.len() == 0 {
            self.0.push(x);
            return true;
        }
        if let Some(_) = self.0.iter().find(|e| e.compare_to_key(&x)) {
            false
        } else {
            self.0.push(x);
            true
        }
    }
    fn remove(&mut self, x: &T) -> Option<T> {
        self.0
            .iter()
            .position(|e| e.compare_to_key(x))
            .map(|i| self.0.remove(i))
    }
    fn find(&self, x: &T) -> Option<&T> {
        self.0.iter().find(|e| e.compare_to_key(x))
    }
}
struct Bag<T: ToKey>(Vec<USet<T>>);
impl<T: ToKey<Key = T>> Bag<T> {
    fn new() -> Bag<T> {
        Bag(Vec::new())
    }
    fn size(&self) -> usize {
        self.0.iter().map(|uset| uset.size()).sum()
    }

    fn add(&mut self, x: T) -> bool {
        if let Some(uset) = self.0.iter_mut().find(|uset| (*uset).find(&x).is_none()) {
            uset.add(x);
        } else {
            let mut uset = USet::new();
            uset.add(x);
            self.0.push(uset);
        }
        true
    }

    fn remove(&mut self, x: &T) -> Option<T> {
        self.0
            .iter_mut()
            .find(|uset| (*uset).find(&x).is_some())
            .and_then(|uset| uset.remove(x))
    }

    fn find(&self, x: &T) -> Option<&T> {
        self.0.iter().find_map(|uset| uset.find(x))
    }

    fn find_all(&self, x: &T) -> Vec<&T> {
        self.0.iter().filter_map(|uset| uset.find(x)).collect()
    }
}
