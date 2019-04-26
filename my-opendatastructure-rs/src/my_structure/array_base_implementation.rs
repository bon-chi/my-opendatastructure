use crate::my_structure::{List, Queue};
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
        if self.a.capacity() >= 3 * self.n {
            self.resize();
        }
        Some(x)
    }
}

struct ArrayQueue<T> {
    a: Vec<T>,
    j: usize,
    n: usize,
}

impl<T: Clone> ArrayQueue<T> {
    fn resize(&mut self) {
        let mut b = Vec::with_capacity(std::cmp::max(1, 2 * self.n));
        for k in 0..self.n {
            b[k] = self.a[(self.j + k) % self.a.capacity()].clone();
        }
        self.a = b;
    }
}

impl<T: Clone> Queue<T> for ArrayQueue<T> {
    fn add(&mut self, x: T) {
        if self.n + 1 >= self.a.capacity() {
            self.resize();
        }
        let len = self.a.capacity();
        self.a[(self.j + self.n) % len] = x;
        self.n += 1;
    }

    fn remove(&mut self) -> Option<T> {
        let x = self.a.swap_remove(self.j);
        self.j = (self.j + 1) % self.a.capacity();
        self.n -= 1;
        if self.a.capacity() >= 3 * self.n {
            self.resize();
        }
        Some(x)
    }
}

struct ArrayDeque<T> {
    a: Vec<T>,
    j: usize,
    n: usize,
}

impl<T> ArrayDeque<T> {
    fn resize(&mut self) {}
}

impl<T: Clone> List<T> for ArrayDeque<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size() {
            return None;
        }
        Some(&self.a[(self.j + index) % self.a.capacity()])
    }

    fn set(&mut self, index: usize, x: T) -> Option<T> {
        if index >= self.size() {
            return None;
        }
        let mut x_slice = vec![x];
        let logical_index = (self.j + index) % self.a.capacity();
        self.a[logical_index..=logical_index].swap_with_slice(&mut x_slice[0..=0]);
        x_slice.into_iter().last()
    }
    fn add(&mut self, index: usize, x: T) {
        if self.n + 1 >= self.a.capacity() {
            self.resize();
        }
        if index < self.n / 2 {
            self.j = if self.j == 0 {
                self.a.capacity() - 1
            } else {
                self.j - 1
            };
            for k in 0..(self.j - 1) {
                let physical_index = (self.j + k) % self.a.capacity();
                let next_physical_index = (self.j + k + 1) % self.a.capacity();
                self.a[physical_index] = self.a[next_physical_index].clone();
            }
        } else {
            for k in ((index + 1)..=(self.n)).rev() {
                let physical_index = (self.j + k) % self.a.capacity();
                let prev_physical_index = (self.j + k - 1) % self.a.capacity();
                self.a[physical_index] = self.a[prev_physical_index].clone();
            }
        }
        let i = (self.j + index) % self.a.capacity();
        self.a[i] = x;
        self.n += 1;
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size() {
            return None;
        }
        let x = self.a[(self.j + index) % self.a.capacity()].clone();
        if index < self.n / 2 {
            for k in (1..=index).rev() {
                let physical_index = (self.j + k) % self.a.capacity();
                let prev_physical_index = (self.j + k - 1) % self.a.capacity();
                self.a[physical_index] = self.a[prev_physical_index].clone();
            }
            self.j = (self.j + 1) % self.a.capacity();
        } else {
            for k in index..(self.n - 1) {
                let physical_index = (self.j + k) % self.a.capacity();
                let next_physical_index = (self.j + k + 1) % self.a.capacity();
                self.a[physical_index] = self.a[next_physical_index].clone();
            }
        }
        self.n -= 1;
        if 3 * self.n < self.a.capacity() {
            self.resize()
        }
        Some(x)
    }
}
