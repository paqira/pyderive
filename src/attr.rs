use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Expr, ExprAssign, Ident, Lit, LitBool, LitStr, Meta, MetaList, Result, Token,
};

use self::{
    pyderive_field::{ExprAssignGeneric, OptionFieldAttr, PyderiveFieldAttr},
    pyo3_field::Pyo3FieldAttr,
    pyo3_struct::{Pyo3StructAttr, RenamingRule},
};

fn take_meta_list(a: &Attribute) -> Option<&MetaList> {
    match &a.meta {
        Meta::List(m) => Some(m),
        _ => None,
    }
}

#[derive(Debug, Default, Clone)]
pub struct Pyo3StructOption {
    pub get: bool,
    pub set: bool,
    pub rename: RenamingRule,
}

impl FromIterator<Pyo3StructAttr> for Pyo3StructOption {
    fn from_iter<T: IntoIterator<Item = Pyo3StructAttr>>(iter: T) -> Self {
        let mut new = Self::default();
        for opt in iter {
            match opt {
                Pyo3StructAttr::Get(_) => {
                    new.get = true;
                }
                Pyo3StructAttr::Set(_) => {
                    new.set = true;
                }
                Pyo3StructAttr::Rename { value, .. } => {
                    new.rename = value;
                }
                Pyo3StructAttr::Other => {}
            }
        }
        new
    }
}

// NEVER returns Error on parsing Pyo3 attr args,
// we just read them.
impl TryFrom<&Vec<Attribute>> for Pyo3StructOption {
    type Error = syn::Error;

    fn try_from(value: &Vec<Attribute>) -> Result<Self> {
        type Attr = Punctuated<Pyo3StructAttr, Token![,]>;

        value
            .iter()
            // FIXME:
            // Replace error handling,
            // here is parsing pyo3 attr arg
            .map(|a| {
                if a.path().is_ident("pyderive") {
                    Err(syn::Error::new(a.meta.span(), "support field only"))
                } else {
                    Ok(a)
                }
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .filter(|a| a.path().is_ident("pyclass") || a.path().is_ident("pyo3"))
            .filter_map(take_meta_list)
            .map(|m| m.parse_args_with(Attr::parse_terminated))
            .collect::<Result<Vec<_>>>()
            .map(|v| v.into_iter().flatten().collect::<Self>())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Pyo3FieldOption {
    pub get: bool,
    pub set: bool,
    pub name: Option<String>,
}

impl FromIterator<Pyo3FieldAttr> for Pyo3FieldOption {
    fn from_iter<T: IntoIterator<Item = Pyo3FieldAttr>>(iter: T) -> Self {
        let mut new = Self::default();

        for opt in iter {
            match opt {
                Pyo3FieldAttr::Get(_) => {
                    new.get = true;
                }
                Pyo3FieldAttr::Set(_) => {
                    new.set = true;
                }
                Pyo3FieldAttr::Name { value, .. } => {
                    new.name = Some(value.value());
                }
                Pyo3FieldAttr::Other => {}
            }
        }
        new
    }
}

// NEVER returns Error on parsing Pyo3 attr args,
// we just read them.
impl TryFrom<&Vec<Attribute>> for Pyo3FieldOption {
    type Error = syn::Error;

    fn try_from(value: &Vec<Attribute>) -> Result<Self> {
        type Attr = Punctuated<Pyo3FieldAttr, Token![,]>;

        value
            .iter()
            .filter(|a| a.path().is_ident("pyo3"))
            .filter_map(take_meta_list)
            .map(|m| m.parse_args_with(Attr::parse_terminated))
            .collect::<Result<Vec<_>>>()
            .map(|v| v.into_iter().flatten().collect::<Self>())
    }
}

#[derive(Debug, Default, Clone)]
pub struct PyderiveFieldOption {
    pub init: Option<bool>,
    pub match_args: Option<bool>,
    pub repr: Option<bool>,
    pub str: Option<bool>,
    pub iter: Option<bool>,
    pub len: Option<bool>,
    pub kw_only: Option<bool>,
    pub dataclass_field: Option<bool>,
    pub default: Option<Expr>,
    pub annotation: Option<String>,
}

impl FromIterator<PyderiveFieldAttr> for syn::Result<PyderiveFieldOption> {
    fn from_iter<T: IntoIterator<Item = PyderiveFieldAttr>>(iter: T) -> Self {
        let mut new = PyderiveFieldOption::default();

        macro_rules! extract_ident {
            ($value:ident) => {
                match $value {
                    OptionFieldAttr::Ident(ident) => ident,
                    OptionFieldAttr::ExprAssign(ExprAssignGeneric { left, .. }) => left,
                }
            };
        }

        macro_rules! is_true {
            ($value:ident) => {
                match $value {
                    OptionFieldAttr::Ident { .. } => true,
                    OptionFieldAttr::ExprAssign(ExprAssignGeneric {
                        right: LitBool { value, .. },
                        ..
                    }) => value,
                }
            };
        }

        for opt in iter {
            match opt {
                PyderiveFieldAttr::Init(v) => match new.init {
                    Some(_) => {
                        return Err(syn::Error::new(extract_ident!(v).span(), "duplicated init"));
                    }
                    None => {
                        new.init = Some(is_true!(v));
                    }
                },
                PyderiveFieldAttr::MatchArgs(v) => match new.match_args {
                    Some(_) => {
                        return Err(syn::Error::new(
                            extract_ident!(v).span(),
                            "duplicated match_args",
                        ));
                    }
                    None => {
                        new.match_args = Some(is_true!(v));
                    }
                },
                PyderiveFieldAttr::Repr(v) => match new.repr {
                    Some(_) => {
                        return Err(syn::Error::new(extract_ident!(v).span(), "duplicated repr"));
                    }
                    None => {
                        new.repr = Some(is_true!(v));
                    }
                },
                PyderiveFieldAttr::Str(v) => match new.str {
                    Some(_) => {
                        return Err(syn::Error::new(extract_ident!(v).span(), "duplicated str"));
                    }
                    None => {
                        new.str = Some(is_true!(v));
                    }
                },
                PyderiveFieldAttr::Iter(v) => match new.iter {
                    Some(_) => {
                        return Err(syn::Error::new(extract_ident!(v).span(), "duplicated iter"));
                    }
                    None => {
                        new.iter = Some(is_true!(v));
                    }
                },
                PyderiveFieldAttr::Len(v) => match new.len {
                    Some(_) => {
                        return Err(syn::Error::new(extract_ident!(v).span(), "duplicated len"));
                    }
                    None => {
                        new.len = Some(is_true!(v));
                    }
                },
                PyderiveFieldAttr::KwOnly(v) => match new.kw_only {
                    Some(_) => {
                        return Err(syn::Error::new(
                            extract_ident!(v).span(),
                            "duplicated kw_only",
                        ));
                    }
                    None => {
                        new.kw_only = Some(is_true!(v));
                    }
                },
                PyderiveFieldAttr::DataclassField(v) => match new.dataclass_field {
                    Some(_) => {
                        return Err(syn::Error::new(
                            extract_ident!(v).span(),
                            "duplicated dataclass_field",
                        ));
                    }
                    None => {
                        new.dataclass_field = Some(is_true!(v));
                    }
                },
                PyderiveFieldAttr::Default(v) => match new.default {
                    Some(_) => {
                        return Err(syn::Error::new(v.left.span(), "duplicated default"));
                    }
                    None => {
                        new.default = Some(*v.right);
                    }
                },
                PyderiveFieldAttr::Annotation(v) => match new.annotation {
                    Some(_) => {
                        return Err(syn::Error::new(v.left.span(), "duplicated annotation"));
                    }
                    None => {
                        new.annotation = Some(v.right.value());
                    }
                },
            }
        }

        Ok(new)
    }
}

impl TryFrom<&Vec<Attribute>> for PyderiveFieldOption {
    type Error = syn::Error;

    fn try_from(value: &Vec<Attribute>) -> Result<Self> {
        type Attr = Punctuated<PyderiveFieldAttr, Token![,]>;

        value
            .iter()
            .filter(|a| a.path().is_ident("pyderive"))
            // FIXME:
            // Shoud it raise Error when #[pyderive]?
            // If not, uncomment the following filter_map.
            // .filter_map(|a| match &a.meta {
            //     Meta::List(_) => Some(a),
            //     _ => None,
            // })
            // FIXME:
            // Shoud it raise Error when #[pyderive]?
            // If not, comment out the following filter_map.
            .map(|a| match &a.meta {
                Meta::List(m) => Ok(m),
                _ => Err(syn::Error::new(
                    a.meta.span(),
                    "supports #[pyderive(..)] form only",
                )),
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|m| {
                m.parse_args_with(Attr::parse_terminated)
                    // FIXME:
                    // Shoud it raise Error when #[pyderive()]?
                    // If not, remove the following and_then.
                    .and_then(|r| {
                        if r.is_empty() {
                            Err(syn::Error::new(
                                m.span(),
                                "effects nothing if argument is empty",
                            ))
                        } else {
                            Ok(r)
                        }
                    })
            })
            // Return #[pyderive()]? Err
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Result<Self>>()
    }
}

// pyo3 struct
pub mod pyo3_struct {
    use super::*;

    pub mod kw {
        // all of supporting option
        syn::custom_keyword!(get_all);
        syn::custom_keyword!(set_all);
        syn::custom_keyword!(rename_all);
    }

    #[derive(Debug, Clone)]
    pub enum RenamingRule {
        CamelCase,
        KebabCase,
        Lowercase,
        PascalCase,
        ScreamingKebabCase,
        ScreamingSnakeCase,
        SnakeCase,
        Uppercase,
        Other,
    }

    impl RenamingRule {
        pub fn rename(&self, name: &str) -> String {
            use heck::*;

            match self {
                Self::CamelCase => name.to_lower_camel_case(),
                Self::KebabCase => name.to_kebab_case(),
                Self::Lowercase => name.to_lowercase(),
                Self::PascalCase => name.to_upper_camel_case(),
                Self::ScreamingKebabCase => name.to_shouty_kebab_case(),
                Self::ScreamingSnakeCase => name.to_shouty_snake_case(),
                Self::SnakeCase => name.to_snake_case(),
                Self::Uppercase => name.to_uppercase(),
                Self::Other => name.to_string(),
            }
        }
    }

    impl Default for RenamingRule {
        fn default() -> Self {
            Self::Other
        }
    }

    impl Parse for RenamingRule {
        fn parse(input: ParseStream) -> Result<Self> {
            let rule: LitStr = input.parse()?;

            match rule.value().as_str() {
                "camelCase" => Ok(Self::CamelCase),
                "kebab-case" => Ok(Self::KebabCase),
                "lowercase" => Ok(Self::Lowercase),
                "PascalCase" => Ok(Self::PascalCase),
                "SCREAMING-KEBAB-CASE" => Ok(Self::ScreamingKebabCase),
                "SCREAMING_SNAKE_CASE" => Ok(Self::ScreamingSnakeCase),
                "snake_case" => Ok(Self::SnakeCase),
                "UPPERCASE" => Ok(Self::Uppercase),
                _ => Ok(Self::Other),
            }
        }
    }

    #[derive(Debug)]
    pub enum Pyo3StructAttr {
        Get(kw::get_all),
        Set(kw::set_all),
        Rename {
            path: kw::rename_all,
            eq_token: Token![=],
            value: RenamingRule,
        },
        Other,
    }

    impl Parse for Pyo3StructAttr {
        fn parse(input: ParseStream) -> Result<Self> {
            if input.peek(kw::get_all) {
                Ok(Self::Get(input.parse()?))
            } else if input.peek(kw::set_all) {
                Ok(Self::Set(input.parse()?))
            } else if input.peek(kw::rename_all) {
                Ok(Self::Rename {
                    path: input.parse()?,
                    eq_token: input.parse()?,
                    value: input.parse()?,
                })
            // ommit others
            } else if input.peek2(Token![=]) {
                // assigment
                let _: Ident = input.parse()?;
                let _: Token![=] = input.parse()?;
                let _: Lit = input.parse()?;
                Ok(Self::Other)
            } else {
                // key only
                let _ = input.parse::<Ident>()?;
                Ok(Self::Other)
            }
        }
    }
}

// pyo3 field
pub mod pyo3_field {
    use super::*;

    pub mod kw {
        // all of supporting option
        syn::custom_keyword!(get);
        syn::custom_keyword!(set);
        syn::custom_keyword!(name);
    }

    #[derive(Debug)]
    pub enum Pyo3FieldAttr {
        Get(kw::get),
        Set(kw::set),
        Name {
            path: kw::name,
            eq_token: Token![=],
            value: LitStr,
        },
        Other,
    }

    impl Parse for Pyo3FieldAttr {
        fn parse(input: ParseStream) -> Result<Self> {
            if input.peek(kw::get) {
                Ok(Self::Get(input.parse()?))
            } else if input.peek(kw::set) {
                Ok(Self::Set(input.parse()?))
            } else if input.peek(kw::name) {
                Ok(Self::Name {
                    path: input.parse()?,
                    eq_token: input.parse()?,
                    value: input.parse()?,
                })
            // ommit others
            } else if input.peek2(Token![=]) {
                // assigment
                let _: Ident = input.parse()?;
                let _: Token![=] = input.parse()?;
                let _: Lit = input.parse()?;
                Ok(Self::Other)
            } else {
                // key only
                let _ = input.parse::<Ident>()?;
                Ok(Self::Other)
            }
        }
    }
}

// pyderive field
pub mod pyderive_field {
    use super::*;

    mod kw {
        syn::custom_keyword!(init);
        syn::custom_keyword!(match_args);
        syn::custom_keyword!(repr);
        syn::custom_keyword!(str);
        syn::custom_keyword!(iter);
        syn::custom_keyword!(len);
        syn::custom_keyword!(kw_only);
        syn::custom_keyword!(dataclass_field);
        syn::custom_keyword!(default);
        syn::custom_keyword!(annotation);
    }

    #[derive(Debug)]
    pub struct ExprAssignGeneric<T, K> {
        pub left: T,
        pub eq_token: syn::token::Eq,
        pub right: K,
    }

    impl<T: Parse, K: Parse> Parse for ExprAssignGeneric<T, K> {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(Self {
                left: input.parse()?,
                eq_token: input.parse()?,
                right: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub enum OptionFieldAttr<T: Parse, K: Parse> {
        Ident(T),
        ExprAssign(ExprAssignGeneric<T, K>),
    }

    impl<T: Parse, K: Parse> Parse for OptionFieldAttr<T, K> {
        fn parse(input: ParseStream) -> Result<Self> {
            if input.peek2(Token![=]) {
                Ok(Self::ExprAssign(input.parse()?))
            } else {
                Ok(Self::Ident(input.parse()?))
            }
        }
    }

    #[derive(Debug)]
    pub enum PyderiveFieldAttr {
        Init(OptionFieldAttr<kw::init, LitBool>),
        MatchArgs(OptionFieldAttr<kw::match_args, LitBool>),
        Repr(OptionFieldAttr<kw::repr, LitBool>),
        Str(OptionFieldAttr<kw::str, LitBool>),
        Iter(OptionFieldAttr<kw::iter, LitBool>),
        Len(OptionFieldAttr<kw::len, LitBool>),
        KwOnly(OptionFieldAttr<kw::kw_only, LitBool>),
        DataclassField(OptionFieldAttr<kw::dataclass_field, LitBool>),
        Default(ExprAssign),
        Annotation(ExprAssignGeneric<kw::annotation, LitStr>),
    }

    impl Parse for PyderiveFieldAttr {
        fn parse(input: ParseStream) -> Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::init) {
                Ok(Self::Init(input.parse()?))
            } else if lookahead.peek(kw::match_args) {
                Ok(Self::MatchArgs(input.parse()?))
            } else if lookahead.peek(kw::repr) {
                Ok(Self::Repr(input.parse()?))
            } else if lookahead.peek(kw::str) {
                Ok(Self::Str(input.parse()?))
            } else if lookahead.peek(kw::iter) {
                Ok(Self::Iter(input.parse()?))
            } else if lookahead.peek(kw::len) {
                Ok(Self::Len(input.parse()?))
            } else if lookahead.peek(kw::kw_only) {
                Ok(Self::KwOnly(input.parse()?))
            } else if lookahead.peek(kw::dataclass_field) {
                Ok(Self::DataclassField(input.parse()?))
            } else if lookahead.peek(kw::default) {
                Ok(Self::Default(input.parse()?))
            } else if lookahead.peek(kw::annotation) {
                Ok(Self::Annotation(input.parse()?))
            } else {
                Err(lookahead.error())
            }
        }
    }
}
