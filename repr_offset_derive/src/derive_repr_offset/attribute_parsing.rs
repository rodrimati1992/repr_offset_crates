use as_derive_utils::{
    attribute_parsing::with_nested_meta,
    datastructure::{DataStructure, DataVariant, Field, FieldMap},
    return_spanned_err, return_syn_err, spanned_err,
    utils::{LinearResult, SynResultExt},
};

use core_extensions::matches;

use proc_macro2::Span;

use quote::ToTokens;

use syn::{Attribute, Ident, Meta, MetaList, MetaNameValue, WherePredicate};

use std::marker::PhantomData;

pub(crate) struct ReprOffsetConfig<'a> {
    pub(crate) debug_print: bool,
    // If there was a #[repr(packed)]
    pub(crate) is_packed: bool,
    pub(crate) starting_offset: Option<syn::Expr>,
    pub(crate) use_usize_offsets: bool,
    pub(crate) offset_prefix: Ident,
    pub(crate) field_map: FieldMap<FieldConfig>,
    pub(crate) extra_bounds: Vec<WherePredicate>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ReprOffsetConfig<'a> {
    fn new(roa: ReprOffsetAttrs<'a>) -> Result<Self, syn::Error> {
        let ReprOffsetAttrs {
            debug_print,
            is_packed,
            is_repr_stable,
            starting_offset,
            use_usize_offsets,
            offset_prefix,
            field_map,
            extra_bounds,
            errors: _,
            _marker: PhantomData,
        } = roa;

        if !is_repr_stable {
            return_syn_err! {
                Span::call_site(),
                "Expected a struct with `#[repr(C)]` or `#[repr(transparent)]` attributes."
            }
        }

        Ok(Self {
            debug_print,
            is_packed,
            starting_offset,
            use_usize_offsets,
            offset_prefix,
            field_map,
            extra_bounds,
            _marker: PhantomData,
        })
    }
}

struct ReprOffsetAttrs<'a> {
    debug_print: bool,
    // If there was a #[repr(packed)]
    is_packed: bool,
    // If there was a #[repr(transparent)] or #[repr(C)] attribute
    is_repr_stable: bool,
    starting_offset: Option<syn::Expr>,
    use_usize_offsets: bool,
    offset_prefix: Ident,
    field_map: FieldMap<FieldConfig>,
    extra_bounds: Vec<WherePredicate>,
    errors: LinearResult<()>,
    _marker: PhantomData<&'a ()>,
}

pub(crate) struct FieldConfig {
    pub(crate) offset_name: Option<OffsetIdent>,
}

pub(crate) enum OffsetIdent {
    Prefix(Ident),
    Full(Ident),
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
enum ParseContext<'a> {
    TypeAttr { data_variant: DataVariant },
    Field { field: &'a Field<'a> },
}

pub(crate) fn parse_attrs_for_derive<'a>(
    ds: &'a DataStructure<'a>,
) -> Result<ReprOffsetConfig<'a>, syn::Error> {
    let mut this = ReprOffsetAttrs {
        debug_print: false,
        is_packed: false,
        is_repr_stable: false,
        starting_offset: None,
        use_usize_offsets: false,
        offset_prefix: Ident::new("OFFSET_", Span::call_site()),
        field_map: FieldMap::with(ds, |_| FieldConfig { offset_name: None }),
        extra_bounds: vec![],
        errors: LinearResult::ok(()),
        _marker: PhantomData,
    };

    let ty_ctx = ParseContext::TypeAttr {
        data_variant: ds.data_variant,
    };
    parse_inner(&mut this, ds.attrs, ty_ctx)?;

    for variant in &ds.variants {
        for field in variant.fields.iter() {
            parse_inner(&mut this, field.attrs, ParseContext::Field { field })?;
        }
    }

    this.errors.take()?;

    ReprOffsetConfig::new(this)
}

/// Parses an individual attribute
fn parse_inner<'a, I>(
    this: &mut ReprOffsetAttrs<'a>,
    attrs: I,
    pctx: ParseContext<'a>,
) -> Result<(), syn::Error>
where
    I: IntoIterator<Item = &'a Attribute>,
{
    for attr in attrs {
        match attr.parse_meta() {
            Ok(Meta::List(list)) => {
                parse_attr_list(this, pctx, list).combine_into_err(&mut this.errors);
            }
            Err(e) => {
                this.errors.push_err(e);
            }
            _ => {}
        }
    }
    Ok(())
}

/// Parses an individual attribute list (A `#[attribute( .. )] attribute`).
fn parse_attr_list<'a>(
    this: &mut ReprOffsetAttrs<'a>,
    pctx: ParseContext<'a>,
    list: MetaList,
) -> Result<(), syn::Error> {
    if list.path.is_ident("roff") {
        with_nested_meta("roff", list.nested, |attr| {
            parse_sabi_attr(this, pctx, attr).combine_into_err(&mut this.errors);
            Ok(())
        })?;
    } else if list.path.is_ident("repr") && matches!(ParseContext::TypeAttr{..} = pctx) {
        with_nested_meta("repr", list.nested, |attr| {
            let path = attr.path();
            if path.is_ident("C") || path.is_ident("transparent") {
                this.is_repr_stable = true;
            } else if path.is_ident("packed") {
                this.is_packed = true;
            }
            Ok(())
        })?;
    }

    Ok(())
}

/// Parses the contents of a `#[sabi( .. )]` attribute.
fn parse_sabi_attr<'a>(
    this: &mut ReprOffsetAttrs<'a>,
    pctx: ParseContext<'a>,
    attr: Meta,
) -> Result<(), syn::Error> {
    fn make_err(tokens: &dyn ToTokens) -> syn::Error {
        spanned_err!(tokens, "unrecognized attribute")
    }
    match (pctx, attr) {
        (ParseContext::Field { field, .. }, Meta::NameValue(MetaNameValue { lit, path, .. })) => {
            let f_config = &mut this.field_map[field.index];
            if path.is_ident("offset") {
                f_config.offset_name = Some(OffsetIdent::Full(parse_lit(&lit)?));
            } else if path.is_ident("offset_prefix") {
                f_config.offset_name = Some(OffsetIdent::Prefix(parse_lit(&lit)?));
            } else {
                return Err(make_err(&path));
            }
        }
        (ParseContext::TypeAttr { .. }, Meta::Path(path)) => {
            if path.is_ident("debug_print") {
                this.debug_print = true;
            } else if path.is_ident("usize_offsets") {
                this.use_usize_offsets = true;
            } else {
                return Err(make_err(&path));
            }
        }
        (ParseContext::TypeAttr { .. }, Meta::NameValue(MetaNameValue { lit, path, .. })) => {
            let ident = path.get_ident().ok_or_else(|| make_err(&path))?;

            if ident == "offset_prefix" {
                this.offset_prefix = parse_lit(&lit)?;
            } else if ident == "unsafe_starting_offset" {
                this.starting_offset = Some(parse_expr(lit)?);
            } else if ident == "bound" {
                this.extra_bounds.push(parse_lit(&lit)?);
            } else {
                return Err(make_err(&path));
            }
        }
        (_, x) => return Err(make_err(&x)),
    }
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////

fn parse_lit<T>(lit: &syn::Lit) -> Result<T, syn::Error>
where
    T: syn::parse::Parse,
{
    match lit {
        syn::Lit::Str(x) => x.parse(),
        _ => Err(spanned_err!(
            lit,
            "Expected string literal containing identifier"
        )),
    }
}

fn parse_expr(lit: syn::Lit) -> Result<syn::Expr, syn::Error> {
    match lit {
        syn::Lit::Str(x) => x.parse(),
        syn::Lit::Int(x) => syn::parse_str(x.base10_digits()),
        _ => return_spanned_err!(lit, "Expected string or integer literal"),
    }
}
