//! This module provides additional math for the SFML ``Vector2<T>``
//! by providing an extention trait.

use sfml::system::Vector2;
use num_traits::float::Float;

/// Extends the functionality of SFML's ``Vector2<T>``.
pub trait Vector2Ext<T> {
    /// Returns the squared length of the vector. More efficient than ``length``.
    fn length_sq(&self) -> T;

    /// Returns the actual length of the vector.
    fn length(&self) -> T;

    /// Normalize the vector.
    fn normalize(&self) -> Self;
}

impl<T: Float> Vector2Ext<T> for Vector2<T> {
    fn length_sq(&self) -> T {
        (self.x * self.x + self.y * self.y)
    }

    fn length(&self) -> T {
        self.length_sq().sqrt()
    }

    fn normalize(&self) -> Self {
        *self / self.length()
    }
}
