//! Extension traits that use `FieldOffset` parameters to operate on fields.
//!
//! These are the extension traits for each kind of type:
//!
//! - non-pointer / `&T` / `&mut T`: [`ROExtAcc`] and [`ROExtOps`]
//!
//! - `*const T` and `*mut T`: [`ROExtRawAcc`] and [`ROExtRawOps`]
//!
//! - `*mut T`: [`ROExtRawMutAcc`] and [`ROExtRawMutOps`]
//!
//! # Imports
//!
//! Here is the code to import all of the extension traits for convenience:
//! ```rust
//! use repr_offset::{ROExtAcc, ROExtOps, ROExtRawAcc, ROExtRawMutAcc, ROExtRawOps, ROExtRawMutOps};
//! ```
//!
//! # Examples
//!
//! # Initializing Types
//!
//! ```rust
//! use repr_offset::{
//!     for_examples::ReprC,
//!     off,
//!     ROExtRawMutOps,
//! };
//!
//! use std::mem::MaybeUninit;
//!
//! type This = ReprC<String, Vec<u8>, usize, Option<char>>;
//!
//! let this = unsafe{
//!     let mut uninit = MaybeUninit::<This>::uninit();
//!     initialize_this(uninit.as_mut_ptr());
//!     uninit.assume_init()
//! };
//!
//! assert_eq!(this.a, "foo");
//! assert_eq!(this.b, [3, 5, 8]);
//! assert_eq!(this.c, 13);
//! assert_eq!(this.d, Some('_'));
//!
//! /// Initializes `this`
//! ///
//! /// # Safety
//! ///
//! /// The passed in pointer must point to an aligned, allocated `This` (including on the stack).
//! unsafe fn initialize_this(this: *mut This) {
//!     this.f_write(off!(a), "foo".to_string());
//!     this.f_write(off!(b), vec![3, 5, 8]);
//!     this.f_write(off!(c), 13_usize);
//!     this.f_write(off!(d), Some('_'));
//! }
//!
//! ```
//!
//!
//!
//! [`ROExtAcc`]: ./trait.ROExtAcc.html
//! [`ROExtOps`]: ./trait.ROExtOps.html
//! [`ROExtRawAcc`]: ./trait.ROExtRawAcc.html
//! [`ROExtRawMutAcc`]: ./trait.ROExtRawMutAcc.html
//! [`ROExtRawOps`]: ./trait.ROExtRawOps.html
//! [`ROExtRawMutOps`]: ./trait.ROExtRawMutOps.html
//!
//! [`FieldOffset`]: ../struct.FieldOffset.html

use crate::{Aligned, FieldOffset};

/// Extension trait for (mutable) references to access fields generically,
/// where the field is determined by a [`FieldOffset`] parameter.
///
///
/// # Safety
///
/// This trait must not to be implemented outside the `repr_offset` crate.
///
/// # Examples
///
/// ### Unaligned field pointers
///
/// Getting raw pointers to all fields in a packed struct.
///
/// ```rust
/// # #![deny(safe_packed_borrows)]
/// use repr_offset::{
///     for_examples::ReprPacked,
///     ROExtAcc,
///     off,
/// };
///
/// type This = ReprPacked<u8, Option<usize>, &'static [u16], &'static str>;
///
/// let value: This = ReprPacked {
///     a: 3,
///     b: Some(5),
///     c: &[0, 1, 2],
///     d: "hello",
/// };
///
/// unsafe {
///     let (a_ptr, b_ptr, c_ptr, d_ptr) = get_field_ptrs(&value);
///     assert_eq!(a_ptr.read_unaligned(), 3);
///     assert_eq!(b_ptr.read_unaligned(), Some(5));
///     assert_eq!(c_ptr.read_unaligned(), &[0, 1, 2]);
///     assert_eq!(d_ptr.read_unaligned(), "hello");
/// }
///
/// // Note: the mutable version of this function couldn't use `f_get_mut_ptr`,
/// //       it must use `ROExtRawMutAcc::f_raw_get_mut`
/// fn get_field_ptrs(
///     this: &This,
/// ) -> (*const u8, *const Option<usize>, *const &'static [u16], *const &'static str) {
///     (
///         this.f_get_ptr(off!(a)),
///         this.f_get_ptr(off!(b)),
///         this.f_get_ptr(off!(c)),
///         this.f_get_ptr(off!(d)),
///     )
/// }
///
/// ```
///
/// [`FieldOffset`]: ../struct.FieldOffset.html
//
// This trait is implemented in src/struct_field_offset/repr_offset_ext_impls.rs
//
pub unsafe trait ROExtAcc: Sized {
    /// Gets a reference to a field, determined by `offset`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::ReprC,
    ///     ROExtAcc, off,
    /// };
    ///
    /// let value = ReprC {
    ///     a: 3,
    ///     b: "foo",
    ///     c: ReprC {
    ///         a: 5,
    ///         b: "bar",
    ///         c: 8,
    ///         d: 13,
    ///     },
    ///     d: false,
    /// };
    ///
    /// assert_eq!(value.f_get(off!(a)), &3);
    /// assert_eq!(value.f_get(off!(c.a)), &5);
    /// assert_eq!(value.f_get(off!(c.b)), &"bar");
    ///
    ///
    /// ```
    fn f_get<F>(&self, offset: FieldOffset<Self, F, Aligned>) -> &F;

    /// Gets a muatble reference to a field, determined by `offset`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::{ReprAlign4, ReprC},
    ///     ROExtAcc, off,
    /// };
    ///
    /// use std::cmp::Ordering;
    ///
    /// let mut value = ReprC {
    ///     a: 3,
    ///     b: Some(5),
    ///     c: Ordering::Less,
    ///     d: ReprAlign4 {
    ///         a: 8,
    ///         b: "bar",
    ///         c: 13,
    ///         d: 21,
    ///     },
    /// };
    ///
    /// let foo = value.f_get_mut(off!(a));
    /// assert_eq!(foo, &mut 3);
    /// *foo += 100;
    /// assert_eq!(value.a, 103);
    ///
    /// let bar = value.f_get_mut(off!(d.a));
    /// assert_eq!(bar, &mut 8);
    ///
    /// let baz = value.f_get_mut(off!(d.d));
    /// assert_eq!(baz, &mut 21);
    /// *baz += 300;
    /// assert_eq!(value.d.d, 321);
    ///
    /// ```
    ///
    fn f_get_mut<F>(&mut self, offset: FieldOffset<Self, F, Aligned>) -> &mut F;

    /// Gets a const pointer to a field,
    /// the field is determined by `offset`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::ReprC,
    ///     ROExtAcc, off,
    /// };
    ///
    /// let value = ReprC {
    ///     a: 3,
    ///     b: "foo",
    ///     c: ReprC {
    ///         a: 5,
    ///         b: "bar",
    ///         c: 8,
    ///         d: 13,
    ///     },
    ///     d: false,
    /// };
    ///
    /// unsafe {
    ///     assert_eq!(value.f_get_ptr(off!(a)).read(), 3);
    ///     assert_eq!(value.f_get_ptr(off!(c.a)).read(), 5);
    ///     assert_eq!(value.f_get_ptr(off!(c.b)).read(), "bar");
    /// }
    ///
    /// ```
    fn f_get_ptr<F, A>(&self, offset: FieldOffset<Self, F, A>) -> *const F;

    /// Gets a mutable pointer to a field, determined by `offset`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::{ReprC, ReprPacked},
    ///     utils::moved,
    ///     ROExtAcc, off,
    /// };
    ///
    /// use std::cmp::Ordering;
    ///
    /// let mut value = ReprPacked {
    ///     a: 3,
    ///     b: Some(5),
    ///     c: Ordering::Less,
    ///     d: ReprC {
    ///         a: 8,
    ///         b: "bar",
    ///         c: 13,
    ///         d: 21,
    ///     },
    /// };
    ///
    /// unsafe {
    ///     let foo = value.f_get_mut_ptr(off!(a));
    ///     let old_a = foo.read_unaligned();
    ///     assert_eq!(old_a, 3);
    ///     foo.write_unaligned(old_a + 100);
    ///     // the `moved` function prevents the creation of a reference to the packed field.
    ///     assert_eq!(moved(value.a), 103);
    ///     
    ///     let baz = value.f_get_mut_ptr(off!(d.d));
    ///     let old_dd = baz.read_unaligned();
    ///     assert_eq!(old_dd, 21);
    ///     baz.write_unaligned(old_dd + 300);
    ///     // the `moved` function prevents the creation of a reference to the packed field.
    ///     assert_eq!(moved(value.d.d), 321);
    /// }
    /// ```
    fn f_get_mut_ptr<F, A>(&mut self, offset: FieldOffset<Self, F, A>) -> *mut F;
}

/// Extension trait for (mutable) references to do generic field operations,
/// where the field is determined by a [`FieldOffset`] parameter.
///
///
/// # Safety
///
/// This trait must not to be implemented outside the `repr_offset` crate.
///
/// # Alignment
///
/// The `A` type parameter is the [`Alignment`] of the field,
/// used to implement methods differently depending on whether the field is
/// [`Aligned`] or [`Unaligned`].
///
/// [`FieldOffset`]: ../struct.FieldOffset.html
/// [`Alignment`]: ../alignment/trait.Alignment.html
/// [`Aligned`]: ../alignment/struct.Aligned.html
/// [`Unaligned`]: ../alignment/struct.Unaligned.html
///
///
//
// This trait is implemented in src/struct_field_offset/repr_offset_ext_impls.rs
//
pub unsafe trait ROExtOps<A>: ROExtAcc {
    /// Replaces a field (determined by `offset`) with `value`,
    /// returning the previous value of the field.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::ReprPacked,
    ///     utils::moved,
    ///     ROExtOps, off,
    /// };
    ///
    /// let mut value = ReprPacked {
    ///     a: 3u128,
    ///     b: Some(5u64),
    ///     c: vec![0, 1],
    ///     d: (),
    /// };
    ///
    /// assert_eq!(value.f_replace(off!(a), 200), 3);
    /// assert_eq!(moved(value.a), 200);
    ///
    /// assert_eq!(value.f_replace(off!(b), None), Some(5));
    /// assert_eq!(moved(value.b), None);
    ///
    /// assert_eq!(value.f_replace(off!(c), vec![2, 3]), vec![0, 1]);
    /// assert_eq!(moved(value.c), vec![2, 3]);
    ///
    /// ```
    fn f_replace<F>(&mut self, offset: FieldOffset<Self, F, A>, value: F) -> F;

    /// Swaps a field (determined by `offset`) with the same field in `right`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::ReprC,
    ///     ROExtOps, off,
    /// };
    ///
    /// let mut left = ReprC {
    ///     a: 3u128,
    ///     b: Some(5u64),
    ///     c: vec![0, 1],
    ///     d: (),
    /// };
    /// let mut right = ReprC {
    ///     a: 55,
    ///     b: None,
    ///     c: vec![89, 144],
    ///     d: (),
    /// };
    ///
    /// left.f_swap(off!(a), &mut right);
    /// assert_eq!(left.a, 55);
    /// assert_eq!(right.a, 3);
    ///
    /// left.f_swap(off!(b), &mut right);
    /// assert_eq!(left.b, None);
    /// assert_eq!(right.b, Some(5));
    ///
    /// left.f_swap(off!(c), &mut right);
    /// assert_eq!(left.c, vec![89, 144]);
    /// assert_eq!(right.c, vec![0, 1]);
    ///
    /// ```
    fn f_swap<F>(&mut self, offset: FieldOffset<Self, F, A>, right: &mut Self);

    /// Gets a copy of a field (determined by `offset`).
    /// The field is determined by `offset`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::ReprPacked,
    ///     ROExtOps, off,
    /// };
    ///
    /// let value = ReprPacked {
    ///     a: 3,
    ///     b: "foo",
    ///     c: 'g',
    ///     d: false,
    /// };
    ///
    /// assert_eq!(value.f_get_copy(off!(a)), 3);
    /// assert_eq!(value.f_get_copy(off!(b)), "foo");
    /// assert_eq!(value.f_get_copy(off!(c)), 'g');
    /// assert_eq!(value.f_get_copy(off!(d)), false);
    ///
    ///
    /// ```
    fn f_get_copy<F>(&self, offset: FieldOffset<Self, F, A>) -> F
    where
        F: Copy;
}

/////////////////////////////////////////////////////////////////////////////////

/// Extension trait for raw pointers to access fields generically,
/// where the field is determined by a [`FieldOffset`] parameter.
///
///
/// # Safety
///
/// This trait must not to be implemented outside the `repr_offset` crate.
///
/// # Example
///
/// This example shows how you can get references to known aligned fields from a raw pointer
/// to a partially initialized value..
///
/// ```rust
/// # #![deny(safe_packed_borrows)]
/// use repr_offset::{
///     for_examples::ReprC,
///     off,
///     ROExtRawAcc, ROExtRawMutOps,
/// };
///
/// use std::mem::MaybeUninit;
///
/// type This = ReprC<u8, Option<usize>, &'static [u16], &'static str>;
///
/// let mut uninit = MaybeUninit::<This>::uninit();
///
/// unsafe {
///     let ptr = uninit.as_mut_ptr();
///     ptr.f_write(off!(a), 3);
///     ptr.f_write(off!(b), Some(5));
///
///     let (a, b) = get_init_refs(&uninit);
///     assert_eq!(*a, 3);
///     assert_eq!(*b, Some(5));
/// }
///
/// /// # Safety
/// ///
/// /// The fields up to and including `b` must be initialized.
/// unsafe fn get_init_refs(this: &MaybeUninit<This>) -> (&u8, &Option<usize>) {
///     let this = this.as_ptr();
///
///     (
///         &*this.f_raw_get(off!(a)),
///         &*this.f_raw_get(off!(b)),
///     )
/// }
///
/// ```
///
///
/// [`FieldOffset`]: ../struct.FieldOffset.html
//
// This trait is implemented in src/struct_field_offset/repr_offset_ext_impls.rs
pub unsafe trait ROExtRawAcc: crate::utils::PointerTarget {
    /// Gets a raw pointer to a field (determined by `offset`) from this raw pointer.
    ///
    /// # Safety
    ///
    /// `self` must point to some allocated object,
    /// allocated at least up to the field (inclusive).
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::{ReprC, ReprPacked},
    ///     tstr::TS,
    ///     GetPubFieldOffset, FieldType,
    ///     ROExtRawAcc,
    ///     pub_off,
    /// };
    ///
    /// use std::cmp::Ordering;
    ///
    /// let value = ReprPacked {
    ///     a: 3,
    ///     b: Some(5),
    ///     c: Ordering::Less,
    ///     d: ReprC {
    ///         a: 8,
    ///         b: "bar",
    ///         c: 13,
    ///         d: 21,
    ///     },
    /// };
    ///
    /// unsafe {
    ///     assert_eq!(copy_fields(&value), (3, 13));
    /// }
    ///
    ///
    /// /// Copies the `a` and `d.c` fields in this.
    /// ///
    /// /// # Safety
    /// ///
    /// /// The `a` and `d.c` fields in this must be initialized
    /// unsafe fn copy_fields<T, O, U>(
    ///     this: *const T,
    /// ) -> (O, U)
    /// where
    ///     T: GetPubFieldOffset<TS!(a), Type = O>,
    ///     T: GetPubFieldOffset<TS!(d,c), Type = U>,
    ///     O: Copy,
    ///     U: Copy,
    /// {
    ///     (
    ///         this.f_raw_get(pub_off!(a)).read_unaligned(),
    ///         this.f_raw_get(pub_off!(d.c)).read_unaligned(),
    ///     )
    /// }
    ///
    ///
    /// ```
    ///
    unsafe fn f_raw_get<F, A>(self, offset: FieldOffset<Self::Target, F, A>) -> *const F;
}

/// Extension trait for mutable raw pointers to access fields generically,
/// where the field is determined by a [`FieldOffset`] parameter.
///
///
/// # Safety
///
/// This trait must not to be implemented outside the `repr_offset` crate.
///
/// # Example
///
/// This example demonstrates how you can get mutable references to the known initialized
/// parts of a struct.
///
/// ```rust
/// # #![deny(safe_packed_borrows)]
/// use repr_offset::{
///     for_examples::ReprC,
///     tstr::TS,
///     off,
///     ROExtRawOps, ROExtRawMutAcc, ROExtRawMutOps,
/// };
///
/// use std::mem::MaybeUninit;
///
/// type This = ReprC<u8, Option<usize>, &'static [u16], &'static str>;
///
/// let mut uninit = MaybeUninit::<This>::uninit();
///
/// unsafe {
///     let ptr = uninit.as_mut_ptr();
///     ptr.f_write(off!(a), 3);
///     ptr.f_write(off!(b), Some(5));
/// }
/// {
///     // Safety: We know that the a and b fields were initialized above.
///     let (a, b) = unsafe{ get_mut_refs(&mut uninit) };
///     assert_eq!(*a, 3);
///     assert_eq!(*b, Some(5));
///     *a += 100;
///     *b.as_mut().unwrap() += 200;
/// }
/// unsafe {
///     let ptr = uninit.as_ptr();
///     assert_eq!(ptr.f_read_copy(off!(a)), 103);
///     assert_eq!(ptr.f_read_copy(off!(b)), Some(205));
/// }
///
/// /// Gets mutable references to the `a` and `b` fields in `this`
/// ///
/// /// # Safety
/// ///
/// /// The fields up to and including `b` must be initialized.
/// unsafe fn get_mut_refs(this: &mut MaybeUninit<This>) -> (&mut u8, &mut Option<usize>) {
///     let this = this.as_mut_ptr();
///
///     let ptrs = (
///         this.f_raw_get_mut(off!(a)),
///         this.f_raw_get_mut(off!(b)),
///     );
///
///     (&mut *ptrs.0, &mut *ptrs.1)
/// }
///
/// ```
///
/// [`FieldOffset`]: ../struct.FieldOffset.html
//
// This trait is implemented in src/struct_field_offset/repr_offset_ext_impls.rs
pub unsafe trait ROExtRawMutAcc: ROExtRawAcc {
    /// Gets a muatble pointer to a field (determined by `offset`) from this mutable pointer.
    ///
    /// # Safety
    ///
    /// `self` must point to some allocated object,
    /// allocated at least up to the field (inclusive).
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::{ReprC, ReprPacked},
    ///     utils::moved,
    ///     tstr::TS,
    ///     GetPubFieldOffset, FieldType,
    ///     ROExtRawMutAcc,
    ///     off,
    /// };
    ///
    /// use std::mem::MaybeUninit;
    ///
    /// type This = ReprPacked<Option<char>, ReprC<u32, u64, String, Vec<u32>>, bool>;
    ///
    /// let mut uninit = MaybeUninit::<This>::uninit();
    ///
    /// /// Initializes a `This` through a pointer
    /// ///
    /// /// # Safety
    /// ///
    /// /// You must pass a pointer to allocated (and writable) memory for `This`.
    /// unsafe fn initialize(this: *mut This) {
    ///     this.f_raw_get_mut(off!(a)).write_unaligned(None);
    ///     this.f_raw_get_mut(off!(b.a)).write_unaligned(3);
    ///     this.f_raw_get_mut(off!(b.b)).write_unaligned(5);
    ///     this.f_raw_get_mut(off!(b.c)).write_unaligned("8".to_string());
    ///     this.f_raw_get_mut(off!(b.d)).write_unaligned(vec![13, 21]);
    ///     this.f_raw_get_mut(off!(c)).write_unaligned(false);
    /// }
    ///
    /// let value = unsafe{
    ///     initialize(uninit.as_mut_ptr());
    ///     uninit.assume_init()
    /// };
    ///
    /// assert_eq!(moved(value.a), None);
    /// assert_eq!(moved(value.b.a), 3);
    /// assert_eq!(moved(value.b.b), 5);
    /// assert_eq!(moved(value.b.c), "8".to_string());
    /// assert_eq!(moved(value.b.d), vec![13, 21]);
    /// assert_eq!(moved(value.c), false);
    ///
    /// ```
    ///
    unsafe fn f_raw_get_mut<F, A>(self, offset: FieldOffset<Self::Target, F, A>) -> *mut F;
}

/// Extension trait for raw pointers to do generic field operations,
/// where the field is determined by a [`FieldOffset`] parameter.
///
///
/// # Safety
///
/// This trait must not to be implemented outside the `repr_offset` crate.
///
/// # Alignment
///
/// The `A` type parameter is the [`Alignment`] of the field,
/// used to implement methods differently depending on whether the field is
/// [`Aligned`] or [`Unaligned`].
///
/// [`FieldOffset`]: ../struct.FieldOffset.html
/// [`Alignment`]: ../alignment/trait.Alignment.html
/// [`Aligned`]: ../alignment/struct.Aligned.html
/// [`Unaligned`]: ../alignment/struct.Unaligned.html
//
// This trait is implemented in src/struct_field_offset/repr_offset_ext_impls.rs
pub unsafe trait ROExtRawOps<A>: ROExtRawAcc {
    /// Copies a field (determined by `offset`) from `self`.
    ///
    /// # Safety
    ///
    /// You must ensure these properties about the pointed-to value:
    ///
    /// - The value must be in an allocated object (this includes the stack)
    /// allocated at least up to the field (inclusive).
    ///
    /// - The field must be initialized
    ///
    /// - If the passed in `offset` is a `FieldOffset<_, _, Aligned>`
    /// (because it is for an aligned field), `self` must be an aligned pointer.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::ReprPacked,
    ///     ROExtRawOps, off,
    /// };
    ///
    /// use std::cmp::Ordering;
    ///
    /// let mut value = ReprPacked {
    ///     a: 3,
    ///     b: Some(5),
    ///     c: Ordering::Less,
    ///     d: (),
    /// };
    ///
    /// let ptr: *const _ = &value;
    /// unsafe {
    ///     assert_eq!(ptr.f_read_copy(off!(a)), 3);
    ///     assert_eq!(ptr.f_read_copy(off!(b)), Some(5));
    ///     assert_eq!(ptr.f_read_copy(off!(c)), Ordering::Less);
    /// }
    /// ```
    ///
    unsafe fn f_read_copy<F>(self, offset: FieldOffset<Self::Target, F, A>) -> F
    where
        F: Copy;

    /// Reads a copy of a field (determined by `offset`) from `self`,
    /// without mutating or moving the field.
    ///
    /// # Safety
    ///
    /// You must ensure these properties about the pointed-to value:
    ///
    /// - The value must be in an allocated object (this includes the stack)
    /// allocated at least up to the field (inclusive).
    ///
    /// - The field must be initialized
    ///
    /// - If the passed in `offset` is a `FieldOffset<_, _, Aligned>`
    /// (because it is for an aligned field), `self` must be an aligned pointer.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::ReprPacked,
    ///     ROExtRawOps, off,
    /// };
    ///
    /// use std::{cmp::Ordering, mem::ManuallyDrop};
    ///
    /// let mut value = ManuallyDrop::new(ReprPacked {
    ///     a: 3,
    ///     b: Some(5),
    ///     c: "hello".to_string(),
    ///     d: vec![0, 1, 2],
    /// });
    ///
    /// let ptr: *const ReprPacked<_, _, _, _> = &*value;
    /// unsafe {
    ///     assert_eq!(ptr.f_read(off!(a)), 3);
    ///     assert_eq!(ptr.f_read(off!(b)), Some(5));
    ///     assert_eq!(ptr.f_read(off!(c)), "hello".to_string());
    ///     assert_eq!(ptr.f_read(off!(d)), vec![0, 1, 2]);
    /// }
    /// ```
    ///
    unsafe fn f_read<F>(self, offset: FieldOffset<Self::Target, F, A>) -> F;
}

/// Extension trait for mutable raw pointers to do generic field operations,
/// where the field is determined by a [`FieldOffset`] parameter.
///
/// # Safety
///
/// This trait must not to be implemented outside the `repr_offset` crate.
///
/// # Alignment
///
/// The `A` type parameter is the [`Alignment`] of the field,
/// used to implement methods differently depending on whether the field is
/// [`Aligned`] or [`Unaligned`].
///
/// [`FieldOffset`]: ../struct.FieldOffset.html
/// [`Alignment`]: ../alignment/trait.Alignment.html
/// [`Aligned`]: ../alignment/struct.Aligned.html
/// [`Unaligned`]: ../alignment/struct.Unaligned.html
///
/// # Example
///
/// This example shows how you can do the equivalent of `std::mem::replace`
/// in partially initialized structs.
///
/// ```rust
/// # #![deny(safe_packed_borrows)]
/// use repr_offset::{
///     for_examples::ReprPacked,
///     utils::moved,
///     ROExtRawMutOps, off,
/// };
///
/// let mut value = ReprPacked {
///     a: false,
///     b: None,
///     c: "",
///     d: [0u64; 10],
/// };
///
/// let (a, b) = unsafe{ replace_fields(&mut value, true, Some('!')) };
///
/// assert_eq!(moved(a), false);
/// assert_eq!(moved(value.a), true);
///
/// assert_eq!(moved(b), None);
/// assert_eq!(moved(value.b), Some('!'));
///
/// assert_eq!(moved(value.c), "");
///
///
/// /// Replaces the `a` and `b` fields in `this`
/// ///
/// /// # Safety
/// ///
/// /// The fields up to and including `b` must be initialized.
/// unsafe fn replace_fields(
///     this: *mut ReprPacked<bool, Option<char>, &'static str, [u64; 10]>,
///     a: bool,
///     b: Option<char>,
/// ) -> (bool, Option<char>) {
///     (
///         this.f_replace_raw(off!(a), a),
///         this.f_replace_raw(off!(b), b),
///     )
/// }
/// ```
///
//
// This trait is implemented in src/struct_field_offset/repr_offset_ext_impls.rs
pub unsafe trait ROExtRawMutOps<A>: ROExtRawMutAcc {
    /// Overwrites the value of a field (determined by `offset`) from `self`,
    /// without dropping the previous value.
    ///
    /// # Safety
    ///
    /// You must ensure these properties:
    ///
    /// - `self` must point to an allocated object (this includes the stack)
    /// allocated at least up to the field (inclusive).
    ///
    /// - If the passed in `offset` is a `FieldOffset<_, _, Aligned>`
    /// (because it is for an aligned field), `self` must be an aligned pointer.
    ///
    /// - The field must be writable(if in doubt, all of the pointed-to value).
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::ReprC,
    ///     utils::moved,
    ///     ROExtRawMutOps, off,
    /// };
    ///
    /// let mut value = ReprC {
    ///     a: 0,
    ///     b: None::<u32>,
    ///     c: Vec::new(),
    ///     d: String::new(),
    /// };
    ///
    /// let ptr: *mut _ = &mut value;
    /// unsafe{
    ///     ptr.f_write(off!(a), 3);
    ///     ptr.f_write(off!(b), Some(5));
    ///     ptr.f_write(off!(c), vec![8, 13]);
    ///     ptr.f_write(off!(d), "world".to_string());
    /// }
    ///
    /// assert_eq!(value.a, 3);
    /// assert_eq!(value.b, Some(5));
    /// assert_eq!(value.c, vec![8, 13]);
    /// assert_eq!(value.d, "world".to_string());
    ///
    /// ```
    ///
    unsafe fn f_write<F>(self, offset: FieldOffset<Self::Target, F, A>, value: F);

    /// Copies a field (determined by `offset`) from `source` to `self`.
    ///
    /// # Safety
    ///
    /// You must ensure these properties:
    ///
    /// - `self` and `source` must point to an allocated object (this includes the stack)
    /// allocated at lest up to the field (inclusive).
    ///
    /// - If the passed in `offset` is a `FieldOffset<_, _, Aligned>`
    /// (because it is for an aligned field), both `self` and `source` must be aligned pointers.
    ///
    /// - The field must be writable (if in doubt, all of the pointed-to value must be writble).
    ///
    /// [`core::ptr::copy`] describes what happens when `self` ànd `source` overlap.
    ///
    ///
    /// [`core::ptr::copy`]: https://doc.rust-lang.org/core/ptr/fn.copy.html
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::ReprC,
    ///     ROExtRawMutOps, off,
    /// };
    ///
    /// let mut left = ReprC {
    ///     a: 3u128,
    ///     b: Some(5u64),
    ///     c: &[8, 13, 21][..],
    ///     d: (),
    /// };
    /// let right = ReprC {
    ///     a: 55,
    ///     b: None,
    ///     c: &[34, 51, 89][..],
    ///     d: (),
    /// };
    ///
    /// let left_ptr: *mut _ = &mut left;
    /// unsafe{
    ///     left_ptr.f_copy_from(off!(a), &right);
    ///     left_ptr.f_copy_from(off!(b), &right);
    ///     left_ptr.f_copy_from(off!(c), &right);
    /// }
    ///
    /// assert_eq!(left.a, 55);
    /// assert_eq!(right.a, 55);
    ///
    /// assert_eq!(left.b, None);
    /// assert_eq!(right.b, None);
    ///
    /// assert_eq!(left.c, &[34, 51, 89][..]);
    /// assert_eq!(right.c, &[34, 51, 89][..]);
    ///
    ///
    /// ```
    ///
    unsafe fn f_copy_from<F>(
        self,
        offset: FieldOffset<Self::Target, F, A>,
        source: *const Self::Target,
    );

    /// Copies a field (determined by `offset`) from `source` to `self`.
    ///
    /// # Safety
    ///
    /// You must ensure these properties:
    ///
    /// - `self` and `source` must point to an allocated object (this includes the stack)
    /// allocated at lest up to the field (inclusive).
    ///
    /// - If the passed in `offset` is a `FieldOffset<_, _, Aligned>`
    /// (because it is for an aligned field), both `self` and `source` must be aligned pointers.
    ///
    /// - The field must be writable (if in doubt, all of the pointed-to value must be writble).
    ///
    /// - The field in `self` and the same field in `source` must not overlap,
    /// (if in doubt, the pointers must not point to overlapping memory).
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::ReprPacked,
    ///     utils::moved,
    ///     ROExtRawMutOps, off,
    /// };
    ///
    /// let mut left = ReprPacked {
    ///     a: false,
    ///     b: None,
    ///     c: "foo",
    ///     d: (),
    /// };
    /// let right = ReprPacked {
    ///     a: true,
    ///     b: Some('?'),
    ///     c: "bar",
    ///     d: (),
    /// };
    ///
    /// let left_ptr: *mut _ = &mut left;
    /// unsafe{
    ///     left_ptr.f_copy_from_nonoverlapping(off!(a), &right);
    ///     left_ptr.f_copy_from_nonoverlapping(off!(b), &right);
    ///     left_ptr.f_copy_from_nonoverlapping(off!(c), &right);
    /// }
    ///
    /// assert_eq!(moved(left.a), true);
    /// assert_eq!(moved(right.a), true);
    ///
    /// assert_eq!(moved(left.b), Some('?'));
    /// assert_eq!(moved(right.b), Some('?'));
    ///
    /// assert_eq!(moved(left.c), "bar");
    /// assert_eq!(moved(right.c), "bar");
    ///
    ///
    /// ```
    ///
    unsafe fn f_copy_from_nonoverlapping<F>(
        self,
        offset: FieldOffset<Self::Target, F, A>,
        source: *const Self::Target,
    );

    /// Replaces a field (determined by `offset`) with `value`,
    /// returning the previous value of the field.
    ///
    /// # Safety
    ///
    /// You must ensure these properties:
    ///
    /// - `self` must point to an allocated object (this includes the stack)
    /// allocated at lest up to the field (inclusive).
    ///
    /// - If the passed in `offset` is a `FieldOffset<_, _, Aligned>`
    /// (because it is for an aligned field), `self` must be an aligned pointers.
    ///
    /// [`core::ptr::copy`]: https://doc.rust-lang.org/core/ptr/fn.copy.html
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::ReprPacked,
    ///     utils::moved,
    ///     ROExtRawMutOps, off,
    /// };
    ///
    /// let mut value = ReprPacked {
    ///     a: 3u128,
    ///     b: Some(5u64),
    ///     c: vec![0, 1],
    ///     d: (),
    /// };
    ///
    /// let ptr: *mut _ = &mut value;
    /// unsafe {
    ///     assert_eq!(ptr.f_replace_raw(off!(a), 200), 3);
    ///     assert_eq!(ptr.f_replace_raw(off!(b), None), Some(5));
    ///     assert_eq!(ptr.f_replace_raw(off!(c), vec![2, 3]), vec![0, 1]);
    /// }
    ///
    /// assert_eq!(moved(value.a), 200);
    /// assert_eq!(moved(value.b), None);
    /// assert_eq!(moved(value.c), vec![2, 3]);
    ///
    /// ```
    unsafe fn f_replace_raw<F>(self, offset: FieldOffset<Self::Target, F, A>, value: F) -> F;

    /// Swaps a field (determined by `offset`) from `self` with the same field in `right`.
    ///
    /// # Safety
    ///
    /// You must ensure these properties:
    ///
    /// - `self` and `source` must point to an allocated object (this includes the stack)
    /// allocated at lest up to the field (inclusive).
    ///
    /// - If the passed in `offset` is a `FieldOffset<_, _, Aligned>`
    /// (because it is for an aligned field), both `self` and `source` must be aligned pointers.
    ///
    /// - The field in `self` and the same field in `source` must be writable
    /// (if in doubt, all of the pointed-to value must be writble).
    ///
    ///
    /// [`core::ptr::swap`] describes what happens when `self` ànd `source` overlap.
    ///
    ///
    /// [`core::ptr::swap`]: https://doc.rust-lang.org/core/ptr/fn.swap.html
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::ReprC,
    ///     ROExtRawMutOps, off,
    /// };
    ///
    /// let mut left = ReprC {
    ///     a: 3u128,
    ///     b: Some(5u64),
    ///     c: &[8, 13, 21][..],
    ///     d: (),
    /// };
    /// let mut right = ReprC {
    ///     a: 55,
    ///     b: None,
    ///     c: &[34, 51, 89][..],
    ///     d: (),
    /// };
    ///
    /// let left_ptr: *mut _ = &mut left;
    /// unsafe{
    ///     left_ptr.f_swap_raw(off!(a), &mut right);
    ///     left_ptr.f_swap_raw(off!(b), &mut right);
    ///     left_ptr.f_swap_raw(off!(c), &mut right);
    /// }
    ///
    /// assert_eq!(left.a, 55);
    /// assert_eq!(right.a, 3);
    ///
    /// assert_eq!(left.b, None);
    /// assert_eq!(right.b, Some(5));
    ///
    /// assert_eq!(left.c, &[34, 51, 89][..]);
    /// assert_eq!(right.c, &[8, 13, 21][..]);
    ///
    ///
    /// ```
    ///
    unsafe fn f_swap_raw<F>(
        self,
        offset: FieldOffset<Self::Target, F, A>,
        right: *mut Self::Target,
    );

    /// Swaps a field (determined by `offset`) from `self` with the same field in `right`.
    /// `self` and `right` must not overlap.
    ///
    ///
    /// # Safety
    ///
    /// You must ensure these properties:
    ///
    /// - `self` and `source` must point to an allocated object (this includes the stack)
    /// allocated at lest up to the field (inclusive).
    ///
    /// - If the passed in `offset` is a `FieldOffset<_, _, Aligned>`
    /// (because it is for an aligned field), both `self` and `source` must be aligned pointers.
    ///
    /// - The field in `self` and the same field in `source` must be writable
    /// (if in doubt, all of the pointed-to value must be writble).
    ///
    /// - The field in `self` and the same field in `source` must not overlap,
    /// (if in doubt, the pointers must not point to overlapping memory).
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{
    ///     for_examples::ReprPacked,
    ///     utils::moved,
    ///     ROExtRawMutOps, off,
    /// };
    ///
    /// let mut left = ReprPacked {
    ///     a: false,
    ///     b: None,
    ///     c: "foo",
    ///     d: (),
    /// };
    /// let mut right = ReprPacked {
    ///     a: true,
    ///     b: Some('?'),
    ///     c: "bar",
    ///     d: (),
    /// };
    ///
    /// let left_ptr: *mut _ = &mut left;
    /// unsafe{
    ///     left_ptr.f_swap_nonoverlapping(off!(a), &mut right);
    ///     left_ptr.f_swap_nonoverlapping(off!(b), &mut right);
    ///     left_ptr.f_swap_nonoverlapping(off!(c), &mut right);
    /// }
    ///
    /// assert_eq!(moved(left.a), true);
    /// assert_eq!(moved(right.a), false);
    ///
    /// assert_eq!(moved(left.b), Some('?'));
    /// assert_eq!(moved(right.b), None);
    ///
    /// assert_eq!(moved(left.c), "bar");
    /// assert_eq!(moved(right.c), "foo");
    ///
    ///
    /// ```
    ///
    unsafe fn f_swap_nonoverlapping<F>(
        self,
        offset: FieldOffset<Self::Target, F, A>,
        right: *mut Self::Target,
    );
}
