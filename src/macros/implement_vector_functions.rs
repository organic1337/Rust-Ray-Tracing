/// This macro is used for implementing the "unit_vector" function
/// on a vector which contains f32 elements.
macro_rules! implement_unit_vector {
    ($vector_type: ty, f32, $($field: ident), *) => {
        impl $vector_type {
            pub fn unit_vector(v: $vector_type) {
                v / v.length()
            }
        }
    };

    ($vector_type: ty, $element_type: ty, $($field: ident), *) => {
        // Cannot implement unit vector for vector with non float
        // elements.
    };
}


macro_rules! implement_cross {
    ($vector_type: ty, f32, $field1: ident, $field2: ident, $field3: ident) => {
        impl $vector_type {
            fn cross(&self, other: Vector) -> Vector {
                Vector::new(
                    self.$field2 * other.$field3 - self.$field3 * other.$field2,
                    self.$field3 * other.$field1 - self.$field1 * other.$field3,
                    self.$field1 * other.$field2 - self.$field2 * other.$field1
                )
            }
        }
    };

    ($vector_type: ty, $element_type: ty, $($field: ident), *) => {
        // Cross product is defined only for vectors with 3 elements.
    };
}


#[macro_export]
macro_rules! implement_vector_functions {
    ($vector_type: ty, $element_type: ty, $($field: ident), *) => {
        use std::ops::{Add, Mul, Div};

        impl $vector_type {
            pub fn size_squared(&self) -> f32 {
                let mut result: f32 = 0.0;
                $(
                    result += (self.$field as f32).powf(2.0);
                )*

                result
            }
            
            pub fn size(&self) -> f32 {
                self.size_squared().sqrt()
            }

            /// Implement dot product.
            /// For example (1, 2, 3) * (1, 0, 1) = 1 + 3 = 4.
            pub fn dot(&self, other: $vector_type) -> f32 {
                let mut result: f32 = 0.0;
                $(
                    result += (self.$field as f32) * (other.$field as f32);
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

        /// Implement $vector_type & scalar product.
        /// For example: 2 * (1, 2, 3) = (2, 4, 6)
        impl Mul<$element_type> for $vector_type {
            type Output = $vector_type;

            fn mul(self, rhs: $element_type) -> Self::Output {
                <$vector_type>::new(
                    $(
                        self.$field * rhs,
                    )*
                )
            }
        }

        /// Implement Cartesian devision.
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


        // Case specific Implementations
        implement_unit_vector!($vector_type, $element_type, $($field), *);
        implement_cross!($vector_type, $element_type, $($field), *);
    }
}