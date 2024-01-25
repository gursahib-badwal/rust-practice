use std::{alloc::{Layout, dealloc, alloc}, ptr};

pub struct Array<T> {
    ptr: *mut T,
    len: usize
}

impl<T> Array<T> {
    pub fn new(len: usize) -> Self {
        // unimplemented!();
        unsafe {
            let ptr_layout = Layout::new::<T>();
            let return_ptr = alloc(ptr_layout);
            //has to add as *mut T because by default the alloc return *mut u8 but we want T 
            return Array {
                ptr: return_ptr as *mut T,
                len: len
            };
        }
    }

    pub unsafe fn element_at(&self, index: usize) -> &T {
        // unimplemented!();
        let array_len = self.len;
        let array_ptr = self.ptr;

        if index >= array_len {
            panic!("Index is out of bounds!!");
        }

        let pointer = array_ptr.add(index);

        return &*pointer;
    }

    pub fn set(&self, index: usize, elem:T) {
        // unimplemented!();
        let array_len = self.len;
        let array_ptr = self.ptr;
        if index >= array_len {
            panic!("Index is out of bounds!!");
        } 

        unsafe {
            let pointer = array_ptr.add(index);
            ptr::write(pointer, elem);
        }
    }
}

impl<T> Drop for Array<T> { 
    fn drop(&mut self) {
        // unimplemented!();
        let array_ptr = self.ptr as *mut u8;
        unsafe {
            let ptr_layout = Layout::new::<T>();
            dealloc(array_ptr, ptr_layout);
        }
    }
}