use super::attrs::Attrs;
use crate::ctxt::Ctxt;
use proc_macro2::TokenStream;
use quote::quote;
use quote::{ToTokens, TokenStreamExt};
use syn::{punctuated::Punctuated, token::Comma, Attribute, Ident, Variant};

pub enum JsonErrorKind {
    NaiveRequest,
    Request,
    NaiveInternal,
    Internal,
}

pub struct JsonError {
    ident: Ident,
    kind: JsonErrorKind,
    attrs: Attrs,
}

impl JsonError {
    pub(crate) fn new_internal_error(
        variant: &Variant,
        _attr: &Attribute,
        ctxt: &Ctxt,
    ) -> Option<Self> {
        match variant.fields.len() {
            0 => Some(JsonError {
                ident: variant.ident.clone(),
                kind: JsonErrorKind::NaiveInternal,
                attrs: Attrs::new(),
            }),
            1 => Some(JsonError {
                ident: variant.ident.clone(),
                kind: JsonErrorKind::Internal,
                attrs: Attrs::new(),
            }),
            _ => {
                ctxt.error_spanned_by(
                    variant,
                    "Tuple variants with more than one fields are not supported",
                );
                None
            }
        }
    }

    pub(crate) fn new_request_error(
        variant: &Variant,
        attr: &Attribute,
        ctxt: &Ctxt,
    ) -> Option<Self> {
        let mut attrs = match Attrs::from_attr(attr, ctxt) {
            Some(attrs) => attrs,
            None => return None,
        };
        attrs.set_optional("message");
        let allowed_fields = ["status", "code", "message"];
        for attr in attrs.mut_inner().iter_mut() {
            if !allowed_fields.contains(&attr.ident.as_str()) {
                ctxt.error_spanned_by(
                    &attr.left,
                    format!(
                        "Unknown attribute {}",
                        attr.left.get_ident().unwrap().to_string()
                    ),
                )
            }
            if attr.ident == "message" {
                attr.optional = true;
            }
        }

        match variant.fields.len() {
            0 => Some(JsonError {
                ident: variant.ident.clone(),
                kind: JsonErrorKind::NaiveRequest,
                attrs,
            }),
            1 => Some(JsonError {
                ident: variant.ident.clone(),
                kind: JsonErrorKind::Request,
                attrs,
            }),
            _ => {
                ctxt.error_spanned_by(variant, "Only one error can be wrapped in each variant");
                None
            }
        }
    }

    pub(crate) fn from_variant(variant: &Variant, ctxt: &Ctxt) -> Option<Self> {
        for attr in &variant.attrs {
            let ident = &attr.path.get_ident();
            if let Some(ident) = ident {
                let ident_string = ident.to_string();
                if ident_string == "request_error" {
                    if let Some(json_error) = Self::new_request_error(variant, attr, ctxt) {
                        return Some(json_error);
                    }
                } else if ident_string == "internal_error" {
                    if let Some(json_error) = Self::new_internal_error(variant, attr, ctxt) {
                        return Some(json_error);
                    }
                }
            }
        }
        ctxt.error_spanned_by(
            variant.ident.clone(),
            "All enum variants should have either a request_error or an internal_error attribute",
        );
        None
    }

    pub(crate) fn expand_match_condition(&self, type_ident: &Ident) -> TokenStream {
        let kind = &self.ident;
        match self.kind {
            JsonErrorKind::NaiveRequest => {
                let (fields, values) = self.attrs.expand_unzip();
                quote! {
                    #type_ident::#kind => yeax_api::JsonError{
                        #(#fields: #values ,)*
                        content: (),
                        ..yeax_api::JsonError::default()
                    }.error_response()
                }
            }
            JsonErrorKind::Request => {
                let (fields, values) = self.attrs.expand_unzip();
                quote! {
                    #type_ident::#kind(err) => yeax_api::JsonError{
                        #(#fields: #values),*,
                        content: err.clone(),
                        ..yeax_api::JsonError::default()
                    }.error_response()
                }
            }
            JsonErrorKind::NaiveInternal => quote! {
                #type_ident::#kind => {
                    yeax_api::JsonError{
                        status: actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                        code: "50000 internal-error".into(),
                        content: (),
                        ..yeax_api::JsonError::default()
                    }.error_response()
                }
            },
            JsonErrorKind::Internal => quote! {
                #type_ident::#kind(err) => {
                    yeax_api::JsonError{
                        status: actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                        code: "50000 internal-error".into(),
                        content: (),
                        ..yeax_api::JsonError::default()
                    }.error_response()
                }
            },
        }
    }
}
pub struct JsonErrors {
    ident: Ident,
    errors: Vec<JsonError>,
}

impl JsonErrors {
    pub(crate) fn from_variants(
        ident: Ident,
        variants: &Punctuated<Variant, Comma>,
        ctxt: &Ctxt,
    ) -> Option<Self> {
        let mut ret = Vec::new();
        for variant in variants.iter() {
            match JsonError::from_variant(variant, ctxt) {
                Some(err) => ret.push(err),
                None => return None,
            }
        }
        Some(Self { ident, errors: ret })
    }
}

impl ToTokens for JsonErrors {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for err_type in &self.errors {
            let cond = err_type.expand_match_condition(&self.ident);
            let gen = quote!(#cond,);
            tokens.append_all(gen);
        }
    }
}
