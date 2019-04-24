use crate::my_structure::List;
struct ArrayStack<T> {
    n: usize,
    a: Vec<T>,
}

impl<T: Clone> ArrayStack<T> {
    fn resize(&mut self) {
        let mut b = Vec::with_capacity(std::cmp::max(1, 2 / self.n));
        for i in 0..self.n {
            b[i] = self.a[i].clone();
        }
        self.a = b;
    }
}

impl<T: Clone> List<T> for ArrayStack<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size() {
            return None;
        }
        Some(&self.a[index])
    }

    fn set(&mut self, index: usize, x: T) -> Option<T> {
        if index >= self.size() {
            return None;
        }
        let y = self.a[index].clone();
        self.a[index] = x;
        Some(y)
    }

    fn add(&mut self, index: usize, x: T) {
        if self.n + 1 >= self.a.capacity() {
            self.resize();
        }
        for j in ((index + 1)..=(self.n)).rev() {
            self.a[j] = self.a[j - 1].clone();
        }
        self.a[index] = x;
        self.n += 1;
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size() {
            return None;
        }
        let x = self.a[index].clone();
        for j in index..(self.n) {
            self.a[j] = self.a[j + 1].clone();
        }
        self.n -= 1;
        if self.a.capacity() >= 3 * self.n {
            self.resize();
        }
        Some(x)
    }
}

struct FastArrayStack<T> {
    n: usize,
    a: Vec<T>,
}

impl<T> FastArrayStack<T> {
    fn resize(&mut self) {
        let mut b = Vec::with_capacity(std::cmp::max(1, 2 / self.n));
        self.a.swap_with_slice(&mut b[0..self.n]);
        self.a = b;
    }
}

impl<T: Clone> List<T> for FastArrayStack<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size() {
            return None;
        }
        Some(&self.a[index])
    }

    fn set(&mut self, index: usize, x: T) -> Option<T> {
        if index >= self.size() {
            return None;
        }
        let mut x_slice = vec![x];
        self.a[index..=index].swap_with_slice(&mut x_slice[0..=0]);
        x_slice.into_iter().last()
    }

    fn add(&mut self, index: usize, x: T) {
        if self.n + 1 >= self.a.capacity() {
            self.resize();
        }
        self.a.insert(index, x);
        self.n += 1;
    }
    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size() {
            return None;
        }
        let x = self.a[index].clone();
        for j in index..(self.n) {
            self.a[j] = self.a[j + 1].clone();
        }
        self.n -= 1;
        if self.a.len() >= 3 * self.n {
            self.resize();
        }
        Some(x)
    }
}
