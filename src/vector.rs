use anyhow::{anyhow, Result};
use std::{
    fmt,
    ops::{Add, AddAssign, Deref, Mul},
};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + fmt::Debug,
{
    if a.len() != b.len() {
        return Err(anyhow!("Vectors must have the same length for dot product"));
    }

    let mut result = T::default();
    for (x, y) in a.iter().zip(b.iter()) {
        result += *x * *y;
    }

    Ok(result)
}
