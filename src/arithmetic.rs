pub use crate::Variable;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

impl<'a, S, T> Add<&'a Variable<S, T>> for Variable<S, T>
where
    T: Add<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn add(self, other: &'a Self) -> Self::Output {
        Variable::new(self.data + &other.data)
    }
}

impl<'a, S, T> Add<&'a Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: Add<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn add(self, other: Self) -> Self::Output {
        Variable::new(&self.data + &other.data)
    }
}

impl<'a, S, T> Add<Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: Add<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn add(self, other: Variable<S, T>) -> Self::Output {
        Variable::new(&self.data + other.data)
    }
}

impl<S, T> Add<Variable<S, T>> for Variable<S, T>
where
    T: Add<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn add(self, other: Self) -> Self::Output {
        Variable::new(self.data + other.data)
    }
}

impl<'a, S, T> AddAssign<&'a Variable<S, T>> for Variable<S, T>
where
    T: AddAssign<&'a T>,
{
    fn add_assign(&mut self, other: &'a Self) {
        self.data += &other.data
    }
}

impl<S, T> AddAssign<Variable<S, T>> for Variable<S, T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, other: Self) {
        self.data += other.data
    }
}

impl<'a, S, T> BitAnd<&'a Variable<S, T>> for Variable<S, T>
where
    T: BitAnd<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn bitand(self, other: &'a Self) -> Self::Output {
        Variable::new(self.data & &other.data)
    }
}

impl<'a, S, T> BitAnd<&'a Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: BitAnd<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn bitand(self, other: Self) -> Self::Output {
        Variable::new(&self.data & &other.data)
    }
}

impl<'a, S, T> BitAnd<Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: BitAnd<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn bitand(self, other: Variable<S, T>) -> Self::Output {
        Variable::new(&self.data & other.data)
    }
}

impl<S, T> BitAnd<Variable<S, T>> for Variable<S, T>
where
    T: BitAnd<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn bitand(self, other: Self) -> Self::Output {
        Variable::new(self.data & other.data)
    }
}

impl<'a, S, T> BitAndAssign<&'a Variable<S, T>> for Variable<S, T>
where
    T: BitAndAssign<&'a T>,
{
    fn bitand_assign(&mut self, other: &'a Self) {
        self.data &= &other.data
    }
}

impl<S, T> BitAndAssign<Variable<S, T>> for Variable<S, T>
where
    T: BitAndAssign<T>,
{
    fn bitand_assign(&mut self, other: Self) {
        self.data &= other.data
    }
}

impl<'a, S, T> BitOr<&'a Variable<S, T>> for Variable<S, T>
where
    T: BitOr<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn bitor(self, other: &'a Self) -> Self::Output {
        Variable::new(self.data | &other.data)
    }
}

impl<'a, S, T> BitOr<&'a Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: BitOr<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn bitor(self, other: Self) -> Self::Output {
        Variable::new(&self.data | &other.data)
    }
}

impl<'a, S, T> BitOr<Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: BitOr<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn bitor(self, other: Variable<S, T>) -> Self::Output {
        Variable::new(&self.data | other.data)
    }
}

impl<S, T> BitOr<Variable<S, T>> for Variable<S, T>
where
    T: BitOr<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn bitor(self, other: Self) -> Self::Output {
        Variable::new(self.data | other.data)
    }
}

impl<'a, S, T> BitOrAssign<&'a Variable<S, T>> for Variable<S, T>
where
    T: BitOrAssign<&'a T>,
{
    fn bitor_assign(&mut self, other: &'a Self) {
        self.data |= &other.data
    }
}

impl<S, T> BitOrAssign<Variable<S, T>> for Variable<S, T>
where
    T: BitOrAssign<T>,
{
    fn bitor_assign(&mut self, other: Self) {
        self.data |= other.data
    }
}

impl<'a, S, T> BitXor<&'a Variable<S, T>> for Variable<S, T>
where
    T: BitXor<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn bitxor(self, other: &'a Self) -> Self::Output {
        Variable::new(self.data ^ &other.data)
    }
}

impl<'a, S, T> BitXor<&'a Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: BitXor<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn bitxor(self, other: Self) -> Self::Output {
        Variable::new(&self.data ^ &other.data)
    }
}

impl<'a, S, T> BitXor<Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: BitXor<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn bitxor(self, other: Variable<S, T>) -> Self::Output {
        Variable::new(&self.data ^ other.data)
    }
}

impl<S, T> BitXor<Variable<S, T>> for Variable<S, T>
where
    T: BitXor<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn bitxor(self, other: Self) -> Self::Output {
        Variable::new(self.data ^ other.data)
    }
}

impl<'a, S, T> BitXorAssign<&'a Variable<S, T>> for Variable<S, T>
where
    T: BitXorAssign<&'a T>,
{
    fn bitxor_assign(&mut self, other: &'a Self) {
        self.data ^= &other.data
    }
}

impl<S, T> BitXorAssign<Variable<S, T>> for Variable<S, T>
where
    T: BitXorAssign<T>,
{
    fn bitxor_assign(&mut self, other: Self) {
        self.data ^= other.data
    }
}

impl<'a, S, T> Div<&'a Variable<S, T>> for Variable<S, T>
where
    T: Div<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn div(self, other: &'a Self) -> Self::Output {
        Variable::new(self.data / &other.data)
    }
}

impl<'a, S, T> Div<&'a Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: Div<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn div(self, other: Self) -> Self::Output {
        Variable::new(&self.data / &other.data)
    }
}

impl<'a, S, T> Div<Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: Div<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn div(self, other: Variable<S, T>) -> Self::Output {
        Variable::new(&self.data / other.data)
    }
}

/// This is for cases such as usize / NonZeroUsize and usize / usize
impl<S, T, U> Div<Variable<S, U>> for Variable<S, T>
where
    T: Div<U, Output = T>,
{
    type Output = Variable<S, T>;
    fn div(self, other: Variable<S, U>) -> Self::Output {
        Variable::new(self.data / other.data)
    }
}

impl<'a, S, T> DivAssign<&'a Variable<S, T>> for Variable<S, T>
where
    T: DivAssign<&'a T>,
{
    fn div_assign(&mut self, other: &'a Self) {
        self.data /= &other.data
    }
}

impl<S, T> DivAssign<Variable<S, T>> for Variable<S, T>
where
    T: DivAssign<T>,
{
    fn div_assign(&mut self, other: Self) {
        self.data /= other.data
    }
}

impl<'a, S, T> Mul<&'a Variable<S, T>> for Variable<S, T>
where
    T: Mul<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn mul(self, other: &'a Self) -> Self::Output {
        Variable::new(self.data * &other.data)
    }
}

impl<'a, S, T> Mul<&'a Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: Mul<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn mul(self, other: Self) -> Self::Output {
        Variable::new(&self.data * &other.data)
    }
}

impl<'a, S, T> Mul<Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: Mul<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn mul(self, other: Variable<S, T>) -> Self::Output {
        Variable::new(&self.data * other.data)
    }
}

impl<S, T> Mul<Variable<S, T>> for Variable<S, T>
where
    T: Mul<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn mul(self, other: Self) -> Self::Output {
        Variable::new(self.data * other.data)
    }
}

impl<'a, S, T> MulAssign<&'a Variable<S, T>> for Variable<S, T>
where
    T: MulAssign<&'a T>,
{
    fn mul_assign(&mut self, other: &'a Self) {
        self.data *= &other.data
    }
}

impl<S, T> MulAssign<Variable<S, T>> for Variable<S, T>
where
    T: MulAssign<T>,
{
    fn mul_assign(&mut self, other: Self) {
        self.data *= other.data
    }
}

impl<'a, S, T> Not for &'a Variable<S, T>
where
    &'a T: Not<Output = T>,
{
    type Output = Variable<S, T>;
    fn not(self) -> Self::Output {
        Variable::new(!&self.data)
    }
}

impl<S, T> Not for Variable<S, T>
where
    T: Not<Output = T>,
{
    type Output = Variable<S, T>;
    fn not(self) -> Self::Output {
        Variable::new(!self.data)
    }
}

impl<'a, S, T> Neg for &'a Variable<S, T>
where
    &'a T: Neg<Output = T>,
{
    type Output = Variable<S, T>;
    fn neg(self) -> Self::Output {
        Variable::new(-&self.data)
    }
}

impl<S, T> Neg for Variable<S, T>
where
    T: Neg<Output = T>,
{
    type Output = Variable<S, T>;
    fn neg(self) -> Self::Output {
        Variable::new(-self.data)
    }
}

impl<'a, S, T> Rem<&'a Variable<S, T>> for Variable<S, T>
where
    T: Rem<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn rem(self, other: &'a Self) -> Self::Output {
        Variable::new(self.data % &other.data)
    }
}

impl<'a, S, T> Rem<&'a Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: Rem<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn rem(self, other: Self) -> Self::Output {
        Variable::new(&self.data % &other.data)
    }
}

impl<'a, S, T> Rem<Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: Rem<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn rem(self, other: Variable<S, T>) -> Self::Output {
        Variable::new(&self.data % other.data)
    }
}

/// This is for cases such as usize / NonZeroUsize and usize / usize
impl<S, T, U> Rem<Variable<S, U>> for Variable<S, T>
where
    T: Rem<U, Output = T>,
{
    type Output = Variable<S, T>;
    fn rem(self, other: Variable<S, U>) -> Self::Output {
        Variable::new(self.data % other.data)
    }
}

impl<'a, S, T> RemAssign<&'a Variable<S, T>> for Variable<S, T>
where
    T: RemAssign<&'a T>,
{
    fn rem_assign(&mut self, other: &'a Self) {
        self.data %= &other.data
    }
}

impl<S, T> RemAssign<Variable<S, T>> for Variable<S, T>
where
    T: RemAssign<T>,
{
    fn rem_assign(&mut self, other: Self) {
        self.data %= other.data
    }
}

impl<'a, S, T, U> Shl<&'a Variable<S, U>> for &'a Variable<S, T>
where
    &'a T: Shl<&'a U, Output = T>,
{
    type Output = Variable<S, T>;
    fn shl(self, other: &'a Variable<S, U>) -> Self::Output {
        Variable::new(&self.data << &other.data)
    }
}

impl<'a, S, T, U> Shl<&'a Variable<S, U>> for Variable<S, T>
where
    T: Shl<&'a U, Output = T>,
{
    type Output = Variable<S, T>;
    fn shl(self, other: &'a Variable<S, U>) -> Self::Output {
        Variable::new(self.data << &other.data)
    }
}

impl<'a, S, T, U> Shl<Variable<S, U>> for &'a Variable<S, T>
where
    &'a T: Shl<U, Output = T>,
{
    type Output = Variable<S, T>;
    fn shl(self, other: Variable<S, U>) -> Self::Output {
        Variable::new(&self.data << other.data)
    }
}

impl<S, T, U> Shl<Variable<S, U>> for Variable<S, T>
where
    T: Shl<U, Output = T>,
{
    type Output = Variable<S, T>;
    fn shl(self, other: Variable<S, U>) -> Self::Output {
        Variable::new(self.data << other.data)
    }
}

impl<'a, S, T, U> ShlAssign<&'a Variable<S, U>> for Variable<S, T>
where
    T: ShlAssign<&'a U>,
{
    fn shl_assign(&mut self, other: &'a Variable<S, U>) {
        self.data <<= &other.data
    }
}

impl<S, T, U> ShlAssign<Variable<S, U>> for Variable<S, T>
where
    T: ShlAssign<U>,
{
    fn shl_assign(&mut self, other: Variable<S, U>) {
        self.data <<= other.data
    }
}

impl<'a, S, T, U> Shr<&'a Variable<S, U>> for &'a Variable<S, T>
where
    &'a T: Shr<&'a U, Output = T>,
{
    type Output = Variable<S, T>;
    fn shr(self, other: &'a Variable<S, U>) -> Self::Output {
        Variable::new(&self.data >> &other.data)
    }
}

impl<'a, S, T, U> Shr<&'a Variable<S, U>> for Variable<S, T>
where
    T: Shr<&'a U, Output = T>,
{
    type Output = Variable<S, T>;
    fn shr(self, other: &'a Variable<S, U>) -> Self::Output {
        Variable::new(self.data >> &other.data)
    }
}

impl<'a, S, T, U> Shr<Variable<S, U>> for &'a Variable<S, T>
where
    &'a T: Shr<U, Output = T>,
{
    type Output = Variable<S, T>;
    fn shr(self, other: Variable<S, U>) -> Self::Output {
        Variable::new(&self.data >> other.data)
    }
}

impl<S, T, U> Shr<Variable<S, U>> for Variable<S, T>
where
    T: Shr<U, Output = T>,
{
    type Output = Variable<S, T>;
    fn shr(self, other: Variable<S, U>) -> Self::Output {
        Variable::new(self.data >> other.data)
    }
}

impl<'a, S, T, U> ShrAssign<&'a Variable<S, U>> for Variable<S, T>
where
    T: ShrAssign<&'a U>,
{
    fn shr_assign(&mut self, other: &'a Variable<S, U>) {
        self.data >>= &other.data
    }
}

impl<S, T, U> ShrAssign<Variable<S, U>> for Variable<S, T>
where
    T: ShrAssign<U>,
{
    fn shr_assign(&mut self, other: Variable<S, U>) {
        self.data >>= other.data
    }
}

impl<'a, S, T> Sub<&'a Variable<S, T>> for Variable<S, T>
where
    T: Sub<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn sub(self, other: &'a Self) -> Self::Output {
        Variable::new(self.data - &other.data)
    }
}

impl<'a, S, T> Sub<&'a Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: Sub<&'a T, Output = T>,
{
    type Output = Variable<S, T>;
    fn sub(self, other: Self) -> Self::Output {
        Variable::new(&self.data - &other.data)
    }
}

impl<'a, S, T> Sub<Variable<S, T>> for &'a Variable<S, T>
where
    &'a T: Sub<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn sub(self, other: Variable<S, T>) -> Self::Output {
        Variable::new(&self.data - other.data)
    }
}

impl<S, T> Sub<Variable<S, T>> for Variable<S, T>
where
    T: Sub<T, Output = T>,
{
    type Output = Variable<S, T>;
    fn sub(self, other: Self) -> Self::Output {
        Variable::new(self.data - other.data)
    }
}

impl<'a, S, T> SubAssign<&'a Variable<S, T>> for Variable<S, T>
where
    T: SubAssign<&'a T>,
{
    fn sub_assign(&mut self, other: &'a Self) {
        self.data -= &other.data
    }
}

impl<S, T> SubAssign<Variable<S, T>> for Variable<S, T>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, other: Self) {
        self.data -= other.data
    }
}
