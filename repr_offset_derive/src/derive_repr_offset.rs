use as_derive_utils::{
    datastructure::{DataStructure, DataVariant, FieldIdent},
    gen_params_in::{GenParamsIn, InWhat},
    return_syn_err, ToTokenFnMut,
};

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, ToTokens};

use syn::{DeriveInput, Ident};

////////////////////////////////////////////////////////////////////////////////

mod attribute_parsing;

use self::attribute_parsing::{OffsetIdent, ReprOffsetConfig};

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn derive(data: DeriveInput) -> Result<TokenStream2, syn::Error> {
    let ds = &DataStructure::new(&data);

    match ds.data_variant {
        DataVariant::Enum => {
            return_syn_err!(Span::call_site(), "Cannot derive ReprOffset on enums yet")
        }
        DataVariant::Union => return_syn_err!(
            Span::call_site(),
            "Cannot derive ReprOffset on a unions yet"
        ),
        DataVariant::Struct => {}
    }

    let options = attribute_parsing::parse_attrs_for_derive(ds)?;
    let output = derive_inner(&ds, &options);
    if options.debug_print {
        panic!("\n\n\n{}\n\n\n", output);
    }
    Ok(output)
}

fn derive_inner(ds: &DataStructure<'_>, options: &ReprOffsetConfig<'_>) -> TokenStream2 {
    let alignment = if options.is_packed {
        quote!(Unaligned)
    } else {
        quote!(Aligned)
    };

    let usize_offsets = options.use_usize_offsets;
    let impl_getfieldoffset = options.impl_getfieldoffset;

    let impl_generics = GenParamsIn::new(ds.generics, InWhat::ImplHeader);

    let name = ds.name;
    let (_, ty_generics, _) = ds.generics.split_for_impl();

    let empty_punct = syn::punctuated::Punctuated::new();
    let where_preds = ds
        .generics
        .where_clause
        .as_ref()
        .map_or(&empty_punct, |x| &x.predicates)
        .iter();

    let struct_ = &ds.variants[0];

    let vis = struct_.fields.iter().map(|x| x.vis);
    let offset_doc = struct_.fields.iter().map(|field| {
        if field.is_public() {
            format!("The offset of the `{}` field.", field.ident())
        } else {
            String::new()
        }
    });
    let offset_name = struct_.fields.iter().map(|field| {
        ToTokenFnMut::new(move |ts| {
            let f_conf = &options.field_map[field.index];
            match &f_conf.offset_name {
                None => concat_field_ident(&options.offset_prefix, &field.ident).to_tokens(ts),
                Some(OffsetIdent::Prefix(prefix)) => {
                    concat_field_ident(prefix, &field.ident).to_tokens(ts)
                }
                Some(OffsetIdent::Full(full)) => full.to_tokens(ts),
            }
        })
    });
    let field_names = struct_.fields.iter().map(|x| &x.ident);
    let field_tys = struct_.fields.iter().map(|x| x.ty);

    let extra_bounds = options.extra_bounds.iter();

    quote! {
        ::repr_offset::unsafe_struct_field_offsets!{
            alignment = ::repr_offset::#alignment,
            usize_offsets = #usize_offsets,
            impl_GetFieldOffset = #impl_getfieldoffset,

            impl[#impl_generics] #name #ty_generics
            where[
                #( #extra_bounds , )*
                #( #where_preds , )*
            ]{
                #(
                    #[doc = #offset_doc]
                    #vis const #offset_name, #field_names: #field_tys;
                )*
            }
        }
    }
}

fn concat_field_ident(prefix: &Ident, field_name: &FieldIdent<'_>) -> Ident {
    Ident::new(
        &format!("{}{}", prefix, field_name.to_string().to_uppercase()),
        field_ident_span(field_name),
    )
}

// Too lazy to add this to FieldIdent
fn field_ident_span(this: &FieldIdent<'_>) -> Span {
    match this {
        FieldIdent::Index(_, ref ident) => ident,
        FieldIdent::Named(ident) => ident,
    }
    .span()
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_cases() {
    use as_derive_utils::test_framework::Tests;

    Tests::load("repr_offset").run_test(|s| syn::parse_str(s).and_then(derive));
}
