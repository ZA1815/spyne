use std::{mem::MaybeUninit, sync::atomic::{AtomicUsize, Ordering}};

#[repr(align(64))]
struct RingIndex(AtomicUsize);

pub struct RingBuffer<T> {
    buf: Box<[MaybeUninit<T>]>,
    capacity: usize,
    write_index: RingIndex,
    read_index: RingIndex
}

impl<T> RingBuffer<T> {
    pub fn new(size: usize) -> Self {
        let mut v: Vec<MaybeUninit<T>> = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(MaybeUninit::uninit());
        }
        
        Self {
            buf: v.into_boxed_slice(),
            capacity: size,
            write_index: RingIndex(AtomicUsize::new(0)),
            read_index: RingIndex(AtomicUsize::new(0))
        }
    }
    
    pub fn enqueue(&self, item: T) -> Result<(), T> {
        let write_idx = self.write_index.0.load(Ordering::Relaxed);
        let read_idx = self.read_index.0.load(Ordering::Acquire);
        if (write_idx + 1) % self.capacity != read_idx {
            unsafe {
                let slot = self.buf.as_ptr().add(write_idx) as *mut MaybeUninit<T>;
                (*slot).write(item);
            }
            self.write_index.0.store((write_idx + 1) % self.capacity, Ordering::Release);
            
            Ok(())
        }
        else {
            Err(item)
        }
    }
    
    pub fn dequeue(&self) -> Option<T> {
        let write_idx = self.write_index.0.load(Ordering::Acquire);
        let read_idx = self.read_index.0.load(Ordering::Relaxed);
        if write_idx != read_idx {
            let item = unsafe { Some(self.buf[read_idx].assume_init_read()) };
            self.read_index.0.store((read_idx + 1) % self.capacity, Ordering::Release);
            
            item
        }
        else {
            None
        }
    }
}

impl<T> Drop for RingBuffer<T> {
    fn drop(&mut self) {
        let mut curr_idx = self.read_index.0.load(Ordering::Relaxed);
        let write_idx = self.write_index.0.load(Ordering::Relaxed);
        while curr_idx != write_idx {
            unsafe { self.buf[curr_idx].assume_init_drop() };
            if curr_idx + 1 == self.buf.len() {
                curr_idx = 0;
            }
            else {
                curr_idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::spsc::RingBuffer;

    #[test]
    fn test_ring_buffer() {
        let rb = RingBuffer::<usize>::new(4);
        rb.enqueue(5).expect("5 push failed");
        rb.enqueue(4).expect("4 push failed");
        rb.enqueue(3).expect("3 push failed");
        rb.enqueue(2).expect_err("2 push should fail");
        assert_eq!(rb.dequeue().unwrap(), 5);
        assert_eq!(rb.dequeue().unwrap(), 4);
        assert_eq!(rb.dequeue().unwrap(), 3);
        assert_eq!(rb.dequeue(), None);
    }
}