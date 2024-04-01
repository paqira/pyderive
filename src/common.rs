use quote::format_ident;
use syn::{
    punctuated::Punctuated, spanned::Spanned, Data, DataEnum, DataStruct, DataUnion, DeriveInput,
    Expr, Field, Fields, FieldsNamed, Ident, Path, Result, Token, Type, TypePath,
};

use crate::attr::{
    pyo3_struct::RenamingRule, PyderiveFieldOption, Pyo3FieldOption, Pyo3StructOption,
};

pub(crate) fn is_py(ty: &Type) -> bool {
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
pub(crate) struct FieldData {
    #[allow(dead_code)]
    pub(crate) index: usize,
    pub(crate) field: Field,
    pub(crate) get: bool,
    pub(crate) set: bool,
    // String -> Some(String) to support Tuple struct
    pub(crate) pyname: String,
    // String -> Some(Ident) to support Tuple struct
    pub(crate) pyident: Ident,
    new: Option<bool>,
    match_args: Option<bool>,
    repr: Option<bool>,
    str: Option<bool>,
    iter: Option<bool>,
    len: Option<bool>,
    kw_only: Option<bool>,
    dataclass_field: Option<bool>,
    pub(crate) default: Option<Expr>,
    default_factory: Option<bool>,
    pub(crate) annotation: Option<String>,
}

impl FieldData {
    #[allow(clippy::wrong_self_convention)]
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new(&self) -> bool {
        self.new.unwrap_or(true)
    }
    pub(crate) fn match_args(&self) -> bool {
        self.match_args.unwrap_or(self.get)
    }
    pub(crate) fn repr(&self) -> bool {
        self.repr.unwrap_or(self.get || self.set)
    }
    pub(crate) fn str(&self) -> bool {
        self.str.unwrap_or(self.get || self.set)
    }
    pub(crate) fn iter(&self) -> bool {
        self.iter.unwrap_or(self.get)
    }
    pub(crate) fn len(&self) -> bool {
        self.len.unwrap_or(self.get)
    }
    pub(crate) fn kw_only(&self) -> bool {
        self.kw_only.unwrap_or(false)
    }
    pub(crate) fn dataclass_field(&self) -> bool {
        self.dataclass_field.unwrap_or(true)
    }
    pub(crate) fn default_factory(&self) -> bool {
        self.default_factory.unwrap_or(false)
    }

    pub(crate) fn try_from_input(input: &DeriveInput) -> Result<Vec<Self>> {
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
                    //
                    new: pyderive_field_opt.new,
                    match_args: pyderive_field_opt.match_args,
                    repr: pyderive_field_opt.repr,
                    str: pyderive_field_opt.str,
                    iter: pyderive_field_opt.iter,
                    len: pyderive_field_opt.len,
                    kw_only: pyderive_field_opt.kw_only,
                    dataclass_field: pyderive_field_opt.dataclass_field,
                    default: pyderive_field_opt.default,
                    default_factory: pyderive_field_opt.default_factory,
                    annotation: pyderive_field_opt.annotation,
                })
            })
            .collect::<Result<Vec<_>>>()
    }
}
