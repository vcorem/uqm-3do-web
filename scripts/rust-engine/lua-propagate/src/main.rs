// SPDX-License-Identifier: GPL-2.0-or-later
// Source-range changes keep generated engine bodies and upstream notices reviewable.
use proc_macro2::{LineColumn, Span};
use quote::ToTokens;
use std::collections::{BTreeMap, BTreeSet};
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use syn::{
    spanned::Spanned,
    visit::{self, Visit},
    Expr, ForeignItem, Item, ReturnType, Type,
};

const LUA_APIS: &str = "luaL_checkinteger luaL_checklstring luaL_checkoption luaL_checktype luaL_error luaL_loadstring luaL_newstate luaL_setfuncs lua_absindex lua_callk lua_close lua_createtable lua_getfield lua_getglobal lua_gettable lua_gettop lua_load lua_pcallk lua_pushboolean lua_pushcclosure lua_pushinteger lua_pushlightuserdata lua_pushnil lua_pushstring lua_pushvalue lua_rawgeti lua_rawseti lua_replace lua_setfield lua_settable lua_settop lua_toboolean lua_tointegerx lua_tolstring lua_tonumberx lua_topointer lua_type lua_typename";
#[derive(Clone)]
struct Call {
    name: Option<String>,
    span: Span,
}
struct Calls(Vec<Call>, bool);
impl<'ast> Visit<'ast> for Calls {
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        let name = direct_name(call);
        self.0.push(Call {
            name,
            span: call.span(),
        });
        visit::visit_expr_call(self, call);
    }
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if mac.path.is_ident("uqm_lua_checked") {
            if let Ok(checked) = syn::parse2::<Checked>(mac.tokens.clone()) {
                self.visit_expr(&checked.operation);
            }
        }
    }
    fn visit_expr_closure(&mut self, closure: &'ast syn::ExprClosure) {
        self.1 = true;
        visit::visit_expr_closure(self, closure);
    }
    fn visit_item_fn(&mut self, _: &'ast syn::ItemFn) {} // nested functions have their own scope
}
struct Function {
    name: String,
    calls: Vec<Call>,
    output: ReturnType,
    contains_closure: bool,
}
struct File {
    path: PathBuf,
    text: String,
    tree: syn::File,
    functions: Vec<Function>,
}
fn paths(path: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(path).expect("read source directory") {
        let p = entry.unwrap().path();
        if p.is_dir() {
            paths(&p, out);
        } else if p.extension().is_some_and(|e| e == "rs") {
            out.push(p);
        }
    }
}
fn offset(text: &str, starts: &[usize], pos: LineColumn) -> usize {
    let base = starts[pos.line - 1];
    base + text[base..]
        .char_indices()
        .nth(pos.column)
        .map_or(text.len() - base, |(byte, _)| byte)
}
struct Checked {
    operation: Expr,
    failure: Expr,
}
impl syn::parse::Parse for Checked {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let operation = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let failure = input.parse()?;
        Ok(Self { operation, failure })
    }
}
struct ValidateMacros {
    count: usize,
    error: Option<String>,
}
impl<'ast> Visit<'ast> for ValidateMacros {
    fn visit_macro(&mut self, expr: &'ast syn::Macro) {
        if expr.path.is_ident("uqm_lua_checked") {
            match syn::parse2::<Checked>(expr.tokens.clone()) {
                Ok(checked) => {
                    if !matches!(checked.operation, Expr::Call(_)) {
                        self.error = Some("checked operation is not a call".into());
                    }
                    self.count += 1;
                    self.visit_expr(&checked.operation);
                    self.visit_expr(&checked.failure);
                }
                Err(error) => self.error = Some(format!("invalid checked macro: {error}")),
            }
        }
    }
}
fn validate_macros(tree: &syn::File) -> Result<usize, String> {
    let mut visitor = ValidateMacros {
        count: 0,
        error: None,
    };
    visitor.visit_file(tree);
    if let Some(error) = visitor.error {
        Err(error)
    } else {
        Ok(visitor.count)
    }
}
fn zero(ty: &Type, items: &BTreeMap<String, &Item>, depth: usize) -> Result<String, String> {
    if depth > 30 {
        return Err("recursive non-pointer return type".into());
    }
    Ok(match ty {
        Type::Tuple(t) if t.elems.is_empty() => "()".into(),
        Type::Ptr(p) => if p.mutability.is_some() {
            "::core::ptr::null_mut()"
        } else {
            "::core::ptr::null()"
        }
        .into(),
        Type::Paren(p) => zero(&p.elem, items, depth + 1)?,
        Type::Group(p) => zero(&p.elem, items, depth + 1)?,
        Type::Array(a) => format!(
            "[{}; {}]",
            zero(&a.elem, items, depth + 1)?,
            a.len.to_token_stream()
        ),
        Type::Path(p) => {
            let ident = p.path.segments.last().unwrap().ident.to_string();
            match ident.as_str() {
                "bool" => "false".into(),
                "c_float" | "c_double" | "f32" | "f64" => "0.0".into(),
                "c_char" | "c_schar" | "c_uchar" | "c_short" | "c_ushort" | "c_int" | "c_uint"
                | "c_long" | "c_ulong" | "c_longlong" | "c_ulonglong" | "usize" | "isize"
                | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "u128" | "i128" => {
                    "0".into()
                }
                "Option" => "None".into(),
                _ => match items
                    .get(&ident)
                    .ok_or_else(|| format!("unresolved return type {ident}"))?
                {
                    Item::Type(t) => zero(&t.ty, items, depth + 1)?,
                    Item::Struct(s) => match &s.fields {
                        syn::Fields::Named(fields) => format!(
                            "{} {{ {} }}",
                            ident,
                            fields
                                .named
                                .iter()
                                .map(|f| Ok(format!(
                                    "{}: {}",
                                    f.ident.as_ref().unwrap(),
                                    zero(&f.ty, items, depth + 1)?
                                )))
                                .collect::<Result<Vec<_>, String>>()?
                                .join(", ")
                        ),
                        syn::Fields::Unnamed(fields) => format!(
                            "{}({})",
                            ident,
                            fields
                                .unnamed
                                .iter()
                                .map(|f| zero(&f.ty, items, depth + 1))
                                .collect::<Result<Vec<_>, _>>()?
                                .join(", ")
                        ),
                        syn::Fields::Unit => ident,
                    },
                    Item::Union(u) => {
                        let f = u.fields.named.first().ok_or("empty union")?;
                        format!(
                            "{} {{ {}: {} }}",
                            ident,
                            f.ident.as_ref().unwrap(),
                            zero(&f.ty, items, depth + 1)?
                        )
                    }
                    _ => return Err(format!("non-FFI return type {ident}")),
                },
            }
        }
        _ => {
            return Err(format!(
                "unsupported/nonzero/reference return type {}",
                ty.to_token_stream()
            ))
        }
    })
}

fn direct_name(call: &syn::ExprCall) -> Option<String> {
    match &*call.func {
        Expr::Path(path) if path.qself.is_none() => {
            let first = path.path.segments.first().unwrap().ident.to_string();
            let segments: Vec<_> = path
                .path
                .segments
                .iter()
                .map(|part| part.ident.to_string())
                .collect();
            // The audited once initializer materializes only static C expressions.
            if segments == ["crate", "uqm_engine_init", "uqm_rust_engine_init"] {
                return Some(String::new());
            }
            if path.path.segments.len() > 1
                && ["core", "std", "alloc", "Vec"].contains(&first.as_str())
            {
                Some(String::new()) // Rust intrinsics, not engine callbacks
            } else {
                Some(path.path.segments.last().unwrap().ident.to_string())
            }
        }
        _ => None,
    }
}
fn span_key(span: Span) -> (usize, usize, usize, usize) {
    let a = span.start();
    let b = span.end();
    (a.line, a.column, b.line, b.column)
}
struct Audit<'a> {
    tainted: &'a BTreeSet<String>,
    declared: &'a BTreeSet<String>,
    guarded: Option<(usize, usize, usize, usize)>,
    errors: Vec<String>,
    count: usize,
}
impl<'ast> Visit<'ast> for Audit<'_> {
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if mac.path.is_ident("uqm_lua_checked") {
            match syn::parse2::<Checked>(mac.tokens.clone()) {
                Ok(checked) => {
                    let previous = self.guarded;
                    self.guarded = Some(span_key(checked.operation.span()));
                    self.count += 1;
                    self.visit_expr(&checked.operation);
                    self.guarded = previous;
                    self.visit_expr(&checked.failure);
                }
                Err(error) => self.errors.push(error.to_string()),
            }
        } else if mac.tokens.to_string().contains("lua_")
            || mac.tokens.to_string().contains("luaL_")
        {
            self.errors
                .push("Lua call hidden inside an unsupported macro".into());
        }
    }
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        let name = direct_name(call);
        let indirect = name.as_ref().is_none_or(|n| {
            !n.is_empty()
                && !self.declared.contains(n)
                && n.chars().next().is_some_and(char::is_lowercase)
        });
        if (indirect || name.as_ref().is_some_and(|n| self.tainted.contains(n)))
            && self.guarded != Some(span_key(call.span()))
        {
            self.errors.push(format!(
                "unchecked propagating call {:?} at {:?}",
                name,
                call.span().start()
            ));
        }
        visit::visit_expr_call(self, call);
    }
}
fn audit(root: &Path, marker: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let report: serde_json::Value = serde_json::from_str(&fs::read_to_string(marker)?)?;
    let mut tainted: BTreeSet<String> = LUA_APIS.split_whitespace().map(str::to_owned).collect();
    let mut source_paths = Vec::new();
    paths(&root.join("src"), &mut source_paths);
    let mut trees = Vec::new();
    let mut declared = BTreeSet::new();
    let mut aliases = 0;
    let mut bodies = Vec::new();
    for path in source_paths {
        let tree = syn::parse_file(&fs::read_to_string(&path)?)?;
        for item in &tree.items {
            match item {
                Item::Fn(f) => {
                    declared.insert(f.sig.ident.to_string());
                    let mut calls = Calls(Vec::new(), false);
                    calls.visit_block(&f.block);
                    bodies.push((f.sig.ident.to_string(), calls.0, calls.1));
                }
                Item::ForeignMod(block) => {
                    for item in &block.items {
                        if let ForeignItem::Fn(f) = item {
                            let name = f.sig.ident.to_string();
                            declared.insert(name.clone());
                            if name.starts_with("lua_") || name.starts_with("luaL_") {
                                let relative = path.strip_prefix(root)?.to_string_lossy();
                                if !relative.starts_with("src/uqm/lua/")
                                    && relative != "src/libs/luauqm/luauqm.rs"
                                {
                                    return Err(format!(
                                        "Lua API outside audited module: {relative}: {name}"
                                    )
                                    .into());
                                }
                                if !LUA_APIS.split_whitespace().any(|api| api == name) {
                                    return Err(format!("unknown Lua API {name}").into());
                                }
                                let expected = format!("uqm_lua_guard_{name}");
                                if f.attrs.len() != 1
                                    || !f.attrs[0].path().is_ident("link_name")
                                    || !f.attrs[0]
                                        .to_token_stream()
                                        .to_string()
                                        .contains(&format!("\"{expected}\""))
                                {
                                    return Err(format!(
                                        "unguarded Lua alias {}:{name}",
                                        path.display()
                                    )
                                    .into());
                                }
                                aliases += 1;
                            }
                        }
                    }
                }
                _ => (),
            }
        }
        trees.push((path, tree));
    }
    loop {
        let before = tainted.len();
        for (name, calls, _) in &bodies {
            if calls.iter().any(|call| {
                call.name.as_ref().is_none_or(|n| {
                    !n.is_empty()
                        && !declared.contains(n)
                        && n.chars().next().is_some_and(char::is_lowercase)
                }) || call.name.as_ref().is_some_and(|n| tainted.contains(n))
            }) {
                tainted.insert(name.clone());
            }
        }
        if before == tainted.len() {
            break;
        }
    }
    for (name, _, contains_closure) in &bodies {
        if *contains_closure && tainted.contains(name) {
            return Err(
                format!("unsupported closure scope in Lua-propagating function {name}").into(),
            );
        }
    }
    let mut total = 0;
    for (path, tree) in trees {
        validate_macros(&tree).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut visitor = Audit {
            tainted: &tainted,
            declared: &declared,
            guarded: None,
            errors: Vec::new(),
            count: 0,
        };
        visitor.visit_file(&tree);
        if !visitor.errors.is_empty() {
            return Err(format!("{}: {}", path.display(), visitor.errors.join("; ")).into());
        }
        total += visitor.count;
    }
    if report["checked_calls"].as_u64() != Some(total as u64) {
        return Err("macro count does not match provenance".into());
    }
    println!("Validated {total} checked calls and {aliases} guarded Lua declarations against the transitive call graph.");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 || !["--check", "--apply", "--audit"].contains(&args[2].as_str()) {
        return Err("usage: uqm-lua-propagate ENGINE_DIR --check|--apply".into());
    }
    let root = Path::new(&args[1]);
    let marker = root.join("lua-propagation-report.json");
    if args[2] == "--audit" {
        return audit(root, &marker);
    }
    if marker.exists() {
        return Err("Lua transform already applied; regenerate clean input first".into());
    }
    let apis: BTreeSet<String> = LUA_APIS.split_whitespace().map(str::to_owned).collect();
    let mut files = Vec::new();
    let mut source_paths = Vec::new();
    paths(&root.join("src"), &mut source_paths);
    source_paths.sort();
    let mut functions = BTreeSet::new();
    let mut declared = BTreeSet::new();
    for path in source_paths {
        let text = fs::read_to_string(&path)?;
        let tree = syn::parse_file(&text).map_err(|e| format!("{}: {}", path.display(), e))?;
        let mut found = Vec::new();
        for item in &tree.items {
            match item {
                Item::Fn(f) => {
                    let mut calls = Calls(Vec::new(), false);
                    calls.visit_block(&f.block);
                    let name = f.sig.ident.to_string();
                    functions.insert(name.clone());
                    declared.insert(name.clone());
                    found.push(Function {
                        name,
                        calls: calls.0,
                        output: f.sig.output.clone(),
                        contains_closure: calls.1,
                    });
                }
                Item::ForeignMod(foreign) => {
                    for item in &foreign.items {
                        if let ForeignItem::Fn(f) = item {
                            declared.insert(f.sig.ident.to_string());
                        }
                    }
                }
                _ => (),
            }
        }
        files.push(File {
            path,
            text,
            tree,
            functions: found,
        });
    }
    let indirect = |call: &Call| {
        call.name.as_ref().is_none_or(|n| {
            !n.is_empty()
                && !declared.contains(n)
                && n.chars().next().is_some_and(char::is_lowercase)
        })
    };
    let mut tainted = apis.clone();
    loop {
        let before = tainted.len();
        for file in &files {
            for f in &file.functions {
                if f.calls
                    .iter()
                    .any(|c| indirect(c) || c.name.as_ref().is_some_and(|n| tainted.contains(n)))
                {
                    tainted.insert(f.name.clone());
                }
            }
        }
        if before == tainted.len() {
            break;
        }
    }
    let lib_path = root.join("lib.rs");
    let mut lib = fs::read_to_string(&lib_path)?;
    if lib.contains("mod uqm_lua_guard") {
        return Err("Lua macro already registered".into());
    }
    let position = lib
        .find("pub mod src")
        .ok_or("missing engine source module")?;
    lib.insert_str(
        position,
        "#[macro_use]\n#[path = \"../uqm-lua/guard.rs\"]\nmod uqm_lua_guard;\n\n",
    );
    let mut reports = Vec::new();
    let mut changes = Vec::new();
    let mut imports = BTreeSet::new();
    let mut count = 0;
    for file in files {
        let relative = file.path.strip_prefix(root)?.to_string_lossy().into_owned();
        let items: BTreeMap<_, _> = file
            .tree
            .items
            .iter()
            .filter_map(|i| match i {
                Item::Type(t) => Some((t.ident.to_string(), i)),
                Item::Struct(s) => Some((s.ident.to_string(), i)),
                Item::Union(u) => Some((u.ident.to_string(), i)),
                _ => None,
            })
            .collect();
        let starts: Vec<usize> = std::iter::once(0)
            .chain(file.text.match_indices('\n').map(|(index, _)| index + 1))
            .collect();
        let mut edits: Vec<(usize, bool, String)> = Vec::new();
        let mut fn_reports = Vec::new();
        for f in &file.functions {
            let affected: Vec<_> = f
                .calls
                .iter()
                .filter(|c| indirect(c) || c.name.as_ref().is_some_and(|n| tainted.contains(n)))
                .collect();
            if affected.is_empty() {
                continue;
            }
            if f.contains_closure {
                return Err(format!("unsupported closure scope in {relative}: {}", f.name).into());
            }
            let default = match &f.output {
                ReturnType::Default => "()".into(),
                ReturnType::Type(_, t) => {
                    zero(t, &items, 0).map_err(|e| format!("{relative}: {}: {e}", f.name))?
                }
            };
            for call in &affected {
                edits.push((
                    offset(&file.text, &starts, call.span.start()),
                    false,
                    "uqm_lua_checked!(".into(),
                ));
                edits.push((
                    offset(&file.text, &starts, call.span.end()),
                    true,
                    format!(", {default})"),
                ));
            }
            fn_reports.push(serde_json::json!({"function":f.name,"checked_calls":affected.len(),"failure_return":default}));
            count += affected.len();
        }
        for item in &file.tree.items {
            if let Item::ForeignMod(block) = item {
                for item in &block.items {
                    if let ForeignItem::Fn(f) = item {
                        let name = f.sig.ident.to_string();
                        if name.starts_with("lua_") || name.starts_with("luaL_") {
                            if !apis.contains(&name) {
                                return Err(format!("unknown Lua API {name} in {relative}").into());
                            }
                            if !relative.starts_with("src/uqm/lua/")
                                && relative != "src/libs/luauqm/luauqm.rs"
                            {
                                return Err(format!(
                                    "Lua API outside audited module: {relative}: {name}"
                                )
                                .into());
                            }
                            if !f.attrs.is_empty() {
                                return Err(format!(
                                    "unexpected Lua import attributes in {relative}: {name}"
                                )
                                .into());
                            }
                            imports.insert(name.clone());
                            edits.push((
                                offset(&file.text, &starts, f.span().start()),
                                false,
                                format!("#[link_name = \"uqm_lua_guard_{name}\"]\n    "),
                            ));
                        }
                    }
                }
            }
        }
        if edits.is_empty() {
            continue;
        }
        // Insert from the end. At the same boundary, close the previous expression
        // before opening a subsequent one; nested call spans keep their AST order.
        edits.sort_by_key(|(pos, close, _)| (*pos, !*close));
        let mut text = file.text;
        for (pos, _, addition) in edits.into_iter().rev() {
            text.insert_str(pos, &addition);
        }
        let transformed =
            syn::parse_file(&text).map_err(|e| format!("transformed {relative}: {e}"))?;
        validate_macros(&transformed).map_err(|e| format!("transformed {relative}: {e}"))?;
        reports.push(serde_json::json!({"file":relative,"functions":fn_reports}));
        changes.push((file.path, text));
    }
    if imports != apis {
        return Err(format!(
            "API inventory changed: missing {:?}",
            apis.difference(&imports).collect::<Vec<_>>()
        )
        .into());
    }
    let report = serde_json::json!({"guard_apis":apis,"changed_files":changes.len(),"checked_calls":count,"propagating_function_names":tainted.difference(&apis).collect::<Vec<_>>(),"files":reports});
    let output = serde_json::to_string_pretty(&report)? + "\n";
    if args[2] == "--apply" {
        for (path, text) in changes {
            fs::write(path, text)?;
        }
        fs::write(lib_path, lib)?;
        fs::write(marker, output)?;
    } else {
        fs::write(root.join("lua-propagation-preview.json"), output)?;
    }
    println!(
        "Lua propagation: {} files, {count} checked calls, {} APIs ({}).",
        reports.len(),
        apis.len(),
        args[2]
    );
    Ok(())
}
