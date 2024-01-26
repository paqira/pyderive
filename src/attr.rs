// from pyo3-macros-backend
//
// - https://github.com/PyO3/pyo3/blob/main/pyo3-macros-backend/src/attributes.rs
// - https://github.com/PyO3/pyo3/blob/main/pyo3-macros-backend/src/utils.rs
//
// modified to support TokenStream in simple way

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Expr, Ident, LitStr, Path, Result, Token,
};

macro_rules! err_spanned {
    ($span:expr => $msg:expr) => {
        syn::Error::new($span, $msg)
    };
}

macro_rules! bail_spanned {
    ($span:expr => $msg:expr) => {
        return Err(err_spanned!($span => $msg))
    };
}
macro_rules! ensure_spanned {
    ($condition:expr, $span:expr => $msg:expr) => {
        if !($condition) {
            bail_spanned!($span => $msg);
        }
    }
}

pub mod attributes {
    use super::*;

    pub mod kw {
        syn::custom_keyword!(annotation);
        syn::custom_keyword!(attribute);
        syn::custom_keyword!(cancel_handle);
        syn::custom_keyword!(dict);
        syn::custom_keyword!(extends);
        syn::custom_keyword!(freelist);
        syn::custom_keyword!(from_py_with);
        syn::custom_keyword!(frozen);
        syn::custom_keyword!(get);
        syn::custom_keyword!(get_all);
        syn::custom_keyword!(item);
        syn::custom_keyword!(from_item_all);
        syn::custom_keyword!(mapping);
        syn::custom_keyword!(module);
        syn::custom_keyword!(name);
        syn::custom_keyword!(pass_module);
        syn::custom_keyword!(rename_all);
        syn::custom_keyword!(sequence);
        syn::custom_keyword!(set);
        syn::custom_keyword!(set_all);
        syn::custom_keyword!(signature);
        syn::custom_keyword!(subclass);
        syn::custom_keyword!(text_signature);
        syn::custom_keyword!(transparent);
        syn::custom_keyword!(unsendable);
        syn::custom_keyword!(weakref);
    }

    #[derive(Clone, Debug)]
    pub struct KeywordAttribute<K, V> {
        pub kw: K,
        pub value: V,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct LitStrValue<T>(pub T);

    impl<T: Parse> Parse for LitStrValue<T> {
        fn parse(input: ParseStream<'_>) -> Result<Self> {
            let lit_str: LitStr = input.parse()?;
            lit_str.parse().map(LitStrValue)
        }
    }

    impl<T: ToTokens> ToTokens for LitStrValue<T> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.0.to_tokens(tokens)
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NameLitStr(pub Ident);

    impl Parse for NameLitStr {
        fn parse(input: ParseStream<'_>) -> Result<Self> {
            let string_literal: LitStr = input.parse()?;
            if let Ok(ident) = string_literal.parse() {
                Ok(NameLitStr(ident))
            } else {
                bail_spanned!(string_literal.span() => "expected a single identifier in double quotes")
            }
        }
    }

    impl ToTokens for NameLitStr {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.0.to_tokens(tokens)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RenamingRule {
        CamelCase,
        KebabCase,
        Lowercase,
        PascalCase,
        ScreamingKebabCase,
        ScreamingSnakeCase,
        SnakeCase,
        Uppercase,
    }

    impl RenamingRule {
        pub fn apply_renaming_rule(&self, name: &str) -> String {
            use heck::*;

            match self {
                RenamingRule::CamelCase => name.to_lower_camel_case(),
                RenamingRule::KebabCase => name.to_kebab_case(),
                RenamingRule::Lowercase => name.to_lowercase(),
                RenamingRule::PascalCase => name.to_upper_camel_case(),
                RenamingRule::ScreamingKebabCase => name.to_shouty_kebab_case(),
                RenamingRule::ScreamingSnakeCase => name.to_shouty_snake_case(),
                RenamingRule::SnakeCase => name.to_snake_case(),
                RenamingRule::Uppercase => name.to_uppercase(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RenamingRuleLitStr {
        pub lit: LitStr,
        pub rule: RenamingRule,
    }

    impl Parse for RenamingRuleLitStr {
        fn parse(input: ParseStream<'_>) -> Result<Self> {
            let string_literal: LitStr = input.parse()?;
            let rule = match string_literal.value().as_ref() {
                "camelCase" => RenamingRule::CamelCase,
                "kebab-case" => RenamingRule::KebabCase,
                "lowercase" => RenamingRule::Lowercase,
                "PascalCase" => RenamingRule::PascalCase,
                "SCREAMING-KEBAB-CASE" => RenamingRule::ScreamingKebabCase,
                "SCREAMING_SNAKE_CASE" => RenamingRule::ScreamingSnakeCase,
                "snake_case" => RenamingRule::SnakeCase,
                "UPPERCASE" => RenamingRule::Uppercase,
                _ => {
                    bail_spanned!(string_literal.span() => "expected a valid renaming rule, possible values are: \"camelCase\", \"kebab-case\", \"lowercase\", \"PascalCase\", \"SCREAMING-KEBAB-CASE\", \"SCREAMING_SNAKE_CASE\", \"snake_case\", \"UPPERCASE\"")
                }
            };
            Ok(Self {
                lit: string_literal,
                rule,
            })
        }
    }

    impl ToTokens for RenamingRuleLitStr {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.lit.to_tokens(tokens)
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum TextSignatureAttributeValue {
        Str(LitStr),
        // `None` ident to disable automatic text signature generation
        Disabled(Ident),
    }

    impl Parse for TextSignatureAttributeValue {
        fn parse(input: ParseStream<'_>) -> Result<Self> {
            if let Ok(lit_str) = input.parse::<LitStr>() {
                return Ok(TextSignatureAttributeValue::Str(lit_str));
            }

            let err_span = match input.parse::<Ident>() {
                Ok(ident) if ident == "None" => {
                    return Ok(TextSignatureAttributeValue::Disabled(ident));
                }
                Ok(other_ident) => other_ident.span(),
                Err(e) => e.span(),
            };

            Err(err_spanned!(err_span => "expected a string literal or `None`"))
        }
    }

    impl ToTokens for TextSignatureAttributeValue {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                TextSignatureAttributeValue::Str(s) => s.to_tokens(tokens),
                TextSignatureAttributeValue::Disabled(b) => b.to_tokens(tokens),
            }
        }
    }

    pub type ExtendsAttribute = KeywordAttribute<kw::extends, Path>;
    pub type FreelistAttribute = KeywordAttribute<kw::freelist, Box<Expr>>;
    pub type ModuleAttribute = KeywordAttribute<kw::module, LitStr>;
    pub type NameAttribute = KeywordAttribute<kw::name, NameLitStr>;
    pub type RenameAllAttribute = KeywordAttribute<kw::rename_all, RenamingRuleLitStr>;

    impl<K: Parse + std::fmt::Debug, V: Parse> Parse for KeywordAttribute<K, V> {
        fn parse(input: ParseStream<'_>) -> Result<Self> {
            let kw: K = input.parse()?;
            let _: Token![=] = input.parse()?;
            let value = input.parse()?;
            Ok(KeywordAttribute { kw, value })
        }
    }

    impl<K: ToTokens, V: ToTokens> ToTokens for KeywordAttribute<K, V> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.kw.to_tokens(tokens);
            Token![=](self.kw.span()).to_tokens(tokens);
            self.value.to_tokens(tokens);
        }
    }
    pub type CrateAttribute = KeywordAttribute<Token![crate], LitStrValue<Path>>;
}

pub mod pyclass {
    use super::*;

    use attributes::{
        self, kw, CrateAttribute, ExtendsAttribute, FreelistAttribute, ModuleAttribute,
        NameAttribute, RenameAllAttribute,
    };

    #[derive(Clone, Default)]
    pub struct PyClassPyO3Options {
        pub krate: Option<CrateAttribute>,
        pub dict: Option<kw::dict>,
        pub extends: Option<ExtendsAttribute>,
        pub get_all: Option<kw::get_all>,
        pub freelist: Option<FreelistAttribute>,
        pub frozen: Option<kw::frozen>,
        pub mapping: Option<kw::mapping>,
        pub module: Option<ModuleAttribute>,
        pub name: Option<NameAttribute>,
        pub rename_all: Option<RenameAllAttribute>,
        pub sequence: Option<kw::sequence>,
        pub set_all: Option<kw::set_all>,
        pub subclass: Option<kw::subclass>,
        pub unsendable: Option<kw::unsendable>,
        pub weakref: Option<kw::weakref>,
    }

    enum PyClassPyO3Option {
        Crate(CrateAttribute),
        Dict(kw::dict),
        Extends(ExtendsAttribute),
        Freelist(FreelistAttribute),
        Frozen(kw::frozen),
        GetAll(kw::get_all),
        Mapping(kw::mapping),
        Module(ModuleAttribute),
        Name(NameAttribute),
        RenameAll(RenameAllAttribute),
        Sequence(kw::sequence),
        SetAll(kw::set_all),
        Subclass(kw::subclass),
        Unsendable(kw::unsendable),
        Weakref(kw::weakref),
    }

    impl Parse for PyClassPyO3Option {
        fn parse(input: ParseStream<'_>) -> Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![crate]) {
                input.parse().map(PyClassPyO3Option::Crate)
            } else if lookahead.peek(kw::dict) {
                input.parse().map(PyClassPyO3Option::Dict)
            } else if lookahead.peek(kw::extends) {
                input.parse().map(PyClassPyO3Option::Extends)
            } else if lookahead.peek(attributes::kw::freelist) {
                input.parse().map(PyClassPyO3Option::Freelist)
            } else if lookahead.peek(attributes::kw::frozen) {
                input.parse().map(PyClassPyO3Option::Frozen)
            } else if lookahead.peek(attributes::kw::get_all) {
                input.parse().map(PyClassPyO3Option::GetAll)
            } else if lookahead.peek(attributes::kw::mapping) {
                input.parse().map(PyClassPyO3Option::Mapping)
            } else if lookahead.peek(attributes::kw::module) {
                input.parse().map(PyClassPyO3Option::Module)
            } else if lookahead.peek(kw::name) {
                input.parse().map(PyClassPyO3Option::Name)
            } else if lookahead.peek(kw::rename_all) {
                input.parse().map(PyClassPyO3Option::RenameAll)
            } else if lookahead.peek(attributes::kw::sequence) {
                input.parse().map(PyClassPyO3Option::Sequence)
            } else if lookahead.peek(attributes::kw::set_all) {
                input.parse().map(PyClassPyO3Option::SetAll)
            } else if lookahead.peek(attributes::kw::subclass) {
                input.parse().map(PyClassPyO3Option::Subclass)
            } else if lookahead.peek(attributes::kw::unsendable) {
                input.parse().map(PyClassPyO3Option::Unsendable)
            } else if lookahead.peek(attributes::kw::weakref) {
                input.parse().map(PyClassPyO3Option::Weakref)
            } else {
                Err(lookahead.error())
            }
        }
    }

    impl Parse for PyClassPyO3Options {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            let mut options: PyClassPyO3Options = Default::default();

            for option in Punctuated::<PyClassPyO3Option, syn::Token![,]>::parse_terminated(input)?
            {
                options.set_option(option)?;
            }

            Ok(options)
        }
    }

    impl PyClassPyO3Options {
        fn set_option(&mut self, option: PyClassPyO3Option) -> syn::Result<()> {
            macro_rules! set_option {
            ($key:ident) => {
                {
                    ensure_spanned!(
                        self.$key.is_none(),
                        $key.span() => concat!("`", stringify!($key), "` may only be specified once")
                    );
                    self.$key = Some($key);
                }
            };
        }

            match option {
                PyClassPyO3Option::Crate(krate) => set_option!(krate),
                PyClassPyO3Option::Dict(dict) => set_option!(dict),
                PyClassPyO3Option::Extends(extends) => set_option!(extends),
                PyClassPyO3Option::Freelist(freelist) => set_option!(freelist),
                PyClassPyO3Option::Frozen(frozen) => set_option!(frozen),
                PyClassPyO3Option::GetAll(get_all) => set_option!(get_all),
                PyClassPyO3Option::Mapping(mapping) => set_option!(mapping),
                PyClassPyO3Option::Module(module) => set_option!(module),
                PyClassPyO3Option::Name(name) => set_option!(name),
                PyClassPyO3Option::RenameAll(rename_all) => set_option!(rename_all),
                PyClassPyO3Option::Sequence(sequence) => set_option!(sequence),
                PyClassPyO3Option::SetAll(set_all) => set_option!(set_all),
                PyClassPyO3Option::Subclass(subclass) => set_option!(subclass),
                PyClassPyO3Option::Unsendable(unsendable) => set_option!(unsendable),
                PyClassPyO3Option::Weakref(weakref) => set_option!(weakref),
            }
            Ok(())
        }
    }

    #[derive(Default)]
    pub struct FieldPyO3Options {
        pub get: Option<kw::get>,
        pub set: Option<kw::set>,
        pub name: Option<NameAttribute>,
    }

    enum FieldPyO3Option {
        Get(attributes::kw::get),
        Set(attributes::kw::set),
        Name(NameAttribute),
    }

    impl Parse for FieldPyO3Option {
        fn parse(input: ParseStream<'_>) -> Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(attributes::kw::get) {
                input.parse().map(FieldPyO3Option::Get)
            } else if lookahead.peek(attributes::kw::set) {
                input.parse().map(FieldPyO3Option::Set)
            } else if lookahead.peek(attributes::kw::name) {
                input.parse().map(FieldPyO3Option::Name)
            } else {
                Err(lookahead.error())
            }
        }
    }

    impl FieldPyO3Options {
        fn set_option(&mut self, option: FieldPyO3Option) -> syn::Result<()> {
            macro_rules! set_option {
                ($key:ident) => {
                    {
                        ensure_spanned!(
                            self.$key.is_none(),
                            $key.span() => concat!("`", stringify!($key), "` may only be specified once")
                        );
                        self.$key = Some($key);
                    }
                };
            }

            match option {
                FieldPyO3Option::Get(get) => set_option!(get),
                FieldPyO3Option::Set(set) => set_option!(set),
                FieldPyO3Option::Name(name) => set_option!(name),
            }
            Ok(())
        }
    }

    impl Parse for FieldPyO3Options {
        fn parse(input: ParseStream) -> Result<Self> {
            let mut options: FieldPyO3Options = Default::default();
            for option in Punctuated::<FieldPyO3Option, syn::Token![,]>::parse_terminated(input)? {
                options.set_option(option)?;
            }

            Ok(options)
        }
    }
}
