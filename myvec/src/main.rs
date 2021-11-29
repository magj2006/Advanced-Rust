use std::marker::PhantomData;
use std::mem;
use std::ptr::{self};

mod rawvec;
use rawvec::RawVec;

mod rawvaliter;
use rawvaliter::RawValIter;
pub struct Vec<T> {
    buf: RawVec<T>,
    len: usize,
}

unsafe impl<T: Send> Send for Vec<T> {}

unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            buf: RawVec::new(),
            len: 0,
        }
    }

    #[inline]
    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    #[inline]
    fn cap(&self) -> usize {
        self.buf.cap
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Vec<T> {
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buf.grow()
        }

        unsafe {
            ptr::write(self.ptr().add(self.len()), elem);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            self.len -= 1;

            unsafe { Some(ptr::read(self.ptr().add(self.len()))) }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bounds");

        if self.cap() == self.len() {
            self.buf.grow()
        }

        unsafe {
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len() - index,
            );

            ptr::write(self.ptr().add(index), elem);

            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out od bounds");

        unsafe {
            self.len -= 1;

            let result = ptr::read(self.ptr().add(index));

            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len() - index,
            );

            result
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

use std::ops::Deref;
impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len()) }
    }
}

use std::ops::DerefMut;
impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len()) }
    }
}

pub struct IntoIter<T> {
    _buf: RawVec<T>,
    iter: RawValIter<T>,
}
pub struct Drain<'a, T: 'a> {
    vec: PhantomData<&'a mut Vec<T>>,
    iter: RawValIter<T>,
}

impl<T> Vec<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        unsafe {
            let iter = RawValIter::new(&self);
            let buf = ptr::read(&self.buf);

            mem::forget(self);

            IntoIter {
                _buf: buf,
                iter: iter,
            }
        }
    }

    pub fn drain(&mut self) -> Drain<T> {
        unsafe {
            let iter = RawValIter::new(&self);

            self.len = 0;

            Drain {
                iter: iter,
                vec: PhantomData,
            }
        }
    }
}


impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}


impl <'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl <'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl <'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

fn main() {
    let mut xs = Vec::new();

    xs.push(1);
    xs.push(2);
    xs.push(3);
    xs.push(4);

    assert_eq!(xs.len(), 4);
}
