use quote::format_ident;
use syn::{
    punctuated::Punctuated, spanned::Spanned, Data, DataEnum, DataStruct, DataUnion, DeriveInput,
    Expr, Field, Fields, FieldsNamed, Ident, Path, Result, Token, Type, TypePath,
};

use crate::attr::{
    pyo3_struct::RenamingRule, PyderiveFieldOption, Pyo3FieldOption, Pyo3StructOption,
};

pub fn is_py(ty: &Type) -> bool {
    match &ty {
        Type::Path(TypePath {
            path: Path { ref segments, .. },
            ..
        }) => {
            let mut iter = segments.iter();
            match iter.next() {
                Some(first) if first.ident.eq("pyo3") => match iter.next() {
                    Some(second) if second.ident.eq("Py") => iter.next().is_none(),
                    _ => false,
                },
                Some(first) if first.ident.eq("Py") => iter.next().is_none(),
                _ => false,
            }
        }
        _ => false,
    }
}

#[derive(Debug, Clone)]
pub struct FieldData {
    pub index: usize,
    pub field: Field,
    pub get: bool,
    pub set: bool,
    // String -> Some(String) to support Tuple struct
    pub pyname: String,
    // String -> Some(Ident) to support Tuple struct
    pub pyident: Ident,
    init: Option<bool>,
    match_args: Option<bool>,
    repr: Option<bool>,
    str: Option<bool>,
    iter: Option<bool>,
    len: Option<bool>,
    kw_only: Option<bool>,
    dataclass_field: Option<bool>,
    pub default: Option<Expr>,
    pub annotation: Option<String>,
}

impl FieldData {
    pub fn init(&self) -> bool {
        self.init.unwrap_or(true)
    }
    pub fn match_args(&self) -> bool {
        self.match_args.unwrap_or(self.get)
    }
    pub fn repr(&self) -> bool {
        self.repr.unwrap_or(self.get || self.set)
    }
    pub fn str(&self) -> bool {
        self.str.unwrap_or(self.get || self.set)
    }
    pub fn iter(&self) -> bool {
        self.iter.unwrap_or(self.get)
    }
    pub fn len(&self) -> bool {
        self.len.unwrap_or(self.get)
    }
    pub fn kw_only(&self) -> bool {
        self.kw_only.unwrap_or(false)
    }
    pub fn dataclass_field(&self) -> bool {
        self.dataclass_field.unwrap_or(true)
    }

    pub fn try_from_input(input: &DeriveInput) -> Result<Vec<Self>> {
        let pyo3_struct_op = Pyo3StructOption::try_from(&input.attrs)?;

        let empty = Punctuated::<Field, Token![,]>::new();
        let fields = match &input.data {
            Data::Struct(DataStruct { fields, .. }) => match fields {
                Fields::Named(FieldsNamed { named, .. }) => named,
                Fields::Unit => &empty,
                unnamed => {
                    return Err(syn::Error::new(
                        unnamed.span(),
                        "support struct with field, not tuple struct",
                    ))
                }
            },
            Data::Enum(DataEnum { enum_token, .. }) => {
                return Err(syn::Error::new(
                    enum_token.span(),
                    "support struct with field, not enum",
                ))
            }
            Data::Union(DataUnion { union_token, .. }) => {
                return Err(syn::Error::new(
                    union_token.span(),
                    "support struct with field, not union",
                ))
            }
        };

        fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
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
                    index,
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
                    dataclass_field: pyderive_field_opt.dataclass_field,
                    default: pyderive_field_opt.default,
                    annotation: pyderive_field_opt.annotation,
                })
            })
            .collect::<syn::Result<Vec<_>>>()
    }
}
