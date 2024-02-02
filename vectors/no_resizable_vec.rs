extern crate alloc;
use core::ptr::NonNull;
use core::mem;
use std::{ptr, ops::{Deref, DerefMut, Index, IndexMut}};
use alloc::alloc::{Layout, alloc, dealloc};
use nondet::Nondet;
use std::io::Read;
use anchor_lang::prelude::borsh::maybestd::io::Write;
use anchor_lang::{AnchorSerialize, AnchorDeserialize};
use borsh::{BorshDeserialize, BorshSerialize};

/////////////////////////
/// Raw Vec
/////////////////////////
struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

impl<T> RawVec<T> {
    fn new_zero_sized() -> Self {
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }

    fn new(capacity: usize) -> Self {

        // ZSTs have no memory allocation
        if mem::size_of::<T>() == 0 || capacity == 0 {
            return RawVec::new_zero_sized();
        }

        let layout: Layout = Layout::array::<T>(capacity).unwrap_or_else(|_| panic!("capacity overflow"));
        let ptr: NonNull<T> = unsafe {
            let ptr: *mut u8 = alloc(layout);
            NonNull::new_unchecked(ptr as *mut T)
        };

        Self {
            ptr,
            cap: capacity,
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        // ZSTs have no memory allocation
        if mem::size_of::<T>() == 0 {
            return;
        }

        let layout: Layout = Layout::array::<T>(self.cap).unwrap();
        unsafe {
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

/////////////////////////
/// NoResizableVec
/////////////////////////
pub struct NoResizableVec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> NoResizableVec<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: RawVec::new(capacity),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.buf.cap
    }

    pub fn push(&mut self, value: T) {
        assert!(self.buf.cap > self.len);
        unsafe {
            let end: *mut T = self.buf.ptr.as_ptr().add(self.len);
            end.write(value);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                let end: *mut T = self.buf.ptr.as_ptr().add(self.len);
                Some(end.read())
            }
        }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        assert!(self.buf.cap > self.len);
        assert!(index < self.len);
        unsafe {
            let ptr: *mut T = self.buf.ptr.as_ptr().add(index);
            ptr.copy_to(ptr.add(1), self.len - index);
            ptr.write(value);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);
        unsafe {
            self.len -= 1;
            let ptr: *mut T = self.buf.ptr.as_ptr().add(index);
            let value: T = ptr.read();
            ptr.add(1).copy_to(ptr, self.len - index);
            value
        }
    }

    pub fn find(&self, value: &T) -> Option<usize> where T: Ord {
        for i in 0..self.len {
            unsafe {
                let ptr: *mut T = self.buf.ptr.as_ptr().add(i);
                if ptr.read() == *value {
                    return Some(i);
                }
            }
        }
        None
    }
}


impl<T> Drop for NoResizableVec<T> {
    fn drop(&mut self) {
        // do nothiing
    }

}

//////////////////
/// Other traits:
/// - Clone
/// - Deref
/// - DerefMut
/// - Index
/// - IndexMut
/// - BosrhSerialize
/// - BorshDeserialize
//////////////////

impl<T> Clone for NoResizableVec<T> {
    fn clone(&self) -> Self {
        let raw_vec = RawVec::new(self.capacity());
        let new_vec = Self {
            buf: raw_vec,
            len: self.len(),
        };

        unsafe {
            ptr::copy_nonoverlapping(
                self.buf.ptr.as_ptr(),
                new_vec.buf.ptr.as_ptr(),
                self.len()
            );
        }

        new_vec
    }
}

impl<T> Deref for NoResizableVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            core::slice::from_raw_parts(self.buf.ptr.as_ptr(), self.len())
        }
    }
}

impl<T> DerefMut for NoResizableVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            core::slice::from_raw_parts_mut(self.buf.ptr.as_ptr(), self.len())
        }
    }
}

impl<T> Index<usize> for NoResizableVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        assert!(index < self.len());
        unsafe {
            if mem::size_of::<T>() == 0 {
                NonNull::<T>::dangling().as_ref()
            } else {
                &*self.buf.ptr.as_ptr().add(index)
            }
        }
    }
}

impl<T> IndexMut<usize> for NoResizableVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.len());
        unsafe {
            if mem::size_of::<T>() == 0 {
                NonNull::<T>::dangling().as_mut()
            } else {
                &mut *self.buf.ptr.as_ptr().add(index)
            }
        }
    }
}

impl<T: AnchorSerialize> BorshSerialize for NoResizableVec<T> {
    fn serialize<W: Write>(&self, writer: &mut W) -> borsh::maybestd::io::Result<()> {
        let len = usize::try_from(self.len()).map_err(|_| std::io::ErrorKind::InvalidInput)?;
        writer.write_all(&len.to_le_bytes())?;

        let cap = usize::try_from(self.buf.cap).map_err(|_| std::io::ErrorKind::InvalidInput)?;
        writer.write_all(&cap.to_le_bytes())?;

        // serialize the vector
        unsafe {
            let slice = core::slice::from_raw_parts(self.buf.ptr.as_ptr(), self.len);
            slice.serialize(writer)
        }
    }
}

/// We need to fix the capacity of the vector.
/// However, this number depends on the specific verification task.
impl<T:Nondet + AnchorDeserialize> BorshDeserialize for NoResizableVec<T> {
    fn deserialize(buf: &mut &[u8]) -> borsh::maybestd::io::Result<Self> {
        // Deserialize the len and capacity
        let len = usize::deserialize(buf)?;
        let capacity = usize::deserialize(buf)?;

        // Currently, we don't rely on serialization / deserialization of
        // the vector elements for verification tasks.
        // So, we can just create a nondet vector of same len and capacity
        let mut vec = NoResizableVec::<T>::new(capacity);
        vec.len = len;

        Ok(vec)
    }

    fn deserialize_reader<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        // Deserialize the len and capacity
        let len = usize::deserialize_reader(reader)?;
        let capacity = usize::deserialize_reader(reader)?;

        // Currently, we don't rely on serialization / deserialization of
        // the vector elements for verification tasks.
        // So, we can just create a nondet vector of same len and capacity
        let mut vec = NoResizableVec::<T>::new(capacity);
        vec.len = len;

        Ok(vec)
    }
}


/////////////////////////
/// Iterator
/////////////////////////

pub struct IntoIter<T> {
    _buf: RawVec<T>,
    start: *const T,
    end: *const T,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() != 0 {
                    let old: *const T = self.start;
                    self.start = self.start.offset(1);
                    Some(ptr::read(old))
                } else {
                    self.start = (self.start as usize + 1*mem::align_of::<T>()) as *const _;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<T>();
        let exact = (self.end as usize - self.start as usize) 
                            / if elem_size == 0 { 1*mem::align_of::<T>() } else {elem_size};
        (exact, Some(exact))
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

impl<T> IntoIterator for NoResizableVec<T> {
    type Item =T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        unsafe {
            let buf = ptr::read(&self.buf);
            let len = self.len();
            mem::forget(self);

            IntoIter {
                start: buf.ptr.as_ptr(),
                end: if mem::size_of::<T>() == 0 {
                    (buf.ptr.as_ptr() as usize + len*mem::align_of::<T>()) as *const _
                } else if len == 0 {
                    buf.ptr.as_ptr()
                } else {
                    buf.ptr.as_ptr().add(len)
                },
                _buf: buf,
            }
        }
    }
}

/////////////////////
/// Macros
/////////////////////

#[macro_export]
macro_rules! cvt_no_resizable_vec {
    ($(values:expr),+ $(,)?) => (
        {
            let ARG_COUNT: usize = 0 $(+ { _ = $values; 1 })*;
            let mut v = vectors::no_resizable_vec::NoResizableVec::new(ARG_COUNT*2);
            $(v.push($values);)*
            v
        }
    );

    ([$($values:expr),* $(,)?]; $cap:expr) => {
        {
            let ARG_COUNT: usize = 0 $(+ { _ = $values; 1 })*;
            assert!(ARG_COUNT <= $cap);
            let mut v = vectors::no_resizable_vec::NoResizableVec::new($cap);
            $(v.push($values);)*
            v
        }
    };
}
