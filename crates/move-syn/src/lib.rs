#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

//! Move syntax parsing using [`unsynn`](::unsynn).

use std::borrow::Cow;
use std::collections::HashMap;

pub use unsynn;
use unsynn::*;

#[cfg(test)]
mod tests;

/// Process raw Move code so that it can be used as input to Rust's tokenizer.
///
/// Move's and Rust's tokens are very similar, with the exception of raw identifiers for which Move
/// uses the syntax "`ident`".
///
/// This function the backticks around identifiers, if found. Thus, we can re-use Rust's tokenizer
/// afterwards, implemented by the [`proc_macro2`] crate. This is relevant because
/// [`unsynn!`]-generated types requires Rust's [`TokenStream`] as input for parsing.
pub fn sanitize_for_tokenizer(content: &str) -> String {
    let regex = raw_ident_regex();
    let mut lines = content.lines().map(|line| {
        // Ignore commented or doc lines
        if !line.trim_start().starts_with("//") {
            regex.replace(line, "$1")
        } else {
            Cow::Borrowed(line)
        }
    });
    lines.next().map_or_else(String::new, |line| {
        let mut sanitized = String::with_capacity(content.len());
        sanitized.push_str(&line);
        for line in lines {
            sanitized.push('\n');
            sanitized.push_str(&line);
        }
        sanitized
    })
}

fn raw_ident_regex() -> regex::Regex {
    regex::Regex::new("`([[:alnum:]_]+)`").expect("Valid regex")
}

pub mod kw {
    //! Move keywords.
    use unsynn::*;

    unsynn! {
        pub keyword Struct = "struct";
        pub keyword Phantom = "phantom";
        pub keyword Public = "public";
        pub keyword Has = "has";
        pub keyword Copy = "copy";
        pub keyword Drop = "drop";
        pub keyword Key = "key";
        pub keyword Store = "store";
        pub keyword Module = "module";
        pub keyword Package = "package";
        pub keyword Friend = "friend";
        pub keyword Use = "use";
        pub keyword Fun = "fun";
        pub keyword As = "as";
        pub keyword Const = "const";
        pub keyword Mut = "mut";
        pub keyword Entry = "entry";
        pub keyword Native = "native";
        pub keyword Macro = "macro";
        pub keyword Vector = "vector";
        pub keyword Enum = "enum";
    }
}

unsynn! {
    pub enum File {
        /// A Move file in the 2024 recommended format.
        ModuleLabel(LabeledModule),
        /// A Move file in the legacy style.
        Legacy(Vec<Module>),
    }

    /// A single module defined with a top-level [label].
    ///
    /// [label]: https://move-book.com/guides/code-quality-checklist#using-module-label
    pub struct LabeledModule {
        attrs: Vec<Attribute>,
        keyword: kw::Module,
        named_addres: Ident,
        path_sep: PathSep,
        ident: Ident,
        semicolon: Semicolon,
        contents: Vec<Item>,
    }

    /// A Move module declaration.
    pub struct Module {
        pub attrs: Vec<Attribute>,
        keyword: kw::Module,
        pub named_addres: Ident,
        path_sep: PathSep,
        pub ident: Ident,
        contents: BraceGroupContaining<Vec<Item>>,
    }
    /// A Move language item.
    pub struct Item {
        pub attrs: Vec<Attribute>,
        vis: Option<Visibility>,
        pub kind: ItemKind,
    }

    /// An attribute like `#[test_only]`, `#[allow(...)]`, doc comment (`/// ...`), etc.
    #[derive(Clone)]
    pub struct Attribute {
        pound: Pound,
        contents: BracketGroupContaining<AttributeContent>,
    }

    #[derive(Clone)]
    enum AttributeContent {
        Doc(Cons<DocKw, Assign, LiteralString>),
        Other(Vec<TokenTree>),
    }

    keyword DocKw = "doc";

    /// Move item visibility.
    ///
    /// `public`, `public(package)`, `public(friend)`
    #[derive(Clone)]
    struct Visibility {
        public: kw::Public,
        modifier: Option<ParenthesisGroupContaining<VisibilityModifier>>,
    }

    /// Move item visibility modifier.
    ///
    /// Examples:
    /// - `public(package)`
    /// - `public(friend)`
    #[derive(Clone)]
    enum VisibilityModifier {
        Package(kw::Package),
        Friend(kw::Friend)
    }

    /// All Move item types.
    #[non_exhaustive]
    pub enum ItemKind {
        Struct(Struct),
        Enum(Enum),
        Import(Import),
        UseFun(UseFun),
        Const(Const),
        Function(Function),
        MacroFun(MacroFun),
        NativeFun(NativeFun)
    }

    pub struct MacroFun {
        macro_kw: kw::Macro,
        fun_kw: kw::Fun,
        ident: Ident,
        generics: Option<MacroGenerics>,
        args: ParenthesisGroup,
        ret: Option<Cons<Colon, Either<MacroReturn, ParenthesisGroup>>>,
        body: BraceGroup,
    }

    struct MacroGenerics {
        lt_token: Lt,
        type_args: DelimitedVec<MacroTypeArg, Comma>,
        gt_token: Gt,
    }

    /// `$T: drop + store`
    struct MacroTypeArg{
        name: MacroTypeName,
        bounds: Option<GenericBounds>,
    }

    /// Either `_` or a 'concrete' type
    enum MacroReturn {
        Underscore(Underscore),
        Concrete(Cons<Option<Ref>, MacroReturnType>),
    }

    /// Return type for macro funs.
    ///
    /// - `$T`
    /// - `&mut $T`
    /// - `&String`
    /// - `Option<$T>`
    enum MacroReturnType {
        MacroTypeName(MacroTypeName),
        Hybrid(HybridMacroType)
    }

    struct HybridMacroType {
        ident: Ident,
        type_args: Option<Cons<Lt, Many<Either<Type, MacroTypeName, Box<HybridMacroType>>, Comma>, Gt>>
    }

    /// `$T`
    struct MacroTypeName {
        dollar: Dollar,
        ident: Ident,
    }

    pub struct NativeFun {
        native_kw: kw::Native,
        fun_kw: kw::Fun,
        ident: Ident,
        generics: Option<Generics>,
        args: ParenthesisGroup,
        ret: Option<Cons<Colon, Either<MaybeRefType, ParenthesisGroup>>>,
        semicolon: Semicolon
    }

    pub struct Function {
        entry: Option<kw::Entry>,
        fun_kw: kw::Fun,
        ident: Ident,
        generics: Option<Generics>,
        args: ParenthesisGroup,
        ret: Option<Cons<Colon, Either<MaybeRefType, ParenthesisGroup>>>,
        body: BraceGroup,
    }

    pub struct Const {
        const_kw: kw::Const,
        ident: Ident,
        colon: Colon,
        ty: Type,
        assign: Assign,
        expr: ConstVal,
        semicolon: Semicolon,
    }

    enum ConstVal {
        Literal(Literal),
        Vector(Cons<kw::Vector, BracketGroup>),
        NamedAddress(Cons<At, Literal>),
        // Hack to parse anything until (but excluding) a `;`
        Expr(Vec<Cons<Except<Semicolon>, TokenTree>>),
    }

    pub struct Import {
        keyword: kw::Use,
        named_address: Ident,
        path_sep: PathSep,
        module: ImportModule,
        semicolon: Semicolon,
    }

    /// `module`, `module as alias`, `module::...`, `{module, ...}`
    enum ImportModule {
        One(ModuleOrItems),
        Many(BraceGroupContaining<CommaDelimitedVec<ModuleOrItems>>),
    }

    #[derive(Clone)]
    struct ModuleOrItems {
        ident: MaybeAliased,
        items: Option<Cons<PathSep, ImportItem>>,
    }

    #[derive(Clone)]
    enum ImportItem {
        One(MaybeAliased),
        Many(BraceGroupContaining<CommaDelimitedVec<MaybeAliased>>)
    }

    #[derive(Clone)]
    struct MaybeAliased {
        ident: Ident,
        alias: Option<Cons<kw::As, Ident>>,
    }

    pub struct UseFun {
        keyword: kw::Use,
        fun_kw: kw::Fun,
        path_prefix: Option<Cons<Ident, PathSep, Ident, PathSep>>,
        fun: Ident,
        as_kw: kw::As,
        ty: Ident,
        dot: Dot,
        method: Ident,
        semicolon: Semicolon,
    }

    // === Structs ===

    /// A Move struct.
    #[derive(Clone)]
    pub struct Struct {
        keyword: kw::Struct,
        pub ident: Ident,
        pub generics: Option<Generics>,
        pub kind: StructKind,
    }

    /// The kinds of structs; either a braced or tuple one.
    #[derive(Clone)]
    pub enum StructKind {
        Braced(BracedStruct),
        Tuple(TupleStruct),
    }

    /// Braced structs have their abilities declared before their fields.
    #[derive(Clone)]
    pub struct BracedStruct {
        abilities: Option<Abilities>,
        contents: NamedFields,
    }

    /// Tuple structs have their abilities declared after their fields, with a trailing semicolon
    /// if so.
    #[derive(Clone)]
    pub struct TupleStruct {
        contents: PositionalFields,
        abilities: Option<Cons<Abilities, Semicolon>>
    }

    // === Enums ===

    #[derive(Clone)]
    pub struct Enum {
        keyword: kw::Enum,
        pub ident: Ident,
        pub generics: Option<Generics>,
        pub abilities: Option<Abilities>,
        content: BraceGroupContaining<CommaDelimitedVec<EnumVariant>>,
    }

    #[derive(Clone)]
    pub struct EnumVariant {
        pub attrs: Vec<Attribute>,
        pub ident: Ident,
        fields: Option<FieldsKind>
    }

    #[derive(Clone)]
    enum FieldsKind {
        Positional(PositionalFields),
        Named(NamedFields),
    }

    // === Datatype fields ===

    #[derive(Clone)]
    struct PositionalFields(ParenthesisGroupContaining<DelimitedVec<UnnamedField, Comma>>);

    #[derive(Clone)]
    struct NamedFields(BraceGroupContaining<DelimitedVec<NamedField, Comma>>);

    /// Field in a braced struct.
    #[derive(Clone)]
    pub struct NamedField {
        pub attrs: Vec<Attribute>,
        pub ident: Ident,
        colon: Colon,
        pub ty: Type,
    }

    /// Field in a tuple struct.
    #[derive(Clone)]
    pub struct UnnamedField {
        pub attrs: Vec<Attribute>,
        pub ty: Type,
    }

    /// Type of a datatype field.
    #[derive(Clone)]
    pub struct Type {
        pub path: TypePath,
        pub type_args: Option<TypeArgs>
    }

    // === Generics ===

    /// The generics of a type.
    ///
    /// # Example
    /// `<T, U: drop, V: key + store>`
    #[derive(Clone)]
    pub struct Generics {
        lt_token: Lt,
        type_args: DelimitedVec<Generic, Comma>,
        gt_token: Gt,
    }

    /// A generic type declaration.
    ///
    /// # Examples
    /// * `T`
    /// * `T: drop`
    /// * `T: key + store`
    /// * `phantom T`
    #[derive(Clone)]
    pub struct Generic {
        pub phantom: Option<kw::Phantom>,
        pub ident: Ident,
        bounds: Option<GenericBounds>
    }

    /// Slightly convoluted, but captures the fact that:
    /// * `:` must be followed by an ability
    /// * additional abilities are preceeded by `+`
    #[derive(Clone)]
    struct GenericBounds {
        colon: Colon,
        first_ability: Ability,
        extra_abilities: Vec<Cons<Plus, Ability>>
    }

    // === Abilities ===

    /// Abilities declaration for a datatype.
    ///
    /// Example: `has key, store`
    #[derive(Clone)]
    struct Abilities {
        has: kw::Has,
        keywords: Many<Ability, Comma>,
    }

    /// Ability keywords.
    #[derive(Clone)]
    pub enum Ability {
        Copy(kw::Copy),
        Drop(kw::Drop),
        Key(kw::Key),
        Store(kw::Store),
    }

    /// Type of function arguments or returns.
    struct MaybeRefType {
        r#ref: Option<Ref>,
        r#type: Type,
    }

    /// The reference prefix
    struct Ref {
        and: And,
        r#mut: Option<kw::Mut>,
    }

    /// Path to a type.
    #[derive(Clone)]
    pub enum TypePath {
        /// Fully qualified,
        Full {
            named_address: Ident,
            sep0: PathSep,
            module: Ident,
            sep1: PathSep,
            r#type: Ident,
        },
        /// Module prefix only, if it was imported already.
        Module {
            module: Ident,
            sep: PathSep,
            r#type: Ident,
        },
        /// Only the type identifier.
        Ident(Ident),
    }

    #[derive(Clone)]
    pub struct TypeArgs {
        lt: Lt,
        args: Many<Box<Type>, Comma>,
        gt: Gt,
    }
}

impl File {
    pub fn into_modules(self) -> impl Iterator<Item = Module> {
        match self {
            Self::ModuleLabel(labeled) => std::iter::once(labeled.into_module()).boxed(),
            Self::Legacy(modules) => modules.into_iter().boxed(),
        }
    }
}

impl LabeledModule {
    pub fn into_module(self) -> Module {
        Module {
            attrs: self.attrs,
            keyword: self.keyword,
            named_addres: self.named_addres,
            path_sep: self.path_sep,
            ident: self.ident,
            contents: BraceGroupContaining {
                content: self.contents,
            },
        }
    }
}

impl Module {
    pub fn structs_mut(&mut self) -> impl Iterator<Item = &mut Struct> {
        self.items_mut().filter_map(|item| match item.kind {
            ItemKind::Struct(ref mut s) => Some(s),
            _ => None,
        })
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.contents.content.iter()
    }

    fn items_mut(&mut self) -> impl Iterator<Item = &mut Item> {
        self.contents.content.iter_mut()
    }

    /// Add `sui` implicit imports as explicit `use` statements to the module.
    ///
    /// [Reference](https://move-book.com/programmability/sui-framework#implicit-imports)
    pub fn with_implicit_sui_imports(&mut self) -> &mut Self {
        // Build the map of implicit imports keyed by the identifiers they export.
        let implicit_imports: HashMap<_, _> = [
            "use sui::object;",
            "use sui::object::ID;",
            "use sui::object::UID;",
            "use sui::tx_context;",
            "use sui::tx_context::TxContext;",
            "use sui::transfer;",
        ]
        .into_iter()
        .map(|text| {
            text.to_token_iter()
                .parse_all::<Import>()
                .expect("Valid imports")
        })
        .map(|import| {
            let ident = import
                .imported_idents()
                .next()
                .expect("Each import exposes exactly one ident");
            (ident.clone(), import)
        })
        .collect();

        self.add_implicit_imports(implicit_imports)
    }

    /// Add `iota` implicit imports as explicit `use` statements to the module.
    ///
    /// Adapted from the `sui` equivalents.
    pub fn with_implicit_iota_imports(&mut self) -> &mut Self {
        // Build the map of implicit imports keyed by the identifiers they export.
        let implicit_imports: HashMap<_, _> = [
            "use iota::object;",
            "use iota::object::ID;",
            "use iota::object::UID;",
            "use iota::tx_context;",
            "use iota::tx_context::TxContext;",
            "use iota::transfer;",
        ]
        .into_iter()
        .map(|text| {
            text.to_token_iter()
                .parse_all::<Import>()
                .expect("Valid imports")
        })
        .map(|import| {
            let ident = import
                .imported_idents()
                .next()
                .expect("Each import exposes exactly one ident");
            (ident.clone(), import)
        })
        .collect();

        self.add_implicit_imports(implicit_imports)
    }

    /// Resolve all struct field types to their fully-qualified paths.
    pub fn fully_qualify_struct_field_types(&mut self) -> &mut Self {
        let imports: HashMap<_, _> = self
            .items()
            .filter_map(|item| match &item.kind {
                ItemKind::Import(import) => Some(import),
                _ => None,
            })
            .flat_map(|import| import.flatten())
            .collect();

        for struct_ in self.structs_mut() {
            struct_.resolve_field_types(&imports);
        }
        self
    }

    fn add_implicit_imports(&mut self, mut implicit_imports: HashMap<Ident, Import>) -> &mut Self {
        // Filter out any that were shadowed by existing imports
        for item in self.items() {
            let ItemKind::Import(import) = &item.kind else {
                continue;
            };
            for ident in import.imported_idents() {
                implicit_imports.remove(ident);
            }
        }

        // Add the remaining implicit imports to the list of module items
        for (_, import) in implicit_imports {
            self.contents.content.push(Item {
                attrs: vec![],
                vis: None,
                kind: ItemKind::Import(import),
            })
        }
        self
    }
}

impl Import {
    fn flatten(&self) -> impl Iterator<Item = (Ident, FlatImport)> + use<> {
        let named_address = self.named_address.clone();
        match &self.module {
            // use named_address::module...
            ImportModule::One(module_or_items) => module_or_items.flatten(named_address),
            // use named_address::{...}
            ImportModule::Many(BraceGroupContaining {
                content: DelimitedVec(ms),
            }) => ms
                .clone()
                .into_iter()
                .flat_map(move |Delimited { value, .. }| value.flatten(named_address.clone()))
                .boxed(),
        }
    }

    /// The list of item idents brought into scope by this import.
    fn imported_idents(&self) -> impl Iterator<Item = &Ident> {
        match &self.module {
            ImportModule::One(module_or_items) => module_or_items.available_idents(),
            ImportModule::Many(BraceGroupContaining {
                content: DelimitedVec(ms),
            }) => ms
                .iter()
                .flat_map(|delimited| delimited.value.available_idents())
                .boxed(),
        }
    }
}

impl ModuleOrItems {
    fn flatten(&self, named_address: Ident) -> Box<dyn Iterator<Item = (Ident, FlatImport)>> {
        let Self { ident, items } = self;
        let module = ident.clone();
        let Some(Cons { second: items, .. }) = items else {
            // module( as alias)?
            return std::iter::once((
                module.available_as().clone(),
                FlatImport::Module {
                    named_address,
                    module: module.ident,
                },
            ))
            .boxed();
        };
        match items {
            // module::item( as alias)?
            ImportItem::One(maybe_aliased) => std::iter::once((
                maybe_aliased.available_as().clone(),
                FlatImport::Item {
                    named_address,
                    // FIXME: module can't have alias here
                    module: module.ident,
                    r#type: maybe_aliased.ident.clone(),
                },
            ))
            .boxed(),
            // module::{(item( as alias)?),+}
            ImportItem::Many(BraceGroupContaining {
                content: DelimitedVec(items),
            }) => {
                let delimiteds = items.clone();
                delimiteds
                    .into_iter()
                    .map(move |Delimited { value, .. }| {
                        (
                            value.available_as().clone(),
                            FlatImport::Item {
                                named_address: named_address.clone(),
                                // FIXME: module can't have alias here
                                module: module.ident.clone(),
                                r#type: value.ident,
                            },
                        )
                    })
                    .boxed()
            }
        }
    }

    fn available_idents(&self) -> Box<dyn Iterator<Item = &Ident> + '_> {
        let Some(Cons { second: items, .. }) = &self.items else {
            return std::iter::once(self.ident.available_as()).boxed();
        };
        match items {
            ImportItem::One(item) => std::iter::once(item.available_as()).boxed(),
            ImportItem::Many(BraceGroupContaining {
                content: DelimitedVec(items),
            }) => items
                .iter()
                .map(|delimited| delimited.value.available_as())
                .boxed(),
        }
    }
}

impl MaybeAliased {
    /// The identifier that's available in scope;
    const fn available_as(&self) -> &Ident {
        if let Some(Cons { second: alias, .. }) = &self.alias {
            alias
        } else {
            &self.ident
        }
    }
}

impl Attribute {
    pub const fn is_doc(&self) -> bool {
        matches!(self.contents.content, AttributeContent::Doc(_))
    }
}

impl Struct {
    pub fn abilities(&self) -> impl Iterator<Item = &Ability> {
        use StructKind as K;
        match &self.kind {
            K::Braced(braced) => braced
                .abilities
                .iter()
                .flat_map(|a| a.keywords.0.iter())
                .map(|d| &d.value)
                .boxed(),
            K::Tuple(tuple) => tuple
                .abilities
                .iter()
                .flat_map(|a| a.first.keywords.0.iter())
                .map(|d| &d.value)
                .boxed(),
        }
    }

    pub fn field_types_mut(&mut self) -> impl Iterator<Item = &mut Type> {
        use StructKind as K;
        match &mut self.kind {
            K::Tuple(TupleStruct { contents, .. }) => contents.types_mut().boxed(),
            K::Braced(BracedStruct { contents, .. }) => contents.types_mut().boxed(),
        }
    }

    fn resolve_field_types(&mut self, imports: &HashMap<Ident, FlatImport>) {
        for ty in self.field_types_mut() {
            ty.resolve(imports);
        }
    }
}

impl NamedFields {
    fn types_mut(&mut self) -> impl Iterator<Item = &mut Type> {
        self.0
            .content
            .0
            .iter_mut()
            .map(|Delimited { value: field, .. }| &mut field.ty)
    }
}

impl PositionalFields {
    fn types_mut(&mut self) -> impl Iterator<Item = &mut Type> {
        self.0
            .content
            .0
            .iter_mut()
            .map(|Delimited { value: field, .. }| &mut field.ty)
    }
}

impl Generics {
    pub fn generics(&self) -> impl Iterator<Item = &Generic> + '_ {
        self.type_args.0.iter().map(|d| &d.value)
    }
}

impl BracedStruct {
    pub fn fields(&self) -> impl Iterator<Item = &NamedField> + Clone + '_ {
        self.contents.0.content.0.iter().map(|d| &d.value)
    }

    /// Whether this struct has no fields.
    pub fn is_empty(&self) -> bool {
        self.contents.0.content.0.is_empty()
    }
}

impl TupleStruct {
    pub fn fields(&self) -> impl Iterator<Item = &UnnamedField> + Clone + '_ {
        self.contents.0.content.0.iter().map(|d| &d.value)
    }

    /// Whether this struct has no fields.
    pub fn is_empty(&self) -> bool {
        self.contents.0.content.0.is_empty()
    }
}

impl Enum {
    pub fn abilities(&self) -> impl Iterator<Item = &Ability> {
        self.abilities
            .iter()
            .flat_map(|a| a.keywords.0.iter())
            .map(|d| &d.value)
    }
}

impl Type {
    pub fn type_args_mut(&mut self) -> impl Iterator<Item = &mut Self> {
        self.type_args
            .iter_mut()
            .flat_map(|args| args.args.0.iter_mut().map(|d| &mut *d.value))
    }

    /// Resolve the types' path to a fully-qualified declaration, recursively.
    fn resolve(&mut self, imports: &HashMap<Ident, FlatImport>) {
        use TypePath as P;
        // First, resolve the type arguments
        for ty in self.type_args_mut() {
            ty.resolve(imports);
        }

        // Then resolve its own path
        // HACK: We trust the Move code is valid, so the expected import should always be found,
        // hence we don't error/panic if it isn't
        let resolved = match &self.path {
            P::Full { .. } => return,
            P::Module { module, r#type, .. } => {
                let Some(FlatImport::Module {
                    named_address,
                    module,
                }) = imports.get(module)
                else {
                    return;
                };
                P::Full {
                    named_address: named_address.clone(),
                    sep0: PathSep::default(),
                    module: module.clone(),
                    sep1: PathSep::default(),
                    r#type: r#type.clone(),
                }
            }
            P::Ident(ident) => {
                let Some(FlatImport::Item {
                    named_address,
                    module,
                    r#type,
                }) = imports.get(ident)
                else {
                    return;
                };
                P::Full {
                    named_address: named_address.clone(),
                    sep0: PathSep::default(),
                    module: module.clone(),
                    sep1: PathSep::default(),
                    r#type: r#type.clone(),
                }
            }
        };
        self.path = resolved;
    }
}

impl TypeArgs {
    /// Guaranteed to be non-empty.
    pub fn types(&self) -> impl Iterator<Item = &Type> {
        self.args.0.iter().map(|args| &*args.value)
    }
}

// === Non-lang items ===

enum FlatImport {
    Module {
        named_address: Ident,
        module: Ident,
    },
    Item {
        named_address: Ident,
        module: Ident,
        r#type: Ident,
    },
}

// === Misc helpers ===

/// Box an iterator, necessary when returning different types that implement [`Iterator`].
trait IteratorBoxed<'a>: Iterator + 'a {
    fn boxed(self) -> Box<dyn Iterator<Item = Self::Item> + 'a>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl<'a, T> IteratorBoxed<'a> for T where T: Iterator + 'a {}
