mod parsing;

use darling::{ast::Data, FromDeriveInput};
use proc_macro2::{Group, Ident, Literal, Span, TokenStream, TokenTree};
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::quote;
use syn::parse_macro_input;

use crate::parsing::PatchStruct;

#[proc_macro_error]
#[proc_macro_derive(Patch, attributes(patch))]
pub fn derive_patch(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match PatchStruct::from_derive_input(&parse_macro_input!(item as DeriveInput)) {
        Ok(input) => input,
        Err(e) => {
            return e.write_errors().into();
        }
    };
    dbg!(input);

    let Data::Struct(struct_data) = input.data else {
        abort!("rust-patch only supports structs");
    };

    let mut apply_sets = Vec::new();
    for field in struct_data.fields {
        
    }
    todo!()
    /*

    let mut apply_sets = Vec::new();
    for (name, ty, attrs) in fields {
        let Type::Path(TypePath { path, .. }) = &ty else { abort!(&ty, "Failed parsing field type as type path") };
        let Some(ident) = path.segments.first().map(|e| &e.ident) else { abort!(&ty, "Field does not contain a valid ident") };
        let mut direct = false;
        let mut as_option = false;
        for attr in get_patch_attrs(attrs) {
            let span = attr.span();
            let content = match syn::parse2(attr) {
                Ok(Meta::List(content)) => content,
                Err(e) => abort!(span, "Failed parsing attribute: {}", e),
            };
            match content.to_string().as_str() {
                "direct" => direct = true,
                "as_option" => as_option = true,
                a => {
                    abort!(span, "Unknown attribute `{}`", a)
                }
            }
        }
        if direct && as_option {
            abort!(&ty, "Only one of `#[patch(direct)]` or `#[patch(as_option)]` may be specified for given field");
        }
        if as_option {
            apply_sets.push(quote! {
                if self.#name.is_some() {
                    target.#name = self.#name;
                }
            })
        } else if &ident.to_string() == "Option" && !direct {
            apply_sets.push(quote! {
                if let Some(val) = self.#name {
                    target.#name = val;
                }
            });
        } else {
            apply_sets.push(quote! {
                target.#name = self.#name;
            });
        }
    }

    let apply_content = quote! {
        #(
            #apply_sets
        )*
    };

    let output = quote! {
        #(
            impl ::rust_patch::Patch<#targets> for #ident {
                fn apply(self, mut target: #targets) -> #targets {
                    { #apply_content }
                    target
                }
            }
        )*
    };

    proc_macro::TokenStream::from(output)

     */
}

// Taken from https://github.com/serde-rs/serde/blob/master/serde_derive/src/internals
fn parse_lit_str<T>(s: &syn::LitStr) -> syn::parse::Result<T>
where
    T: Parse,
{
    let tokens = spanned_tokens(s)?;
    syn::parse2(tokens)
}

fn spanned_tokens(s: &syn::LitStr) -> syn::parse::Result<TokenStream> {
    let stream = syn::parse_str(&s.value())?;
    Ok(respan(stream, s.span()))
}

fn respan(stream: TokenStream, span: Span) -> TokenStream {
    stream
        .into_iter()
        .map(|token| respan_token(token, span))
        .collect()
}

fn respan_token(mut token: TokenTree, span: Span) -> TokenTree {
    if let TokenTree::Group(g) = &mut token {
        *g = Group::new(g.delimiter(), respan(g.stream(), span));
    }
    token.set_span(span);
    token
}
