use crate::Variable;
use std::cmp::Ordering;

/// This trait is not safe .. it can leak information about the number
/// of elements which should be high variable
impl<S, T: Iterator> Iterator for Variable<S, T> {
    type Item = Variable<S, T::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(Variable::new)
    }
}

/// Leaks information about if two variables are equal or not
/// would be safe if output was Variable<S, bool>
impl<S, T: PartialOrd> PartialEq for Variable<S, T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<S, T: Ord> Eq for Variable<S, T> {}

/// leaks information about Order of variables would be safe
/// if output was Variable<S, Option<Variable<S, Ordering>>>
impl<S, T: PartialOrd> PartialOrd for Variable<S, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

/// leaks information about Order of variables would be safe
/// if output was Variable<S, Ordering>.
impl<S, T: Ord> Ord for Variable<S, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}
