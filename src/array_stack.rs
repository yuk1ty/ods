extern crate core;

pub struct ArrayStack<T> {
    buf: Vec<T>,
    len: usize,
}

impl<T> ArrayStack<T> {
    pub fn new(size: usize) -> Self {
        ArrayStack {
            buf: Vec::with_capacity(size),
            len: size,
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        assert!(i <= self.len);
        self.buf.get::<usize>(i)
    }

    pub fn set(&mut self, i: usize, x: T) {
        assert!(i <= self.len);
        self.buf[i] = x;
    }

    pub fn add(&mut self, i: usize, x: T) {
        assert!(i <= self.len);

        self.buf.insert(i, x);
        self.len = self.buf.len();
    }

    pub fn into_vec(self) -> Vec<T> {
        self.buf
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

pub struct FastArrayStack<T: Default> {
    buf: Vec<T>,
    len: usize
}

impl<T: Default> FastArrayStack<T> {
    pub fn new(size: usize) -> Self {
        FastArrayStack {
            buf: Vec::with_capacity(size),
            len: 0,
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        assert!(i <= self.len);
        self.buf.get::<usize>(i)
    }

    pub fn set(&mut self, i: usize, x: T) {
        assert!(i <= self.len);

        unsafe {
            let p = self.buf.as_mut_ptr().offset(i as isize);
            core::ptr::write(p, x);
            self.len = self.len + 1;
        }
    }

    pub fn add(&mut self, i: usize, element: T) {
        let len = self.len;
        assert!(i <= len);

        if len == self.buf.capacity() {
            self.buf.resize_default(len * 2);
        }

        unsafe {
            {
                let p = self.buf.as_mut_ptr().offset(i as isize);
                core::ptr::copy(p, p.offset(1), len - i);
                core::ptr::write(p, element);
            }
            self.len = len + 1;
            self.buf.set_len(self.len);
            self.buf.truncate(self.len);
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.buf
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
