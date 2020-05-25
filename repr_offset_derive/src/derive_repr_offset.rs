use as_derive_utils::{
    datastructure::{DataStructure, DataVariant},
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
    let output = derive_inner(&ds, &options)?;
    if options.debug_print {
        panic!("\n\n\n{}\n\n\n", output);
    }
    Ok(output)
}

fn derive_inner(
    ds: &DataStructure<'_>,
    options: &ReprOffsetConfig<'_>,
) -> Result<TokenStream2, syn::Error> {
    let alignment = if options.is_packed {
        quote!(Unaligned)
    } else {
        quote!(Aligned)
    };

    let usize_offsets = options.use_usize_offsets;
    let starting_offset = options.starting_offset.iter();

    let impl_generics = GenParamsIn::new(ds.generics, InWhat::ImplHeader);

    let name = ds.name;
    let (_, ty_generics, _) = ds.generics.split_for_impl();

    let empty_punct = syn::punctuated::Punctuated::new();
    let where_preds = ds
        .generics
        .where_clause
        .as_ref()
        .map_or(&empty_punct, |x| &x.predicates);

    let struct_ = &ds.variants[0];

    let vis = struct_.fields.iter().map(|x| x.vis);
    let offset_name = struct_.fields.iter().map(|field| {
        ToTokenFnMut::new(move |ts| {
            let f_conf = &options.field_map[field.index];
            match &f_conf.offset_name {
                None => concat_field_ident(&options.offset_prefix, field.ident()).to_tokens(ts),
                Some(OffsetIdent::Prefix(prefix)) => {
                    concat_field_ident(prefix, field.ident()).to_tokens(ts)
                }
                Some(OffsetIdent::Full(full)) => full.to_tokens(ts),
            }
        })
    });
    let field_tys = struct_.fields.iter().map(|x| x.ty);

    Ok(quote! {
        ::repr_offset::unsafe_struct_field_offsets!{
            alignment = ::repr_offset::#alignment,
            usize_offsets = #usize_offsets,
            #( starting_offset = #starting_offset, )*

            impl[#impl_generics] #name #ty_generics
            where[ #where_preds ]
            {
                #(
                    #vis const #offset_name: #field_tys;
                )*
            }
        }
    })
}

fn concat_field_ident(prefix: &Ident, field_name: &Ident) -> Ident {
    Ident::new(
        &format!("{}{}", prefix, field_name.to_string().to_uppercase()),
        field_name.span(),
    )
}