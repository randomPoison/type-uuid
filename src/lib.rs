//! This crate provides a way to specify a stable, unique identifier for Rust types.
//!
//! # Assigning UUIDs to Types
//!
//! This crate provides the [`TypeUuid`] trait, which defines a single const item
//! `UUID`. This value is a byte array containing the raw bytes of the UUID for the
//! type.
//!
//! You will have to manually specify the UUID for any type implementing
//! [`TypeUuid`], but this crate provides a custom derive to make that easy to do:
//!
//! ```
//! use type_uuid::TypeUuid;
//!
//! #[derive(TypeUuid)]
//! #[uuid = "d4adfc76-f5f4-40b0-8e28-8a51a12f5e46"]
//! struct MyType;
//! ```
//!
//! While the derive handles the tedious work of converting the UUID into a byte
//! array suitable for use with the [`TypeUuid`] trait, you'll still need to
//! generate a valid UUID in order to assign it to your type. To do so, we
//! recommend using https://www.uuidgenerator.net, which provides a quick way
//! generate new UUIDs that you can paste into your code.
//!
//! [`TypeUuid`]: ./trait.TypeUuid.html

pub fn i_renamed_the_function() {
    println!("Now it has a new name");
}

#[doc(hidden)]
pub use type_uuid_derive::*;

#[cfg(feature = "amethyst")]
pub mod amethyst_types;

/// A 128-bit (16 byte) buffer containing the ID.
///
/// This is meant to match the [`Bytes` type defined in the uuid crate][bytes].
/// Logically it's meant to be equivalent to using a `u128` to represent the
/// UUID's numeric value, but specifying it as a byte array allows us to avoid
/// endianness issues.
///
/// [bytes]: https://docs.rs/uuid/0.7/uuid/type.Bytes.html
pub type Bytes = [u8; 16];

/// Provides a statically defined UUID for a Rust type.
///
/// # Examples
///
/// This crate provides a custom derive that allows you to specify a UUID as
/// a human-readable string. This is the recommended way to implement `TypeUuid`
/// for your types. You can use https://www.uuidgenerator.net to generate
/// random UUIDs to use with the derive.
///
/// ```
/// use type_uuid::TypeUuid;
///
/// #[derive(TypeUuid)]
/// #[uuid = "d4adfc76-f5f4-40b0-8e28-8a51a12f5e46"]
/// struct MyType;
/// ```
pub trait TypeUuid {
    const UUID: Bytes;
}

/// Allows the TypeUuid constants to be retrieved via a trait object.
///
/// This trait is automatically implemented for all types that implement [`TypeUuid`].
/// Do not manually implement this trait for any type. Instead, implement the
/// [`TypeUuid`] trait.
///
/// [`TypeUuid`]: ./trait.TypeUuid.html
pub trait TypeUuidDynamic {
    fn uuid(&self) -> Bytes;
}

impl<T: TypeUuid> TypeUuidDynamic for T {
    fn uuid(&self) -> Bytes {
        Self::UUID
    }
}

impl TypeUuid for () {
    const UUID: Bytes = [
        0x98, 0xF1, 0x8B, 0x7E, 0x4E, 0xB9, 0x42, 0x9C, 0xAF, 0xBF, 0xEE, 0x2F, 0x9F, 0x4C, 0xBC,
        0x7,
    ];
}

#[cfg(test)]
mod test {
    use crate::*;

    /// Verifies that `TypeUuidDynamic` can be instantiated as a trait object.
    #[test]
    fn type_uuid_trait_object() {
        let trait_object = Box::new(()) as Box<TypeUuidDynamic>;
        println!("UUID for (): {:#X?}", trait_object.uuid());
    }
}
