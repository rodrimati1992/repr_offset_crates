/// The `ReprOffset` derive macro defines associated constants with the offset of every field,
/// and implements the [`GetFieldOffset`] trait,
///
/// The `ReprOffset` derive is from the `repr_offset_derive` crate,
/// and is re-exported by this crate when
/// the "derive" feature is enabled (it's disabled by default).
///
/// Adding the "derive" feature to the Cargo.toml file:
/// ```toml
/// repr_offset = { version = "0.2", features = ["derive"] }
/// ```
///
/// # Generated items
///
/// By default, this derive macro generates:
///
/// - [`FieldOffset`] inherent associated constants, with the same privacy as the field,
/// named `OFFSET_<field_name>` (where `<field_name>` is the name of the field uppercased).
///
/// - Impls of the [`GetFieldOffset`] trait for each field.
///
/// - An impl of the [`ImplsGetFieldOffset`] marker trait.
///
/// # Valid Representation Attributes
///
/// These are the valid representation attributes:
///
/// - `#[repr(C)]`
///
/// - `#[repr(transparent)]`: This is treated the same as `#[repr(C)]`.
///
/// - `#[repr(C, align(1000))]`
///
/// - `#[repr(C, packed)]`
///
/// - `#[repr(C, packed(1000))]`
///
/// One of those must be used,otherwise the derive macro will error.
///
///
/// # Container Attributes
///
/// ### `#[roff(usize_offsets)]`
///
/// Changes the generated offset associated constants from [`FieldOffset`] to `usize`.
///
/// Example:
/// ```rust
/// use repr_offset::{
///     ReprOffset,
///     off,
///     Aligned, FieldOffset,
/// };
///
/// 
///
/// #[repr(C)]
/// #[derive(ReprOffset)]
/// #[roff(usize_offsets)]
/// struct Foo{
///     x: u8,
///     y: u64,
///     z: String,
/// }
///
/// let _: usize = Foo::OFFSET_X;
/// let _: usize = Foo::OFFSET_Y;
/// let _: usize = Foo::OFFSET_Z;
///
/// // You can still get the `FieldOffset` of fields using the `GetFieldOffset` impls for `Foo`,
/// // in this case through the `off` macro.
/// let _: FieldOffset<Foo, u8, Aligned> = off!(x);
/// let _: FieldOffset<Foo, u64, Aligned> = off!(y);
/// let _: FieldOffset<Foo, String, Aligned> = off!(z);
///
/// ```
///
/// ### `#[roff(bound = "T: Foo")]`
///
/// This attribute adds a constraint to the generated impl block that defines
/// the field offset constants.
///
/// Examples:
///
/// - `#[roff(bound = "T: 'a")]`
///
/// - `#[roff(bound = "U: Foo")]`
///
/// ### `#[roff(impl_GetFieldOffset = true)]`
///
/// Chooses whether [`GetFieldOffset`] is implemented for all the fields or none of them,
/// if `true` then [`GetFieldOffset`] is implemented for all the fields,
/// if `false` then [`GetFieldOffset`] is implemented for none of the fields.
/// 
///
/// # Field attributes
///
/// ### `#[roff(offset = "fooo")]`
///
/// Changes the name of the generated offset for the field.
///
/// Example:
/// ```rust
/// use repr_offset::{
///     ReprOffset,
///     off,
///     Aligned, FieldOffset,
/// };
///
/// #[repr(C)]
/// #[derive(ReprOffset)]
/// struct Foo{
///     x: u8,
///     y: u64,
///     #[roff(offset = "this_is_lowercase")]
///     z: String,
/// }
///
/// let _: FieldOffset<Foo, u8, Aligned> = Foo::OFFSET_X;
/// let _: FieldOffset<Foo, u64, Aligned> = Foo::OFFSET_Y;
/// let _: FieldOffset<Foo, String, Aligned> = Foo::this_is_lowercase;
///
/// // The `off` macro can still access the offset for the `z` field with its original name.
/// let _: FieldOffset<Foo, u8, Aligned> = off!(x);
/// let _: FieldOffset<Foo, u64, Aligned> = off!(y);
/// let _: FieldOffset<Foo, String, Aligned> = off!(z);
///
/// ```
///
///
/// # Container or Field attributes
///
/// ### `#[roff(offset_prefix = "FOO" )]`
///
/// Changes the prefix of the name of the generated offset(s) for the field(s).
///
/// When used on the type, it uses this as the default prefix of all
/// the offset constants for the fields.
///
/// When used on a field,
/// it overrides the prefix of the name of the offset constant for the field.
///
/// Example:
/// ```rust
/// use repr_offset::{
///     ReprOffset,
///     off,
///     FieldOffset, Unaligned,
/// };
///
/// #[repr(C, packed)]
/// #[derive(ReprOffset)]
/// #[roff(offset_prefix = "OFF_")]
/// struct Foo{
///     x: u8,
///     y: u64,
///     // This overrides the `offset_prefix` attribute above.
///     #[roff(offset_prefix = "OOO_")]
///     z: String,
/// }
///
/// let _: FieldOffset<Foo, u8, Unaligned> = Foo::OFF_X;
/// let _: FieldOffset<Foo, u64, Unaligned> = Foo::OFF_Y;
/// let _: FieldOffset<Foo, String, Unaligned> = Foo::OOO_Z;
///
/// // The `off` macro gets the `FieldOffset` using the `GetFieldOffset` impls for `Foo`.
/// let _: FieldOffset<Foo, u8, Unaligned> = off!(x);
/// let _: FieldOffset<Foo, u64, Unaligned> = off!(y);
/// let _: FieldOffset<Foo, String, Unaligned> = off!(z);
///
/// ```
///
///
/// [`FieldOffset`]: ./struct.FieldOffset.html
///
///
/// # Examples
///
/// ### Out parameters
///
/// This example demonstrates how you can write each field individually to an out parameter
/// (a way that complex values can be returned in the C language).
///
/// ```rust
/// use repr_offset::{
///     ReprOffset,
///     off,
///     FieldOffset, ROExtRawMutOps, Unaligned,
/// };
///
/// use std::ffi::{CStr, CString};
/// use std::os::raw::c_char;
///
/// fn main(){
///     let mut results = Vec::<Fields>::with_capacity(3);
///
///     unsafe{
///         let ptr = results.as_mut_ptr();
///         assert_eq!( write_fields(10, 2, ptr.offset(0)), ErrorCode::Ok );
///         assert_eq!( write_fields(22, 3, ptr.offset(1)), ErrorCode::Ok );
///         assert_eq!( write_fields(1, 0, ptr.offset(2)), ErrorCode::DivisionByZero );
///         results.set_len(2);
///
///         let cstr_as_str = |ptr| CStr::from_ptr(ptr).to_str().unwrap();
///
///         assert_eq!( results[0].divided, 5 );
///         assert_eq!( cstr_as_str(results[0].string), "5" );
///
///         assert_eq!( results[1].divided, 7 );
///         assert_eq!( cstr_as_str(results[1].string), "7" );
///     }
/// }
///
/// #[no_mangle]
/// pub unsafe extern fn write_fields(left: u32, right: u32, out: *mut Fields) -> ErrorCode {
///     let divided = match left.checked_div(right) {
///         Some(x) => x,
///         None => return ErrorCode::DivisionByZero,
///     };
///
///     let written_string= CString::new(divided.to_string())
///         .expect("There shouldn't be a nul byte in the string returned by `u32::to_string`")
///         .into_raw();
///
///     unsafe{
///         Fields::OFFSET_DIVIDED.write(out, divided);
///
///         // Another way to write to a field,
///         // using the `ROExtRawMutOps` extension trait, and `off` macro.
///         out.f_write(off!(string), written_string);
///     }
///
///     ErrorCode::Ok
/// }
///
/// #[no_mangle]
/// pub unsafe extern fn cstring_free(ptr: *mut c_char) {
///     drop(CString::from_raw(ptr));
/// }
///
/// #[repr(C)]
/// #[derive(Debug, ReprOffset)]
/// pub struct Fields{
///     divided: u32,
///     string: *mut c_char,
/// }
///
/// impl Drop for Fields {
///     fn drop(&mut self) {
///         unsafe{ cstring_free(self.string); }
///     }
/// }
///
/// #[derive(Debug, PartialEq)]
/// #[repr(u8)]
/// pub enum ErrorCode{
///     Ok,
///     DivisionByZero,
/// }
///
///
/// ```
///
///
/// [`GetFieldOffset`]: ./get_field_offset/trait.GetFieldOffset.html
/// [`ImplsGetFieldOffset`]: ./get_field_offset/trait.ImplsGetFieldOffset.html
///
#[doc(inline)]
#[cfg(feature = "derive")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "derive")))]
pub use repr_offset_derive::ReprOffset;
