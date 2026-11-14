//! Deliberately constrained syntax scanner; never resolves server imports.
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
use syn::{
    Ident, Item, LitInt, LitStr, Token, Type, braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
};

type Result<T> = std::result::Result<T, String>;

struct Mode {
    name: String,
    ty: Option<Type>,
}

impl Parse for Mode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let ty = if input.peek(syn::token::Paren) {
            let inner;
            parenthesized!(inner in input);
            let ty = inner.parse()?;

            if !inner.is_empty() {
                return Err(inner.error("unexpected codec tokens"));
            }

            Some(ty)
        } else {
            None
        };

        Ok(Self {
            name: name.to_string(),
            ty,
        })
    }
}

/// One parsed `api_endpoint!` declaration.
pub struct Declaration {
    pub marker: Ident,
    handler: Ident,
    method: Ident,
    path: LitStr,
    params: Vec<(Ident, Type)>,
    query: Mode,
    body: Mode,
    response: Mode,
    statuses: Vec<u16>,
    auth: Ident,
    request_headers: Vec<LitStr>,
    response_headers: Vec<LitStr>,
}

fn key(input: ParseStream, expected: &str) -> syn::Result<()> {
    let name = input.parse::<Ident>()?;

    if name != expected {
        return Err(syn::Error::new(name.span(), format!("expected {expected}")));
    }

    input.parse::<Token![:]>()?;
    Ok(())
}

fn comma(input: ParseStream) -> syn::Result<()> {
    input.parse::<Token![,]>()?;
    Ok(())
}

fn strings(input: ParseStream) -> syn::Result<Vec<LitStr>> {
    let inner;
    bracketed!(inner in input);

    Ok(inner
        .parse_terminated(|input| input.parse::<LitStr>(), Token![,])?
        .into_iter()
        .collect())
}

impl Parse for Declaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![pub]>()?;
        let marker = input.parse()?;
        let inner;
        braced!(inner in input);
        key(&inner, "handler")?;
        let handler = inner.parse()?;
        comma(&inner)?;
        key(&inner, "method")?;
        let method = inner.parse()?;
        comma(&inner)?;
        key(&inner, "path")?;
        let path = inner.parse()?;
        comma(&inner)?;
        key(&inner, "path_params")?;
        let fields;
        braced!(fields in inner);
        let mut params = vec![];

        while !fields.is_empty() {
            let name = fields.parse()?;
            fields.parse::<Token![:]>()?;
            let ty = fields.parse()?;
            params.push((name, ty));
            if !fields.is_empty() {
                comma(&fields)?;
            }
        }

        comma(&inner)?;
        key(&inner, "query")?;
        let query = inner.parse()?;
        comma(&inner)?;
        key(&inner, "body")?;
        let body = inner.parse()?;
        comma(&inner)?;
        key(&inner, "response")?;
        let response = inner.parse()?;
        comma(&inner)?;
        key(&inner, "success")?;
        let list;
        bracketed!(list in inner);

        let statuses = list
            .parse_terminated(|input| input.parse::<LitInt>(), Token![,])?
            .iter()
            .map(|n| n.base10_parse())
            .collect::<syn::Result<Vec<u16>>>()?;

        comma(&inner)?;
        key(&inner, "auth")?;
        let auth = inner.parse()?;
        comma(&inner)?;
        key(&inner, "request_headers")?;
        let request_headers = strings(&inner)?;
        comma(&inner)?;
        key(&inner, "response_headers")?;
        let response_headers = strings(&inner)?;
        comma(&inner)?;

        if !inner.is_empty() || !input.is_empty() {
            return Err(inner.error("unexpected endpoint declaration tokens"));
        }

        Ok(Self {
            marker,
            handler,
            method,
            path,
            params,
            query,
            body,
            response,
            statuses,
            auth,
            request_headers,
            response_headers,
        })
    }
}

fn valid_type(ty: &Type, locals: &BTreeSet<String>) -> Result<()> {
    let rendered = ty.to_token_stream().to_string().replace(' ', "");
    let leaf = ["String", "u16", "u32", "u64", "i64", "bool", "f32", "Uuid"];

    if leaf.contains(&rendered.as_str()) || locals.contains(&rendered) {
        return Ok(());
    }

    let shared = [
        "AccountInfo",
        "DesktopSession",
        "RedeemTicket",
        "RegisterDevice",
        "DeviceCredentials",
        "StartSession",
        "SessionStarted",
        "EndRecording",
        "RecordingResult",
        "SetShareMode",
        "ShareState",
        "HistoryPage",
        "RecordingDetails",
        "ShareInfo",
        "RawChunk",
        "RawSnapshot",
    ];

    if shared
        .iter()
        .any(|name| rendered == format!("protocol::{name}"))
    {
        return Ok(());
    }

    if [
        "math::model::OwnedInputGame",
        "math::model::InputGame<'wire>",
        "math::model::OutputGame",
        "math::model::Realtime<'wire>",
        "math::riot::RiotRealtime<'wire>",
        "math::riot::FrameChunk<'wire>",
    ]
    .contains(&rendered.as_str())
    {
        return Ok(());
    }

    if let Type::Path(path) = ty
        && path.path.segments.len() == 1
        && let Some(segment) = &path.path.segments.get(0)
        && ["Option", "Vec"].contains(&segment.ident.to_string().as_str())
        && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
        && args.args.len() == 1
        && let syn::GenericArgument::Type(ty) = &args.args[0]
    {
        return valid_type(ty, locals);
    }

    Err(format!("unsupported contract type {rendered}"))
}

impl Mode {
    fn wire_type(&self) -> TokenStream {
        self.ty
            .as_ref()
            .unwrap()
            .to_token_stream()
            .to_string()
            .replace("'wire", "'a")
            .parse()
            .unwrap()
    }

    fn check_mode(&self, allowed: &[&str], locals: &BTreeSet<String>) -> Result<()> {
        if !allowed.contains(&self.name.as_str()) {
            return Err(format!("unsupported codec {}", self.name));
        }

        if ["none", "empty", "browser"].contains(&self.name.as_str()) {
            if self.ty.is_some() {
                return Err("bodyless codec cannot have a type".into());
            }
        } else {
            valid_type(self.ty.as_ref().ok_or("missing wire type")?, locals)?;
        }

        Ok(())
    }
}

impl Declaration {
    /// Renders the endpoint marker, its `Endpoint` impl and its typed inputs/outputs.
    fn render(&self, locals: &BTreeSet<String>) -> Result<TokenStream> {
        self.query.check_mode(&["none", "form"], locals)?;
        self.body.check_mode(
            &["none", "bincode", "borrowed_bincode", "brotli_bincode"],
            locals,
        )?;

        self.response.check_mode(
            &[
                "envelope",
                "borrowed_envelope",
                "borrowed_bincode",
                "brotli_bincode",
                "live",
                "empty",
                "browser",
            ],
            locals,
        )?;

        let marker = &self.marker;
        let method = &self.method;

        if !["GET", "POST", "PUT", "PATCH"].contains(&method.to_string().as_str()) {
            return Err("unsupported method".into());
        }

        let path = self.path.value();

        if !path.starts_with("/api/") || path.contains(['?', '#']) {
            return Err("path must be a full /api path".into());
        }

        let mut placeholders = BTreeSet::new();

        for segment in path.split('/') {
            if segment.starts_with('{') && segment.ends_with('}') {
                let name = &segment[1..segment.len() - 1];

                if !placeholders.insert(name.to_string()) {
                    return Err("duplicate path placeholder".into());
                }
            } else if segment.contains(['{', '}']) {
                return Err("placeholder must occupy a whole segment".into());
            }
        }

        let names = self
            .params
            .iter()
            .map(|(name, _)| name.to_string())
            .collect::<BTreeSet<_>>();

        if names.len() != self.params.len() || names != placeholders {
            return Err("path placeholder/field mismatch".into());
        }

        for (_, ty) in &self.params {
            valid_type(ty, locals)?;
            let ty = ty.to_token_stream().to_string();

            if !["String", "Uuid", "u32", "u64", "u16"].contains(&ty.as_str()) {
                return Err("unsupported path field type".into());
            }
        }

        let path_ty = format_ident!("{}Path", marker);
        let (path_definition, path_type, path_expr) = if self.params.is_empty() {
            (quote! {}, quote!(NoPath), quote!(String::from(#path)))
        } else {
            let fields = self.params.iter().map(|(n, t)| quote!(pub #n:#t));
            let pieces = path.split('/').enumerate().map(|(i, s)| {
                let prefix = if i == 0 { "" } else { "/" };

                if s.starts_with('{') {
                    let name = format_ident!("{}", &s[1..s.len() - 1]);
                    quote! {
                        out.push_str(#prefix);
                        out.push_str(&encode_segment(&path.#name));
                    }
                } else {
                    quote! {
                        out.push_str(#prefix);
                        out.push_str(#s);
                    }
                }
            });

            (
                quote!(
                    #[derive(Clone, Debug)]
                    pub struct #path_ty {
                        #(#fields,)*
                    }
                ),
                quote!(#path_ty),
                quote!({
                    let mut out = String::new();
                    #(#pieces)*
                    out
                }),
            )
        };

        let (query_ty, query_expr) = if self.query.name == "none" {
            (quote!(NoQuery), quote!(Ok(String::new())))
        } else {
            (
                self.query.wire_type(),
                quote!(serde_urlencoded::to_string(query).map_err(|_| ContractError::Query)),
            )
        };

        let (body_ty, body_expr, encoding) = if self.body.name == "none" {
            (
                quote!(NoBody),
                quote!(Ok(Vec::new())),
                quote!(BodyEncoding::None),
            )
        } else {
            (
                self.body.wire_type(),
                quote!(encode_bincode(body)),
                if self.body.name == "brotli_bincode" {
                    quote!(BodyEncoding::BrotliBincode)
                } else {
                    quote!(BodyEncoding::Bincode)
                },
            )
        };

        let statuses = &self.statuses;

        if statuses.is_empty()
            || statuses.iter().any(|s| {
                !(200..400).contains(s)
                    && !(self.response.name == "browser" && (400..600).contains(s))
            })
            || statuses.iter().collect::<BTreeSet<_>>().len() != statuses.len()
        {
            return Err("invalid success statuses".into());
        }

        if statuses.contains(&204) && !["empty", "live"].contains(&self.response.name.as_str()) {
            return Err("204 requires empty or live response".into());
        }

        let (output_ty, mode, decode) = match self.response.name.as_str() {
            "empty" => (
                quote!(Acknowledgement),
                quote!(ResponseMode::Empty),
                quote! {
                    if !bytes.is_empty() {
                        return Err(ContractError::UnexpectedBody)
                    }

                    Ok(Acknowledgement { status })
                },
            ),
            "browser" => (
                quote!(BrowserResponse<'a>),
                quote!(ResponseMode::Browser),
                quote! {
                    let location = if (300..400).contains(&status) {
                        Some(headers.required("Location")?.to_owned())
                    } else {
                        None
                    };

                    Ok(BrowserResponse {
                        status,
                        body: bytes,
                        location
                    })
                },
            ),
            "envelope" | "borrowed_envelope" => {
                let ty = self.response.wire_type();
                (
                    ty.clone(),
                    if self.response.name == "envelope" {
                        quote!(ResponseMode::Envelope)
                    } else {
                        quote!(ResponseMode::BorrowedEnvelope)
                    },
                    quote! {
                        let response = protocol::ApiResponse::<#ty>::borrow_decode(bytes,status)?;
                        response.result.map_err(|error| ContractError::Remote { status, error })
                    },
                )
            }
            "live" => {
                let ty = self.response.wire_type();
                (
                    quote!(Option<LiveFrame<'a>>),
                    quote!(ResponseMode::Bincode),
                    quote! {
                        if status == 204 {
                            if !bytes.is_empty() {
                                return Err(ContractError::UnexpectedBody)
                            }

                            return Ok(None)
                        }

                        let seq = headers.required("X-Frame-Seq")?
                            .parse()
                            .map_err(|_| ContractError::Header("X-Frame-Seq".into()))?;

                        let captured_at = headers.required("X-Captured-At")?
                            .parse()
                            .map_err(|_| ContractError::Header("X-Captured-At".into()))?;

                        let game_time: f32 = headers.required("X-Game-Time")?
                            .parse()
                            .map_err(|_| ContractError::Header("X-Game-Time".into()))?;

                        if !game_time.is_finite(){
                            return Err(ContractError::Header("X-Game-Time".into()))
                        }

                        Ok(
                            Some(
                                LiveFrame {
                                    seq,
                                    captured_at,
                                    game_time,
                                    frame: decode_bincode::<#ty>(bytes)?
                                }
                            )
                        )
                    },
                )
            }
            _ => {
                let ty = self.response.wire_type();
                (
                    ty.clone(),
                    if self.response.name == "brotli_bincode" {
                        quote!(ResponseMode::BrotliBincode)
                    } else {
                        quote!(ResponseMode::Bincode)
                    },
                    quote!(decode_bincode::<#ty>(bytes)),
                )
            }
        };

        let request_headers = &self.request_headers;
        let response_headers = &self.response_headers;

        let (headers_ty, headers_expr) = if request_headers.is_empty() {
            (quote!(NoHeaders), quote!(Ok(Vec::new())))
        } else if request_headers.len() == 1 && request_headers[0].value() == "X-Content-Sha256" {
            (
                quote!(ContentHash),
                quote! {
                    if headers.sha256.len() != 64
                        || !headers.sha256.bytes().all(|b| b.is_ascii_hexdigit())
                    {
                        return Err(ContractError::Header("X-Content-Sha256".into()))
                    }

                    Ok(vec![(String::from("X-Content-Sha256"), headers.sha256.clone())])
                },
            )
        } else {
            return Err("unsupported typed request header policy".into());
        };

        let auth = match self.auth.to_string().as_str() {
            "anonymous" => quote!(Auth::Anonymous),
            "device" => quote!(Auth::Device),
            "account" => quote!(Auth::Account),
            "owner" => quote!(Auth::Owner),
            "capability" => quote!(Auth::Capability),
            "browser" => quote!(Auth::Browser),
            _ => return Err("unknown auth policy".into()),
        };

        let precheck = if self.response.name == "browser" {
            quote! {
                if !Self::SUCCESS.contains(&status) && !(400..600).contains(&status) {
                    return Err(ContractError::UnexpectedStatus(status))
                }
            }
        } else {
            quote! {
                check_status::<Self>(status,bytes)?;
            }
        };

        let literal = &self.path;

        Ok(quote! {
            #path_definition
            #[derive(Clone,Copy,Debug)]
            pub struct #marker;

            #[allow(unused_variables)]
            impl Endpoint for #marker {
                type Path = #path_type;
                type Query = #query_ty;
                type Headers = #headers_ty;
                type Body<'a> = #body_ty;
                type Output<'a> = #output_ty;

                const METHOD: Method = Method::#method;
                const PATH: &'static str = #literal;
                const SUCCESS: &'static[u16] = &[#(#statuses),*];
                const RESPONSE_MODE: ResponseMode = #mode;
                const BODY_ENCODING: BodyEncoding = #encoding;
                const AUTH: Auth = #auth;
                const REQUEST_HEADERS: &'static [&'static str] = &[#(#request_headers),*];
                const RESPONSE_HEADERS: &'static[&'static str] = &[#(#response_headers),*];

                #[allow(clippy::single_char_add_str, clippy::needless_borrows_for_generic_args)]
                fn path(path: &Self::Path) -> String {
                    #path_expr
                }

                fn query(query: &Self::Query) -> Result<String, ContractError> {
                    #query_expr
                }

                fn headers(headers: &Self::Headers) -> Result<Vec<(String, String)>, ContractError> {
                    #headers_expr
                }

                fn encode_body(body: &Self::Body<'_>) -> Result<Vec<u8>, ContractError> {
                    #body_expr
                }

                fn decode<'a>(
                    status: u16,
                    headers: &ResponseHeaders,
                    bytes: &'a[u8]
                ) -> Result<Self::Output<'a>, ContractError> {
                    #precheck
                    #decode
                }
            }
        })
    }
}

/// Generates a standalone Rust module from explicit macros in route source files.
pub fn generate_sources(sources: &[(&str, &str)]) -> Result<String> {
    let mut inputs = sources.to_vec();

    inputs.sort_by_key(|(path, _)| *path);

    let mut declarations = Vec::new();
    let mut definitions = BTreeMap::new();

    for (filename, source) in inputs {
        let file = syn::parse_file(source).map_err(|e| format!("{filename}: {e}"))?;
        let mut paths = BTreeSet::new();

        for item in file.items {
            let Item::Macro(item) = item else { continue };
            let name = item.mac.path.segments.last().unwrap().ident.to_string();

            if name == "api_endpoint" {
                let declaration = syn::parse2::<Declaration>(item.mac.tokens).map_err(|e| {
                    format!(
                        "{filename}:{}:{}: {e}",
                        e.span().start().line,
                        e.span().start().column + 1
                    )
                })?;

                paths.insert(declaration.path.value());
                declarations.push((filename, declaration));
            } else if name == "api_contract" {
                let definition = syn::parse2::<syn::ItemStruct>(item.mac.tokens)
                    .map_err(|e| format!("{filename}: {e}"))?;

                if !matches!(definition.vis, syn::Visibility::Public(_))
                    || !definition.generics.params.is_empty()
                    || !definition.attrs.is_empty()
                {
                    return Err(format!(
                        "{filename}: contract structs must be public, non-generic, without outer attributes"
                    ));
                }

                if definitions
                    .insert(definition.ident.to_string(), definition)
                    .is_some()
                {
                    return Err(format!("{filename}: duplicate contract type"));
                }
            }
        }

        if paths.len() > 1 {
            return Err(format!("{filename}: one URL path per route file required"));
        }
    }

    let locals = definitions.keys().cloned().collect::<BTreeSet<_>>();
    let mut output = quote! {
        // This module deliberately has no server dependency.
        use crate::protocol;
        use crate::protocol::endpoint::*;
        use std::{string::String, vec::Vec};
        use uuid::Uuid;

        #[derive(Debug)]
        pub struct LiveFrame<'a> {
            pub seq: u64,
            pub captured_at: i64,
            pub game_time: f32,
            pub frame: math::riot::RiotRealtime<'a>,
        }

        #[derive(Debug)]
        pub struct BrowserResponse<'a> {
            pub status: u16,
            pub body: &'a [u8],
            pub location: Option<String>
        }
    };

    for (_, mut definition) in definitions {
        let syn::Fields::Named(fields) = &mut definition.fields else {
            return Err("only named contract fields are supported".into());
        };

        for field in &mut fields.named {
            if !matches!(field.vis, syn::Visibility::Public(_))
                || field.attrs.iter().any(|a| {
                    !a.path().is_ident("serde")
                        || !a
                            .parse_args::<Ident>()
                            .is_ok_and(|argument| argument == "default")
                })
            {
                return Err(
                    "contract fields must be public with only #[serde(default)] attributes".into(),
                );
            }

            valid_type(&field.ty, &locals)?;
            field.attrs.push(syn::parse_quote!(#[bincode(with_serde)]));
        }

        output.extend(quote! { #[derive(Clone,Debug,Default,serde::Serialize,serde::Deserialize,bincode::Encode,bincode::Decode)] #definition });
    }

    let mut keys = BTreeSet::new();
    let mut markers = BTreeSet::new();

    declarations.sort_by_key(|(_, d)| (d.path.value(), d.method.to_string()));

    for (filename, d) in &declarations {
        if !keys.insert((d.method.to_string(), d.path.value()))
            || !markers.insert(d.marker.to_string())
        {
            return Err(format!("{filename}: duplicate route or marker"));
        }

        if d.handler.to_string().is_empty() {
            return Err("missing handler".into());
        }

        output.extend(d.render(&locals).map_err(|e| {
            format!(
                "{filename}:{}:{}: {e}",
                d.marker.span().start().line,
                d.marker.span().start().column + 1
            )
        })?);
    }

    let count = declarations.len();
    let names = declarations.iter().map(|(_, d)| {
        let name = &d.marker;
        quote! {
            (#name::METHOD, #name::PATH)
        }
    });

    output.extend(quote! {
        pub const ROUTE_COUNT: usize = #count;
        pub const ROUTES: &[(Method, &str)] = &[#(#names),*];
    });

    let syntax = syn::parse2(output).map_err(|e| format!("generated source: {e}"))?;

    Ok(format!(
        "// @generated by api_codegen; do not edit.\n{}",
        prettyplease::unparse(&syntax)
    ))
}

/// Server route declarations, relative to the workspace root.
pub const ROUTES_DIR: &str = "crates/server/src/server/routes";
/// Generated contracts, relative to the workspace root.
pub const GENERATED_FILE: &str = "crates/api/src/generated.rs";

fn collect(path: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let path = entry?.path();

        if path.is_dir() {
            collect(&path, files)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }

    Ok(())
}

/// Reads every `.rs` file under `routes` (recursively, in a stable order) and
/// generates the contracts they declare.
pub fn generate_from_routes(routes: &Path) -> Result<String> {
    let mut files = vec![];

    collect(routes, &mut files).map_err(|e| format!("{}: {e}", routes.display()))?;
    files.sort();

    let sources = files
        .iter()
        .map(|path| {
            let name = path
                .strip_prefix(routes)
                .map_err(|e| e.to_string())?
                .to_string_lossy()
                .into_owned();
            let source =
                fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;

            Ok((name, source))
        })
        .collect::<Result<Vec<_>>>()?;

    let refs = sources
        .iter()
        .map(|(name, source)| (name.as_str(), source.as_str()))
        .collect::<Vec<_>>();

    generate_sources(&refs)
}

/// Regenerates `output` from `routes`, writing only when its contents change so
/// an unchanged file keeps its timestamp. Returns whether the file was written.
pub fn write_if_changed(routes: &Path, output: &Path) -> Result<bool> {
    let generated = generate_from_routes(routes)?;

    if fs::read_to_string(output).is_ok_and(|current| current == generated) {
        return Ok(false);
    }

    fs::write(output, generated).map_err(|e| format!("{}: {e}", output.display()))?;
    Ok(true)
}
