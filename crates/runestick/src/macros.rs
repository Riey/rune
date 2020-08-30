/// Implement the value trait for an external type.
///
/// This is required to support the external type as a type argument in a
/// registered function.
///
/// This will be **deprecated** once (or if) [specialization] lands.
///
/// [specialization]: https://github.com/rust-lang/rust/issues/31844
#[macro_export]
macro_rules! decl_external {
    ($external:ty) => {
        $crate::decl_internal!($external);

        impl $crate::FromValue for $external {
            fn from_value(value: $crate::Value) -> Result<Self, $crate::ValueError> {
                let any = value.into_any()?;
                let any = any.take_downcast::<$external>()?;
                Ok(any)
            }
        }
    };
}

/// Implement the value trait for an internal type.
#[macro_export]
macro_rules! decl_internal {
    ($external:ty) => {
        impl $crate::ReflectValueType for $external {
            type Owned = $external;

            fn value_type() -> $crate::ValueType {
                $crate::ValueType::Any(std::any::TypeId::of::<$external>())
            }

            fn value_type_info() -> $crate::ValueTypeInfo {
                $crate::ValueTypeInfo::Any(std::any::type_name::<$external>())
            }
        }

        impl<'a> $crate::ReflectValueType for &'a $external {
            type Owned = $external;

            fn value_type() -> $crate::ValueType {
                $crate::ValueType::Any(std::any::TypeId::of::<$external>())
            }

            fn value_type_info() -> $crate::ValueTypeInfo {
                $crate::ValueTypeInfo::Any(std::any::type_name::<$external>())
            }
        }

        impl<'a> $crate::ReflectValueType for &'a mut $external {
            type Owned = $external;

            fn value_type() -> $crate::ValueType {
                $crate::ValueType::Any(std::any::TypeId::of::<$external>())
            }

            fn value_type_info() -> $crate::ValueTypeInfo {
                $crate::ValueTypeInfo::Any(std::any::type_name::<$external>())
            }
        }

        impl $crate::ToValue for $external {
            fn to_value(self) -> Result<$crate::Value, $crate::ValueError> {
                let any = $crate::Any::new(self);
                let shared = $crate::Shared::new(any);
                Ok($crate::Value::Any(shared))
            }
        }

        impl<'a> $crate::UnsafeFromValue for &'a $external {
            type Output = *const $external;
            type Guard = $crate::RawOwnedRef;

            unsafe fn unsafe_from_value(
                value: $crate::Value,
            ) -> Result<(Self::Output, Self::Guard), $crate::ValueError> {
                Ok(value.unsafe_into_any_ref()?)
            }

            unsafe fn to_arg(output: Self::Output) -> Self {
                &*output
            }
        }

        impl<'a> $crate::UnsafeFromValue for &'a mut $external {
            type Output = *mut $external;
            type Guard = $crate::RawOwnedMut;

            unsafe fn unsafe_from_value(
                value: $crate::Value,
            ) -> Result<(Self::Output, Self::Guard), $crate::ValueError> {
                Ok(value.unsafe_into_any_mut()?)
            }

            unsafe fn to_arg(output: Self::Output) -> Self {
                &mut *output
            }
        }
    };
}

/// Declare value types for the specific kind.
macro_rules! value_types {
    ($ident:ident, $owned:ty => $($ty:ty),+) => {
        $(
            impl $crate::ReflectValueType for $ty {
                type Owned = $owned;

                fn value_type() -> $crate::ValueType {
                    $crate::ValueType::$ident
                }

                fn value_type_info() -> $crate::ValueTypeInfo {
                    $crate::ValueTypeInfo::$ident
                }
            }
        )*
    };
}