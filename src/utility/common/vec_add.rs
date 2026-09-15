use std::ops::{Add, Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct AddableVec<T>(pub Vec<T>);

impl<T> AddableVec<T> {
    pub fn append(&mut self, other: AddableVec<T>) {
        other.0.into_iter().for_each(|x| self.0.push(x));
    }
}

impl<T> Into<Vec<T>> for AddableVec<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}

impl<T> From<Vec<T>> for AddableVec<T> {
    fn from(vec: Vec<T>) -> Self {
        AddableVec(vec)
    }
}

impl<T> Deref for AddableVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for AddableVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Add for AddableVec<T> {
    type Output = Self;

    fn add(mut self, mut other: Self) -> Self {
        self.0.append(&mut other.0);
        self
    }
}
