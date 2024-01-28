use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Ident, Lit, LitBool, LitStr, Meta, MetaList, Result, Token,
};

use self::{
    pyderive_field::{ExprAssignLit, OptionFieldAttr, PyderiveFieldAttr},
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
    pub name: Option<String>,
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
                Pyo3StructAttr::Name { value: val, .. } => {
                    new.name = Some(val.value().to_string());
                }
                Pyo3StructAttr::Rename { value: val, .. } => {
                    new.rename = val;
                }
                Pyo3StructAttr::Other => {}
            }
        }
        new
    }
}

impl TryFrom<&Vec<Attribute>> for Pyo3StructOption {
    type Error = syn::Error;

    fn try_from(value: &Vec<Attribute>) -> Result<Self> {
        type Attr = Punctuated<Pyo3StructAttr, Token![,]>;

        value
            .iter()
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
                Pyo3FieldAttr::Name { value: val, .. } => {
                    new.name = Some(val.value().to_string());
                }
                Pyo3FieldAttr::Other => {}
            }
        }
        new
    }
}

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
    pub default: Option<Lit>,
}

impl FromIterator<PyderiveFieldAttr> for syn::Result<PyderiveFieldOption> {
    fn from_iter<T: IntoIterator<Item = PyderiveFieldAttr>>(iter: T) -> Self {
        let mut new = PyderiveFieldOption::default();

        macro_rules! extract_ident {
            ($value:ident) => {
                match $value {
                    OptionFieldAttr::Ident { ident } => ident,
                    OptionFieldAttr::ExprAssign(ExprAssignLit { ident, .. }) => ident,
                }
            };
        }

        macro_rules! is_true {
            ($value:ident) => {
                match $value {
                    OptionFieldAttr::Ident { .. } => true,
                    OptionFieldAttr::ExprAssign(ExprAssignLit {
                        value: LitBool { value, .. },
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
                PyderiveFieldAttr::Default(v) => match new.default {
                    Some(_) => {
                        return Err(syn::Error::new(v.ident.span(), "duplicated default"));
                    }
                    None => {
                        new.default = Some(v.value);
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
            .filter_map(take_meta_list)
            .map(|m| m.parse_args_with(Attr::parse_terminated))
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
        syn::custom_keyword!(name);
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
        Name {
            path: kw::name,
            eq_token: Token![=],
            value: LitStr,
        },
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
            } else if input.peek(kw::name) {
                Ok(Self::Name {
                    path: input.parse()?,
                    eq_token: input.parse()?,
                    value: input.parse()?,
                })
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
        syn::custom_keyword!(default);
    }

    #[derive(Debug)]
    pub struct ExprAssignLit<T, K> {
        pub ident: T,
        pub value: K,
    }

    impl<T: Parse, K: Parse> Parse for ExprAssignLit<T, K> {
        fn parse(input: ParseStream) -> Result<Self> {
            let ident: T = input.parse()?;
            let _: Token![=] = input.parse()?;
            let value: K = input.parse()?;
            Ok(Self { ident, value })
        }
    }

    #[derive(Debug)]
    pub enum OptionFieldAttr<T: Parse, K: Parse> {
        Ident { ident: T },
        ExprAssign(ExprAssignLit<T, K>),
    }

    impl<T: Parse, K: Parse> Parse for OptionFieldAttr<T, K> {
        fn parse(input: ParseStream) -> Result<Self> {
            if input.peek2(Token![=]) {
                let ident: T = input.parse()?;
                let _: Token![=] = input.parse()?;
                let value: K = input.parse()?;
                Ok(Self::ExprAssign(ExprAssignLit { ident, value }))
            } else {
                Ok(Self::Ident {
                    ident: input.parse()?,
                })
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
        Default(ExprAssignLit<kw::default, Lit>),
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
            } else if lookahead.peek(kw::default) {
                Ok(Self::Default(input.parse()?))
            } else {
                Err(lookahead.error())
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct PyderiveFieldOption {
        pub init: Option<OptionFieldAttr<kw::init, LitBool>>,
        pub match_args: Option<OptionFieldAttr<kw::match_args, LitBool>>,
        pub repr: Option<OptionFieldAttr<kw::repr, LitBool>>,
        pub str: Option<OptionFieldAttr<kw::str, LitBool>>,
        pub iter: Option<OptionFieldAttr<kw::iter, LitBool>>,
        pub len: Option<OptionFieldAttr<kw::len, LitBool>>,
        pub kw_only: Option<OptionFieldAttr<kw::kw_only, LitBool>>,
        pub default: Option<OptionFieldAttr<kw::default, Lit>>,
    }
}
