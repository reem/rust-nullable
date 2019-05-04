//! Nullable, lifetime-tracked pointers.

use std::marker;
use std::mem;

pub struct Nullable<'a, T: 'a> {
    lifetime: marker::PhantomData<&'a ()>,
    data: *const T
}

unsafe impl<T: Send> Send for Nullable<'static, T> {}
unsafe impl<'a, T: Sync> Sync for Nullable<'a, T> {}

impl<'a, T> Nullable<'a, T> {
    pub unsafe fn new(t: *const T) -> Nullable<'a, T> {
        Nullable {
            lifetime: marker::PhantomData,
            data: t
        }
    }

    pub fn from_ref(t: &'a T) -> Nullable<'a, T> {
        unsafe { Nullable::new(t) }
    }

    pub unsafe fn deref(&self) -> &'a T {
        mem::transmute(self.data)
    }
}

pub struct NullableMut<'a, T: 'a> {
    lifetime: marker::PhantomData<&'a ()>,
    data: *mut T
}

impl<'a, T> NullableMut<'a, T> {
    pub unsafe fn new(t: *mut T) -> NullableMut<'a, T> {
        NullableMut {
            lifetime: marker::PhantomData,
            data: t
        }
    }

    pub fn from_mut(t: &'a mut T) -> NullableMut<'a, T> {
        unsafe { NullableMut::new(t) }
    }

    pub unsafe fn deref(&self) -> &'a T {
        mem::transmute(self.data)
    }

    pub unsafe fn deref_mut(&mut self) -> &'a mut T {
        mem::transmute(self.data)
    }
}

