use quote::format_ident;
use syn::{spanned::Spanned, Field, Ident, Path, Type, TypePath};

use crate::attr::{st::RenamingRule, FieldOption, StructOption};

macro_rules! fields_named {
    ($n:ident) => {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named: $n, .. }),
            ..
        })
    };
}
pub(crate) use fields_named;

macro_rules! fields_unnamed {
    ($n:ident) => {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed: $n, .. }),
            ..
        })
    };
}
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
pub struct StructData {
    pub ident: Ident,
    option: StructOption,
}

impl StructData {
    pub fn new(ident: Ident, option: StructOption) -> Self {
        Self { ident, option }
    }
    pub fn name(&self) -> String {
        self.ident.to_string()
    }
    pub fn pyname(&self) -> String {
        match &self.option.name {
            Some(name) => name.to_owned(),
            None => self.name(),
        }
    }
}

#[derive(Debug)]
pub struct FieldData {
    field: Field,
    struct_option: StructOption,
    field_option: FieldOption,
}

impl FieldData {
    pub fn try_from_data(
        input: syn::DeriveInput,
        struct_option: &StructOption,
    ) -> syn::Result<Vec<Self>> {
        match &input.data {
            fields_named!(n) => n,
            // fields_unnamed!(n) => n,
            _ => return Err(syn::Error::new(input.span(), "support struct with field only")),
        }
        .iter()
        .map(|field| {
            Ok(FieldData::new(
                field.clone(),
                struct_option.clone(),
                FieldOption::try_from(&field.attrs)?,
            ))
        })
        .collect::<syn::Result<Vec<_>>>()
    }

    pub fn new(field: Field, struct_option: StructOption, field_option: FieldOption) -> Self {
        Self {
            field,
            struct_option,
            field_option,
        }
    }
    pub fn ident(&self) -> Ident {
        self.field.ident.to_owned().unwrap()
    }
    pub fn pyident(&self) -> Ident {
        format_ident!("{}", self.pyname())
    }
    pub fn ty(&self) -> Type {
        self.field.ty.to_owned()
    }
    pub fn name(&self) -> String {
        self.field.ident.as_ref().unwrap().to_string()
    }
    pub fn pyname(&self) -> String {
        match &self.field_option.name {
            Some(name) => name.to_owned(),
            None => match &self.struct_option.rename {
                RenamingRule::Other => self.name(),
                rule => rule.rename(&self.name()),
            },
        }
    }
    pub fn get(&self) -> bool {
        self.field_option.get || self.struct_option.get
    }
    pub fn set(&self) -> bool {
        self.field_option.set || self.struct_option.set
    }
}
