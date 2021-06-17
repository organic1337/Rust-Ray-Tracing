#[macro_use]

#[macro_export]
macro_rules! implement_vector_functions {
    ($vector_type: ty, $($field: ident), *) => {
        /// Implement vector size calculation
        impl $vector_type {
            pub fn size_squared(&self) -> f32 {
                let mut result: f32 = 0.0;
                $(
                    result += self.$field.powf(2.0);
                )*

                result
            }
            
            pub fn size(&self) -> f32 {
                self.size_squared().sqrt()
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
        impl Mul<f32> for $vector_type {
            type Output = $vector_type;

            fn mul(self, rhs: f32) -> Self::Output {
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
    }
}