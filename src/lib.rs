mod arithmetic;
mod highvar;
mod lowvar;
mod unsafelib;

pub use highvar::*;
pub use lowvar::*;
use std::convert::From;
use std::marker::PhantomData;

pub struct High;

#[derive(Default)]
pub struct Variable<S, T> {
    #[doc(hidden)]
    pub data: T,
    #[doc(hidden)]
    pub status: PhantomData<S>,
}

impl<S, T> Variable<S, T> {
    pub fn new(data: T) -> Self {
        Variable {
            data,
            status: PhantomData,
        }
    }
}

impl<S, T> From<T> for Variable<S, T> {
    fn from(data: T) -> Self {
        Variable {
            data,
            status: PhantomData,
        }
    }
}

impl<S, T: Clone> Clone for Variable<S, T> {
    fn clone(&self) -> Self {
        Variable {
            data: self.data.clone(),
            status: PhantomData,
        }
    }
}

impl<S, T: Copy> Copy for Variable<S, T> {}

impl<T> From<LowVar<T>> for HighVar<T> {
    fn from(l: LowVar<T>) -> Self {
        HighVar {
            data: l.data,
            status: PhantomData,
        }
    }
}

//TODO implement Product, Drop, SliceIndex, Sum
