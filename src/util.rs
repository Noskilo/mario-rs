pub mod lerp {
    use std::ops::{Add, Mul};
    use num_traits::{One, Float, Zero};

    pub trait Lerp<F> {
        /// Interpolate and extrapolate between `self` and `other` using `t` as the parameter.
        ///
        /// At `t == 0.0`, the result is equal to `self`.
        /// At `t == 1.0`, the result is equal to `other`.
        /// At all other points, the result is a mix of `self` and `other`, proportional to `t`.
        ///
        /// `t` is unbounded, so extrapolation and negative interpolation are no problem.
        ///
        /// # Examples
        ///
        /// Basic lerping on floating points:
        ///
        /// ```
        /// use lerp::Lerp;
        ///
        /// let four_32 = 3.0_f32.lerp(5.0, 0.5);
        /// assert_eq!(four_32, 4.0);
        /// let four_64 = 3.0_f64.lerp(5.0, 0.5);
        /// assert_eq!(four_64, 4.0);
        /// ```
        ///
        /// Extrapolation:
        ///
        /// ```
        /// # use lerp::Lerp;
        /// assert_eq!(3.0.lerp(4.0, 2.0), 5.0);
        /// ```
        ///
        /// Negative extrapolation:
        ///
        /// ```
        /// # use lerp::Lerp;
        /// assert_eq!(3.0.lerp(4.0, -1.0), 2.0);
        /// ```
        ///
        /// Reverse interpolation:
        ///
        /// ```
        /// # use lerp::Lerp;
        /// assert_eq!(5.0.lerp(3.0, 0.5), 4.0);
        /// ```
        fn lerp(self, other: Self, t: F) -> Self;

        /// Interpolate between `self` and `other` precisely per the `lerp` function, bounding `t`
        /// in the inclusive range [0..1].
        ///
        /// # Examples
        ///
        /// Bounding on numbers greater than one:
        ///
        /// ```
        /// # use lerp::Lerp;
        /// assert_eq!(3.0.lerp_bounded(4.0, 2.0), 4.0);
        /// ```
        ///
        /// Bounding on numbers less than zero:
        ///
        /// ```
        /// # use lerp::Lerp;
        /// assert_eq!(3.0.lerp_bounded(5.0, -2.0), 3.0);
        /// ```
        fn lerp_bounded(self, other: Self, t: F) -> Self
        where
            Self: Sized,
            F: PartialOrd + Copy + Zero + One,
        {
            let t = match t {
                t if t < F::zero() => F::zero(),
                t if t > F::one() => F::one(),
                t => t,
            };
            self.lerp(other, t)
        }
    }

    impl<T, F> Lerp<F> for T
    where
        T: Add<Output = T> + Mul<F, Output = T>,
        F: Float,
    {
        fn lerp(self, other: T, t: F) -> T {
            self * (F::one() - t) + other * t
        }
    }
}
