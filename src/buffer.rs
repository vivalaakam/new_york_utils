use std::ops::{Add, AddAssign, Sub, SubAssign};

pub struct Buffer<T> {
    vals: Vec<T>,
    pub(crate) len: usize,
    pushes: usize,
    pub(crate) index: usize,
    pub(crate) sum: T,
}

impl<T> Buffer<T>
where
    T: Default + Clone + Copy + Add + Sub + SubAssign + AddAssign,
{
    pub fn new(len: usize) -> Self {
        Buffer {
            vals: vec![T::default(); len],
            len,
            pushes: 0,
            index: 0,
            sum: T::default(),
        }
    }

    pub fn push(&mut self, val: T) {
        if self.pushes >= self.len {
            self.sum -= self.vals.get(self.index).expect("").clone();
        }

        self.sum += val;
        self.vals[self.index] = val;
        self.pushes += 1;
        self.index += 1;
        if self.index >= self.len {
            self.index = 0;
        }
    }

    pub fn qpush(&mut self, val: T) {
        self.vals[self.index] = val;
        self.index += 1;
        if self.index >= self.len {
            self.index = 0;
        }
    }

    pub fn get(&self, ind: usize) -> T {
        self.vals
            .get((self.index + self.len - 1 + ind) % self.len)
            .expect("")
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_f64() {
        let mut buffer = Buffer::new(2);
        buffer.push(1.);
        assert_eq!(1.0, buffer.sum);
        buffer.push(1.5);
        assert_eq!(2.5, buffer.sum);
        buffer.push(2.5);
        assert_eq!(4.0, buffer.sum);
    }

    #[test]
    fn buffer_u64() {
        let mut buffer: Buffer<u64> = Buffer::new(2);
        buffer.push(1);
        assert_eq!(1, buffer.sum);
        buffer.push(2);
        assert_eq!(3, buffer.sum);
        buffer.push(3);
        assert_eq!(5, buffer.sum);
    }
}
