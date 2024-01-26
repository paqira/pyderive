extern crate proc_macro;
use syn::{Attribute, Ident, Path, Type, TypePath};

use crate::attr::pyclass::{FieldPyO3Options, PyClassPyO3Options};

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

pub struct ClassAttrOption {
    pub options: Option<PyClassPyO3Options>,
}

impl ClassAttrOption {
    pub fn try_from_attrs(attrs: &[Attribute]) -> syn::Result<Self> {
        let options = attrs
            .iter()
            .map(|attr| match &attr.meta {
                syn::Meta::List(l) => Some(l),
                _ => None,
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .filter(|l| l.path.segments.iter().any(|seg| seg.ident.eq(&"pyclass")))
            .map(|l| syn::parse2::<PyClassPyO3Options>(l.tokens.to_owned()))
            .next()
            .map_or(Ok(None), |v| v.map(Some))?;

        Ok(Self { options })
    }
}

pub struct FieldAttrOption {
    field: Option<FieldPyO3Options>,
}

impl FieldAttrOption {
    pub fn parse_field_attr(attrs: &[Attribute]) -> syn::Result<Self> {
        let options = attrs
            .iter()
            .map(|attr| match &attr.meta {
                syn::Meta::List(l) => Some(l),
                _ => None,
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .filter(|l| l.path.segments.iter().any(|seg| seg.ident.eq(&"pyo3")))
            .map(|l| syn::parse2::<FieldPyO3Options>(l.tokens.to_owned()))
            .next()
            .map_or(Ok(None), |v| v.map(Some))?;

        Ok(Self { field: options })
    }

    // Returns `true` if one of get_all, set_all, get, set exists.
    pub fn is_visible(&self, opt: &ClassAttrOption) -> bool {
        match &opt.options {
            Some(PyClassPyO3Options {
                get_all: Some(_), ..
            })
            | Some(PyClassPyO3Options {
                set_all: Some(_), ..
            }) => true,
            _ => match &self.field {
                Some(_) => true,
                None => false,
            },
        }
    }

    pub fn is_gettable(&self, opt: &ClassAttrOption) -> bool {
        match &opt.options {
            Some(PyClassPyO3Options {
                get_all: Some(_), ..
            }) => true,
            _ => match &self.field {
                Some(FieldPyO3Options { get: Some(_), .. }) => true,
                _ => false,
            },
        }
    }

    #[allow(dead_code)]
    fn is_settable(&self, opt: &ClassAttrOption) -> bool {
        match &opt.options {
            Some(PyClassPyO3Options {
                set_all: Some(_), ..
            }) => true,
            _ => match &self.field {
                Some(FieldPyO3Options { set: Some(_), .. }) => true,
                _ => false,
            },
        }
    }

    // Returns python name
    pub fn py_name(&self, ident: &Ident, opt: &ClassAttrOption) -> String {
        match &self.field {
            // priotize #[pyo3(name="...")]
            Some(FieldPyO3Options {
                name: Some(name), ..
            }) => name.value.0.to_string(),
            _ => match &opt.options {
                // renaming
                Some(PyClassPyO3Options {
                    rename_all: Some(o),
                    ..
                }) => o.value.rule.apply_renaming_rule(&ident.to_string()),
                // otherwise
                _ => ident.to_string(),
            },
        }
        .into()
    }
}
