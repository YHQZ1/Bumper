use std::ptr;

pub struct BumpAllocator {
    storage: Vec<u8>,
    offset: usize,
}

impl BumpAllocator {
    pub fn new(size: usize) -> Self {
        BumpAllocator {
            storage: vec![0u8; size],
            offset: 0,
        }
    }

    /// Allocate raw bytes with a given size and alignment.
    /// Returns None if there isn't enough space.
    pub fn alloc(&mut self, size: usize, align: usize) -> Option<&mut [u8]> {
        let current = self.offset;
        let aligned = (current + align - 1) & !(align - 1);
        let new_offset = aligned + size;

        if new_offset > self.storage.len() {
            return None;
        }

        self.offset = new_offset;
        Some(&mut self.storage[aligned..new_offset])
    }

    /// Allocate a typed value T inside the arena.
    /// Returns a mutable reference tied to the arena's lifetime.
    pub fn alloc_val<T>(&mut self, val: T) -> Option<&mut T> {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();

        let current = self.offset;
        let aligned = (current + align - 1) & !(align - 1);
        let new_offset = aligned + size;

        if new_offset > self.storage.len() {
            return None;
        }

        self.offset = new_offset;

        let ptr = self.storage[aligned..new_offset].as_mut_ptr() as *mut T;

        unsafe {
            ptr::write(ptr, val);
            Some(&mut *ptr)
        }
    }

    /// Reset the allocator — all memory is available again.
    /// Existing references must no longer be used after this.
    pub fn reset(&mut self) {
        self.offset = 0;
    }

    pub fn used(&self) -> usize {
        self.offset
    }

    pub fn capacity(&self) -> usize {
        self.storage.len()
    }

    pub fn remaining(&self) -> usize {
        self.storage.len() - self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_alloc() {
        let mut arena = BumpAllocator::new(1024);
        let slot = arena.alloc(4, 4).expect("allocation failed");
        slot[0] = 1;
        slot[1] = 2;
        slot[2] = 3;
        slot[3] = 4;
        assert_eq!(slot, &[1, 2, 3, 4]);
    }

    #[test]
    fn test_multiple_allocs() {
        let mut arena = BumpAllocator::new(1024);
        let a = arena.alloc(4, 4).expect("first alloc failed");
        a.fill(1);
        let b = arena.alloc(8, 4).expect("second alloc failed");
        b.fill(2);
        assert_eq!(arena.used(), 12);
    }

    #[test]
    fn test_out_of_memory() {
        let mut arena = BumpAllocator::new(8);
        assert!(arena.alloc(4, 4).is_some());
        assert!(arena.alloc(4, 4).is_some());
        assert!(arena.alloc(1, 1).is_none());
    }

    #[test]
    fn test_reset() {
        let mut arena = BumpAllocator::new(1024);
        arena.alloc(512, 4).expect("alloc failed");
        assert_eq!(arena.used(), 512);
        arena.reset();
        assert_eq!(arena.used(), 0);
        assert!(arena.alloc(512, 4).is_some());
    }

    #[test]
    fn test_alignment() {
        let mut arena = BumpAllocator::new(1024);
        arena.alloc(1, 1).expect("alloc failed");
        let slot = arena.alloc(4, 4).expect("aligned alloc failed");
        assert_eq!(slot.as_ptr() as usize % 4, 0);
    }

    #[test]
    fn test_alloc_val_u32() {
        let mut arena = BumpAllocator::new(1024);
        let x = arena.alloc_val(42u32).expect("alloc_val failed");
        assert_eq!(*x, 42);
        *x = 100;
        assert_eq!(*x, 100);
    }

    #[test]
    fn test_alloc_val_struct() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: f32,
            y: f32,
        }

        let mut arena = BumpAllocator::new(1024);
        let p = arena.alloc_val(Point { x: 1.5, y: 2.5 }).expect("alloc_val failed");
        assert_eq!(p.x, 1.5);
        assert_eq!(p.y, 2.5);
        p.x = 9.9;
        assert_eq!(p.x, 9.9);
    }

    #[test]
    fn test_alloc_val_out_of_memory() {
        let mut arena = BumpAllocator::new(4);
        assert!(arena.alloc_val(42u32).is_some());
        assert!(arena.alloc_val(1u8).is_none());
    }

    #[test]
    fn test_remaining() {
        let mut arena = BumpAllocator::new(1024);
        arena.alloc(100, 1).unwrap();
        assert_eq!(arena.remaining(), 924);
    }
}