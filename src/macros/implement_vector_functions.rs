/// This macro is used for implementing the "unit" function
/// on a vector which contains f64 elements.
#[macro_export]
macro_rules! implement_unit_function {
    ($vector_type: ty, $($field: ident), *) => {
        impl $vector_type {
            pub fn unit(self) -> $vector_type {
                self / self.size()
            }
        }
    };
}



/// Implement cross product for vector.
/// Cross product is defined only for vectors containing 3 elements.
#[macro_export]
macro_rules! implement_cross_function {
    ($vector_type: ty, $field1: ident, $field2: ident, $field3: ident) => {
        impl $vector_type {
            fn cross(&self, other: $vector_type) -> $vector_type {
                <$vector_type>::new(
                    self.$field2 * other.$field3 - self.$field3 * other.$field2,
                    self.$field3 * other.$field1 - self.$field1 * other.$field3,
                    self.$field1 * other.$field2 - self.$field2 * other.$field1
                )
            }
        }
    };
}


#[macro_export]
macro_rules! implement_common_vector_functions {
    ($vector_type: ty, $element_type: ty, $($field: ident), *) => {
        use std::ops::{Add, Mul, Div, Sub};

        impl $vector_type {
            pub fn size_squared(&self) -> f64 {
                let mut result: f64 = 0.0;
                $(
                    result += (self.$field as f64).powf(2.0);
                )*

                result
            }
            
            pub fn size(&self) -> f64 {
                self.size_squared().sqrt()
            }

            /// Implement dot product.
            /// For example (1, 2, 3) * (1, 0, 1) = 1 + 3 = 4.
            pub fn dot(&self, other: $vector_type) -> f64 {
                let mut result: f64 = 0.0;
                $(
                    result += (self.$field as f64) * (other.$field as f64);
                )*

                result
            }
        }
        

        /// Implement vector addition.
        /// For example: (1, 0, 1) + (1, 2, 3) = (2, 2, 4)
        impl Add for $vector_type {
            type Output = $vector_type;

            fn add(self, rhs: Self) -> Self::Output {
                <$vector_type>::new(
                    $(
                        self.$field + rhs.$field,
                    )*
                )
            }
        }

        /// Implement vector substraction.
        /// For example: (1, 0, 1) - (1, 2, 3) = (0, -2, -2)
        impl Sub for $vector_type {
            type Output = $vector_type;

            fn sub(self, rhs: Self) -> Self::Output {
                <$vector_type>::new(
                    $(
                        self.$field - rhs.$field,
                    )*
                )
            }
        }

        /// Implement Cartesian product.
        /// For example: (1, 2) * (2, 3) = (2, 6)
        impl Mul for $vector_type {
            type Output = $vector_type;

            fn mul(self, rhs: Self) -> Self::Output {
                <$vector_type>::new(
                    $(
                        self.$field * rhs.$field,
                    )*
                )
            }
        }

        /// Implement vector and scalar product.
        /// For example: 2 * (1, 2, 3) = (2, 4, 6)
        impl Mul<$vector_type> for $element_type {
            type Output = $vector_type;

            fn mul(self, rhs: $vector_type) -> Self::Output {
                <$vector_type>::new(
                    $(
                        rhs.$field * self,
                    )*
                )
            }
        }

        /// Implement vector and scalar division.
        /// For example: (2, 4, 2) / (2 = (1, 2, 1)
        impl Div<$element_type> for $vector_type {
            type Output = $vector_type;

            fn div(self, rhs: $element_type) -> Self::Output {
                <$vector_type>::new(
                    $(
                        self.$field / rhs,
                    )*
                )
            }
        }

        /// Implement Cartesian division.
        /// For example: (2, 4, 2) / (2, 1, 2) = (1, 4, 1)
        impl Div for $vector_type {
            type Output = $vector_type;

            fn div(self, rhs: Self) -> Self::Output {
                <$vector_type>::new(
                    $(
                        self.$field / rhs.$field,
                    )*
                )
            }
        }
    }
}