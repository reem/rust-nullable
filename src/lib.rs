#![deny(missing_docs, warnings)]

//! Nullable, lifetime-tracked pointers.

use std::kinds::marker;

pub struct Nullable<'a, T: 'a> {
    lifetime: marker::ContravariantLifetime<'a>,
    data: *const T
}

unsafe impl<T: Send> Send for Nullable<'static, T> {}
unsafe impl<'a, T: Sync> Sync for Nullable<'a, T> {}

impl<'a, T> Nullable<'a, T> {
    pub unsafe fn new(t: *const T) -> Nullable<'a, T> {
        Nullable {
            lifetime: marker::ContravariantLifetime,
            data: t
        }
    }

    pub fn from_ref(t: &'a T) -> Nullable<'a, T> {
        Nullable::new(t)
    }

    pub unsafe fn deref(&self) -> &'a T {
        mem::transmute(self.data)
    }
}

pub struct NullableMut<'a, T: 'a> {
    lifetime: marker::ContravariantLifetime<'a>,
    data: *mut T
}

impl<'a, T> NullableMut<'a, T> {
    pub unsafe fn new(t: *mut T) -> NullableMut<'a, T> {
        NullableMut {
            lifetime: marker::ContravariantLifetime,
            data: t
        }
    }

    pub fn from_mut(t: &'a mut T) -> NullableMut<'a, T> {
        NullableMut::new(t)
    }

    pub unsafe fn deref(&self) -> &'a T {
        mem::transmute(self.data)
    }

    pub unsafe fn deref_mut(&mut self) -> &'a mut T {
        mem::transmute(self.data)
    }
}

