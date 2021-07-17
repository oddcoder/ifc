use crate::Variable;
use std::error::Error;
use std::fmt::{
    self, Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex,
};
use std::hash::{Hash, Hasher};

pub struct Low;
pub type LowVar<T> = Variable<Low, T>;

impl<T: Debug> Debug for LowVar<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("LowVar").field("data", &self.data).finish()
    }
}

impl<T: Display> Display for LowVar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<T: Error> Error for LowVar<T> {}

/// We cannot have Hash for HighVar because hash is
/// not cryptographically secure
impl<T: Hash> Hash for LowVar<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

/// We cannot have Hash for HighVar because hash is
/// not cryptographically secure
impl<T: Hasher> Hasher for LowVar<T> {
    fn finish(&self) -> u64 {
        self.data.finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        self.data.write(bytes)
    }
}

impl<T: Binary> Binary for LowVar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Binary::fmt(&self.data, f)
    }
}

impl<T: LowerExp> LowerExp for LowVar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerExp::fmt(&self.data, f)
    }
}

impl<T: LowerHex> LowerHex for LowVar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(&self.data, f)
    }
}

impl<T: Octal> Octal for LowVar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Octal::fmt(&self.data, f)
    }
}

impl<T: UpperExp> UpperExp for LowVar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        UpperExp::fmt(&self.data, f)
    }
}

impl<T: UpperHex> UpperHex for LowVar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        UpperHex::fmt(&self.data, f)
    }
}

pub trait Inner {
    type Output;
    fn inner(self) -> Self::Output;
}

impl<T> Inner for LowVar<T> {
    type Output = T;
    fn inner(self) -> T {
        self.data
    }
}

impl<'a, T> Inner for &'a LowVar<T> {
    type Output = &'a T;
    fn inner(self) -> &'a T {
        &self.data
    }
}

impl<'a, T> Inner for &'a mut LowVar<T> {
    type Output = &'a mut T;
    fn inner(self) -> &'a mut T {
        &mut self.data
    }
}
