use quote::format_ident;
use syn::{spanned::Spanned, Field, Ident, Lit, Path, Type, TypePath};

use crate::attr::{
    pyo3_struct::RenamingRule, PyderiveFieldOption, Pyo3FieldOption, Pyo3StructOption,
};

macro_rules! fields_named {
    ($n:ident) => {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named: $n, .. }),
            ..
        })
    };
}
pub(crate) use fields_named;

#[allow(unused_macros)]
macro_rules! fields_unnamed {
    ($n:ident) => {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed: $n, .. }),
            ..
        })
    };
}
#[allow(unused_imports)]
pub(crate) use fields_unnamed;

pub fn is_py(ty: &Type) -> bool {
    match &ty {
        Type::Path(TypePath {
            path: Path {
                segments: ref seg, ..
            },
            ..
        }) => seg.last(),
        _ => None,
    }
    .map_or(false, |seg| seg.ident.eq("py"))
}

pub fn is_string(ty: &Type) -> bool {
    match &ty {
        Type::Path(TypePath {
            path: Path {
                segments: ref seg, ..
            },
            ..
        }) => seg.last(),
        _ => None,
    }
    .map_or(false, |seg| seg.ident.eq("String"))
}

#[derive(Debug)]
pub struct FieldData {
    pub field: Field,
    pub get: bool,
    pub set: bool,
    pub pyname: String,
    pub pyident: Ident,
    pub init: Option<bool>,
    pub match_args: Option<bool>,
    pub repr: Option<bool>,
    pub str: Option<bool>,
    pub iter: Option<bool>,
    pub len: Option<bool>,
    pub kw_only: Option<bool>,
    pub default: Option<Lit>,
}

impl FieldData {
    pub fn try_from_input(input: &syn::DeriveInput) -> syn::Result<Vec<Self>> {
        let pyo3_struct_op = Pyo3StructOption::try_from(&input.attrs)?;

        let fields = match &input.data {
            fields_named!(n) => n,
            // fields_unnamed!(n) => n,
            _ => {
                return Err(syn::Error::new(
                    input.span(),
                    "support struct with field only",
                ))
            }
        };

        fields
            .iter()
            .map(|field| {
                let pyo3_field_opt = Pyo3FieldOption::try_from(&field.attrs)?;
                let pyderive_field_opt = PyderiveFieldOption::try_from(&field.attrs)?;

                let get = pyo3_struct_op.get || pyo3_field_opt.get;
                let set = pyo3_struct_op.set || pyo3_field_opt.set;
                let pyname = match pyo3_field_opt.name {
                    Some(name) => name,
                    None => match pyo3_struct_op.rename {
                        RenamingRule::Other => field.ident.as_ref().unwrap().to_string(),
                        _ => pyo3_struct_op
                            .rename
                            .rename(&field.ident.as_ref().unwrap().to_string()),
                    },
                };

                Ok(FieldData {
                    field: field.to_owned(),
                    get,
                    set,
                    pyname: pyname.clone(),
                    pyident: format_ident!("{}", pyname),
                    init: pyderive_field_opt.init,
                    match_args: pyderive_field_opt.match_args,
                    repr: pyderive_field_opt.repr,
                    str: pyderive_field_opt.str,
                    iter: pyderive_field_opt.iter,
                    len: pyderive_field_opt.len,
                    kw_only: pyderive_field_opt.kw_only,
                    default: pyderive_field_opt.default,
                })
            })
            .collect::<syn::Result<Vec<_>>>()
    }
}
