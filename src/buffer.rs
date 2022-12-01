use std::ops::{Add, AddAssign, Sub, SubAssign};

pub struct Buffer<T> {
    vals: Vec<T>,
    len: usize,
    pushes: usize,
    index: usize,
    sum: T,
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

    /**
    get buffer value

    # Examples

    ```
    use new_york_utils::Buffer;

    let mut buffer = Buffer::new(2);
    buffer.push(1);
    buffer.push(2);
    assert_eq!(buffer.get_value(0), 2);
    assert_eq!(buffer.get_value(1), 1);

    buffer.push(3);
    assert_eq!(buffer.get_value(0), 3);
    assert_eq!(buffer.get_value(1), 2);
    assert_eq!(buffer.get_value(2), 3);
    ```
     */
    pub fn get_value(&self, ind: usize) -> T {
        self.vals
            .get((self.index + self.len - 1 + ind) % self.len)
            .expect("")
            .clone()
    }

    /**
    get buffer value

    # Examples

    ```
    use new_york_utils::Buffer;

    let mut buffer = Buffer::new(2);
    buffer.push(1);
    buffer.push(2);
    assert_eq!(buffer.get_sum(), 3);

    buffer.push(3);
    assert_eq!(buffer.get_sum(), 5);
    ```
     */
    pub fn get_sum(&self) -> T {
        self.sum
    }
}
