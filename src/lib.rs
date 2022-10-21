use std::{borrow::Borrow};

use proc_macro::{TokenStream};
use proc_macro2::{TokenStream as TokenStream2, Span};
use quote::{quote, format_ident};
use syn::token::For;
use syn::{parse_macro_input, ItemImpl, ImplItem, Visibility, Type};

trait TraitA<T> : {
}

#[proc_macro_attribute]
pub fn generate_trait(_args: TokenStream, input: TokenStream) ->  
  TokenStream 
{
  let mut item_impl = parse_macro_input!(input as ItemImpl);
  assert!(item_impl.trait_.is_none());
  
  let mut item_trait = {
    // TODO: allow visibility override

    let generics_params = &item_impl.generics.params;
    let generics_where_clause = &item_impl.generics.where_clause;

    // TODO: allow name override

    let ident = if let Type::Path(self_ty_path) = item_impl.self_ty.borrow() {
      if let Some(path_last_seg) = self_ty_path.path.segments.last() {
        format_ident!("Is{}", path_last_seg.ident)
      } else { panic!() }
    } else { panic!() };

    // TODO: do super traits
    // TODO: default visibility should be Inherit

    item_impl.trait_ = Some((None, ident.clone().into(), For { span: Span::mixed_site() }));
    quote!(pub trait #ident <#generics_params> : #generics_where_clause)
  };

  let mut trait_items = TokenStream2::new();
  for item in &mut item_impl.items {
    // TODO: Allow skip attribute
    // TODO: match Const and Type

    if let ImplItem::Method(item_method) = item {
      item_method.vis = Visibility::Inherited;

      let item_method_sig = &item_method.sig;
      trait_items.extend(quote!(#item_method_sig;).into_iter());
    }
  }
  item_trait.extend(quote!({#trait_items}).into_iter());

  quote!(#item_trait #item_impl).into()
}