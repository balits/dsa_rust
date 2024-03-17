use std::{
    alloc::{self, Layout},
    isize, mem,
    ptr::NonNull,
};

fn panic_cap_overflow() -> ! {
    panic!("Capacity overflow");
}

pub struct Vector<T> {
    // null ptr optimization
    ptr: Option<NonNull<T>>,
    cap: usize,
    len: usize,
}

impl<T> Vector<T> {
    pub fn capacity(&self) -> usize {
        self.cap
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Self {
            ptr: None,
            cap: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            self.grow()
        }

        unsafe {
            self.ptr.unwrap().as_ptr().add(self.len).write(elem);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            self.len -= 1;
            let elem = unsafe { self.ptr.unwrap().as_ptr().add(self.len).read() };
            Some(elem)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.len() == 0 {
            None
        } else {
            let elem = unsafe { &*self.ptr.unwrap().as_ptr().add(self.len - 1) };
            Some(elem)
        }
    }

    /// Doubles the size of the vector.
    fn grow(&mut self) {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");

        // No-op: this guarantes that any further doubling only applies on vars that are <= 2^32
        if self.cap as isize > isize::MAX || self.cap as isize > isize::MAX {
            return;
        }

        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };

        // Because of the no-op earlier, this should never fail
        let new_layout = Layout::array::<T>(new_cap).unwrap();

        let new_ptr = unsafe {
            match self.ptr {
                None => alloc::alloc(new_layout),
                Some(old_ptr) => {
                    let old_layout = Layout::array::<T>(self.cap).unwrap();
                    alloc::realloc(old_ptr.as_ptr() as *mut u8, old_layout, new_layout.size())
                }
            }
        };

        // On allocation fail, `new_ptr` will be null
        self.ptr = if new_ptr.is_null() {
            alloc::handle_alloc_error(new_layout)
        } else {
            // new_ptr is isn't null here
            NonNull::new(new_ptr as *mut _)
        };
        self.cap = new_cap;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_and_push_1() {
        let mut v = Vector::new();

        v.push("Something");

        assert_eq!(v.capacity(), 1);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn push_10() {
        let mut v = Vector::new();
        let fav_books = "A Dance With Dragons A Feast For Crows Two Words";

        fav_books.split(" ").for_each(|s| v.push(s));

        assert_eq!(v.len(), 10);
        assert_eq!(v.capacity(), 16);
    }

    #[test]
    fn push_10_pop_11() {
        let mut v = Vector::new();
        let fav_books = "A Dance With Dragons A Feast For Crows Two Words";

        fav_books.split(" ").for_each(|s| v.push(s));

        assert_eq!(v.len(), 10);
        assert_eq!(v.capacity(), 16);

        fav_books
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .rev()
            .for_each(|s| assert_eq!(v.pop(), Some(*s)));

        assert_eq!(v.pop(), None);
    }

    #[test]
    fn push_10_pop_and_peek_11() {
        let mut v = Vector::new();
        let fav_books = "A Dance With Dragons A Feast For Crows Two Words";

        fav_books.split(" ").for_each(|s| v.push(s));

        assert_eq!(v.len(), 10);
        assert_eq!(v.capacity(), 16);

        fav_books
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .rev()
            .for_each(|s| {
                assert_eq!(v.peek(), Some(s));
                assert_eq!(v.pop(), Some(*s));
            });

        assert_eq!(v.peek(), None);
        assert_eq!(v.pop(), None);
    }
}
