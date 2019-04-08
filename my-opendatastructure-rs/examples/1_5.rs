use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

fn main() {
    let mut set = HashSet::new();
    set.insert(KeyValues {
        key: 1,
        values: vec![1],
    });

    let mut bag = Bag(set);

    println!("start: {:?}", bag);
    bag.add(1);
    println!("add: {:?}", bag);
    bag.add(2);
    println!("add: {:?}", bag);
    bag.add(3);
    println!("add: {:?}", bag);
    println!("find: {:?}", bag.find(1));
    println!("find: {:?}", bag.find(2));
    println!("find: {:?}", bag.find(3));
    println!("find: {:?}", bag.find_all(1));
    println!("find: {:?}", bag.find_all(2));
    println!("find: {:?}", bag.find_all(3));
    bag.remove(1);
    println!("remove: {:?}", bag);
    bag.remove(2);
    println!("remove: {:?}", bag);
    bag.remove(3);
    println!("remove: {:?}", bag);

    let mut set = HashSet::new();
    let first = Human {
        name: String::from("a"),
        height: 150,
    };
    set.insert(KeyValues {
        key: first.clone(),
        values: vec![first],
    });
    let mut bag = Bag(set);
    let a = Human {
        name: String::from("a"),
        height: 155,
    };
    let b = Human {
        name: String::from("b"),
        height: 157,
    };
    let c = Human {
        name: String::from("c"),
        height: 160,
    };
    println!("start: {:?}", bag);
    bag.add(a.clone());
    println!("add: {:?}", bag);
    bag.add(b.clone());
    println!("add: {:?}", bag);
    bag.add(c.clone());
    println!("add: {:?}", bag);
    println!("find: {:?}", bag.find(a.clone()));
    println!("find: {:?}", bag.find(b.clone()));
    println!("find: {:?}", bag.find(c.clone()));
    println!("find: {:?}", bag.find_all(a.clone()));
    println!("find: {:?}", bag.find_all(b.clone()));
    println!("find: {:?}", bag.find_all(c.clone()));
    bag.remove(a.clone());
    println!("remove: {:?}", bag);
    bag.remove(b.clone());
    println!("remove: {:?}", bag);
    bag.remove(c.clone());
    println!("remove: {:?}", bag);
}

#[derive(Debug, Clone)]
struct Human {
    name: String,
    height: i32,
}

impl PartialEq for Human {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Human {}

impl Hash for Human {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug)]
struct Bag<T: Hash + Eq + PartialEq + Debug + Clone>(USet<T>);

type USet<T> = HashSet<KeyValues<T>>;

#[derive(Debug)]
struct KeyValues<T: Hash + Eq + PartialEq + Debug> {
    key: T,
    values: Vec<T>,
}

impl<T: Hash + Eq + PartialEq + Debug> PartialEq for KeyValues<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T: Hash + Eq + PartialEq + Debug> Eq for KeyValues<T> {}

impl<T: Hash + Eq + PartialEq + Debug> Hash for KeyValues<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<T: Hash + Eq + PartialEq + Debug + Clone> Bag<T> {
    fn add(&mut self, x: T) {
        let mut uset = KeyValues {
            key: x,
            values: Vec::new(),
        };
        if let Some(mut u_set) = self.0.take(&uset) {
            u_set.values.push(uset.key);
            self.0.replace(u_set);
        } else {
            uset.values.push(uset.key.clone());
            self.0.insert(uset);
        }
    }
    fn remove(&mut self, x: T) -> bool {
        self.0.remove(&KeyValues {
            key: x,
            values: Vec::new(),
        })
    }

    fn find(&mut self, x: T) -> Option<&T> {
        let kv = KeyValues {
            key: x,
            values: Vec::new(),
        };
        if let Some(kv) = self.0.get(&kv) {
            kv.values.first()
        } else {
            None
        }
    }
    fn find_all(&mut self, x: T) -> Option<&Vec<T>> {
        let kv = KeyValues {
            key: x,
            values: Vec::new(),
        };
        if let Some(kv) = self.0.get(&kv) {
            Some(&kv.values)
        } else {
            None
        }
    }
}
