use std::mem::{MaybeUninit, replace};

/// A queue that allocates its elements on the stack. It uses a primitive array with pointers to
/// the head and tail of the queue. The queue is empty if the head and tail pointers are equal.
///
/// The queue is different from VecDeque in that attempting to add an element to a full queue will
/// simply return the element back.
#[derive(Debug)]
pub struct ArrayQueue<T, const C: usize> {
    data: [MaybeUninit<T>; C],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T, const C: usize> Default for ArrayQueue<T, C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const C: usize> ArrayQueue<T, C> {
    /// Creates a new empty ArrayQueue.
    pub fn new() -> Self {
        Self {
            // data: MaybeUninit::uninit_array(),
            data: unsafe { MaybeUninit::uninit().assume_init() },
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    /// Returns true if the queue is empty.
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of elements in the queue.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Clears the queue of all elements.
    pub fn clear(&mut self) {
        if self.is_empty() {
            return;
        }

        unsafe {
            // [x, x, T, ., ., H, x, x]
            // or
            // [., H, x, x, T, ., ., .]
            //
            // drop elements from tail to the head pointer / end of the buffer
            let end = if self.tail < self.head { self.head } else { C };

            for i in self.tail..end { // range will be empty if head is behind tail
                self.data[i].assume_init_drop();
            }

            // [., ., H, x, x, T, ., .]
            //
            // if head is behind tail, drop elements from head to start of the buffer
            if self.tail >= self.head {
                for i in 0..self.head {
                    self.data[i].assume_init_drop();
                }
            }
        }

        self.head = 0;
        self.tail = 0;
        self.len = 0;
    }

    /// Pushs an element to the queue. If the queue is full, Err(T) is returned.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len() == C {
            return Err(value);
        }

        self.head %= C;
        self.data[self.head].write(value);
        self.head += 1;
        self.len += 1;

        Ok(())
    }

    /// Pops an element from the queue.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.tail %= C;
        let res = replace(&mut self.data[self.tail], MaybeUninit::uninit());
        self.tail += 1;
        self.len -= 1;

        unsafe { Some(res.assume_init()) }
    }
}

impl<T, const C: usize> Drop for ArrayQueue<T, C> {
    fn drop(&mut self) {
        self.clear();
    }
}
