use crate::{High, LowVar, Variable};

pub type HighVar<T> = Variable<High, T>;
use std::marker::PhantomData;

pub trait UnsafeInner {
    type Output;
    unsafe fn inner(self) -> Self::Output;
}

impl<T> HighVar<T> {
    pub fn declassify(self) -> LowVar<T> {
        LowVar {
            data: self.data,
            status: PhantomData,
        }
    }
}

impl<T> UnsafeInner for HighVar<T> {
    type Output = T;
    unsafe fn inner(self) -> T {
        self.data
    }
}

impl<'a, T> UnsafeInner for &'a HighVar<T> {
    type Output = &'a T;
    unsafe fn inner(self) -> &'a T {
        &self.data
    }
}

impl<'a, T> UnsafeInner for &'a mut HighVar<T> {
    type Output = &'a mut T;
    unsafe fn inner(self) -> &'a mut T {
        &mut self.data
    }
}
