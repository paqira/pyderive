use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, ExprAssign, Ident, Lit, LitBool, LitStr, Meta, MetaList, Result, Token,
};

use self::{
    pyderive_field::PyderiveFieldAttr,
    pyo3_field::Pyo3FieldAttr,
    pyo3_struct::{Pyo3StructAttr, RenamingRule},
};

fn is_pyo3_struct_attr<'a>(a: &'a &Attribute) -> bool {
    a.path().is_ident("pyclass") || a.path().is_ident("pyo3")
}

fn is_pyo3_field_attr<'a>(a: &'a &Attribute) -> bool {
    a.path().is_ident("pyo3")
}

fn is_pyderive_field_attr<'a>(a: &'a &Attribute) -> bool {
    a.path().is_ident("pyderive")
}

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

    fn try_from(value: &Vec<Attribute>) -> syn::Result<Self> {
        type AttrList = Punctuated<Pyo3StructAttr, Token![,]>;

        value
            .iter()
            .filter(is_pyo3_struct_attr)
            .filter_map(take_meta_list)
            .map(|m| m.parse_args_with(AttrList::parse_terminated))
            .collect::<syn::Result<Vec<_>>>()
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

    fn try_from(value: &Vec<Attribute>) -> syn::Result<Self> {
        type AttrList = Punctuated<Pyo3FieldAttr, Token![,]>;

        value
            .iter()
            .filter(is_pyo3_field_attr)
            .filter_map(take_meta_list)
            .map(|m| m.parse_args_with(AttrList::parse_terminated))
            .collect::<syn::Result<Vec<_>>>()
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

impl FromIterator<PyderiveFieldAttr> for PyderiveFieldOption {
    fn from_iter<T: IntoIterator<Item = PyderiveFieldAttr>>(iter: T) -> Self {
        let mut new = Self::default();

        for opt in iter {
            match opt {
                PyderiveFieldAttr::Init(v) => {
                    new.init = Some(v);
                }
                PyderiveFieldAttr::MatchArgs(v) => {
                    new.match_args = Some(v);
                }
                PyderiveFieldAttr::Repr(v) => {
                    new.repr = Some(v);
                }
                PyderiveFieldAttr::Str(v) => {
                    new.str = Some(v);
                }
                PyderiveFieldAttr::Iter(v) => {
                    new.iter = Some(v);
                }
                PyderiveFieldAttr::Len(v) => {
                    new.len = Some(v);
                }
                PyderiveFieldAttr::KwOnly(v) => {
                    new.kw_only = Some(v);
                }
                PyderiveFieldAttr::Default(v) => {
                    new.default = Some(v);
                }
            }
        }
        new
    }
}

impl TryFrom<&Vec<Attribute>> for PyderiveFieldOption {
    type Error = syn::Error;

    fn try_from(value: &Vec<Attribute>) -> syn::Result<Self> {
        type AttrList = Punctuated<PyderiveFieldAttr, Token![,]>;

        value
            .iter()
            .filter(is_pyderive_field_attr)
            .filter_map(take_meta_list)
            .map(|m| m.parse_args_with(AttrList::parse_terminated))
            .collect::<syn::Result<Vec<_>>>()
            .map(|v| v.into_iter().flatten().collect::<Self>())
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
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::get_all) {
                Ok(Self::Get(input.parse()?))
            } else if lookahead.peek(kw::set_all) {
                Ok(Self::Set(input.parse()?))
            } else if lookahead.peek(kw::name) {
                Ok(Self::Name {
                    path: input.parse()?,
                    eq_token: input.parse()?,
                    value: input.parse()?,
                })
            } else if lookahead.peek(kw::rename_all) {
                Ok(Self::Rename {
                    path: input.parse()?,
                    eq_token: input.parse()?,
                    value: input.parse()?,
                })
            // drop others
            } else if input.peek2(Token![=]) {
                let _ = input.parse::<ExprAssign>()?;
                Ok(Self::Other)
            } else {
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
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::get) {
                Ok(Self::Get(input.parse()?))
            } else if lookahead.peek(kw::set) {
                Ok(Self::Set(input.parse()?))
            } else if lookahead.peek(kw::name) {
                Ok(Self::Name {
                    path: input.parse()?,
                    eq_token: input.parse()?,
                    value: input.parse()?,
                })
            // ommit others
            } else if input.peek2(Token![=]) {
                // assignment
                let _ = input.parse::<ExprAssign>()?;
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
    pub struct ExprAssignLitBool {
        pub left: Ident,
        pub eq_token: Token![=],
        pub right: LitBool,
    }

    impl Parse for ExprAssignLitBool {
        fn parse(input: ParseStream) -> Result<Self> {
            let left = input.parse()?;
            let eq_token: Token![=] = input.parse()?;
            let right: LitBool = input.parse()?;
            Ok(Self {
                left,
                eq_token,
                right,
            })
        }
    }

    #[derive(Debug)]
    pub struct ExprAssignLit {
        pub left: Ident,
        pub eq_token: Token![=],
        pub right: Lit,
    }

    impl Parse for ExprAssignLit {
        fn parse(input: ParseStream) -> Result<Self> {
            let left = input.parse()?;
            let eq_token: Token![=] = input.parse()?;
            let right: Lit = input.parse()?;
            Ok(Self {
                left,
                eq_token,
                right,
            })
        }
    }

    #[derive(Debug, Clone)]
    pub enum PyderiveFieldAttr {
        Init(bool),
        MatchArgs(bool),
        Repr(bool),
        Str(bool),
        Iter(bool),
        Len(bool),
        KwOnly(bool),
        Default(Lit),
    }

    impl Parse for PyderiveFieldAttr {
        fn parse(input: ParseStream) -> Result<Self> {
            macro_rules! parse_assign_bool {
            ($input:ident, $ident_ty:ty) => {
                if $input.peek2(Token![=]) {
                    let parsed: ExprAssignLitBool = $input.parse()?;
                    parsed.right.value
                } else {
                    let _: $ident_ty = $input.parse()?;
                    true
                }
            };
        }

            let lookahead = input.lookahead1();
            if lookahead.peek(kw::init) {
                let value = parse_assign_bool!(input, kw::init);
                Ok(Self::Init(value))
            } else if lookahead.peek(kw::match_args) {
                let value = parse_assign_bool!(input, kw::match_args);
                Ok(Self::MatchArgs(value))
            } else if lookahead.peek(kw::repr) {
                let value = parse_assign_bool!(input, kw::repr);
                Ok(Self::Repr(value))
            } else if lookahead.peek(kw::str) {
                let value = parse_assign_bool!(input, kw::str);
                Ok(Self::Str(value))
            } else if lookahead.peek(kw::iter) {
                let value = parse_assign_bool!(input, kw::iter);
                Ok(Self::Iter(value))
            } else if lookahead.peek(kw::len) {
                let value = parse_assign_bool!(input, kw::len);
                Ok(Self::Len(value))
            } else if lookahead.peek(kw::kw_only) {
                let value = parse_assign_bool!(input, kw::kw_only);
                Ok(Self::KwOnly(value))
            } else if lookahead.peek(kw::default) {
                let parsed: ExprAssignLit = input.parse()?;
                Ok(Self::Default(parsed.right))
            } else {
                Err(lookahead.error())
            }
        }
    }
}
