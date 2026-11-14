use api_codegen::{Declaration, generate_sources};
use syn::{Item, Token};
const ACCOUNT: &str = r#"api_endpoint! { pub GetAccount { handler: account, method: GET, path: "/api/account", path_params: {}, query: none, body: none, response: envelope(protocol::AccountInfo), success: [200], auth: account, request_headers: [], response_headers: ["Cache-Control"], } }"#;
#[test]
fn deterministic_generation() {
    let one = generate_sources(&[("account.rs", ACCOUNT)]).unwrap();
    assert_eq!(one, generate_sources(&[("account.rs", ACCOUNT)]).unwrap());
    assert!(one.contains("impl Endpoint for GetAccount"));
}
#[test]
fn rejects_duplicate_routes() {
    assert!(generate_sources(&[("a.rs", ACCOUNT), ("b.rs", ACCOUNT)]).is_err());
}
#[test]
fn rejects_placeholder_mismatch() {
    assert!(
        generate_sources(&[(
            "a.rs",
            &ACCOUNT.replace("/api/account", "/api/account/{id}")
        )])
        .is_err()
    );
}
#[test]
fn rejects_unknown_types_and_codecs() {
    for source in [
        ACCOUNT.replace("protocol::AccountInfo", "server::Secret"),
        ACCOUNT.replace("envelope(", "json("),
    ] {
        assert!(generate_sources(&[("a.rs", &source)]).is_err());
    }
}
#[test]
fn rejects_multiple_paths_per_file() {
    let source = format!(
        "{ACCOUNT}\n{}",
        ACCOUNT
            .replace("GetAccount", "Another")
            .replace("/api/account", "/api/another")
    );
    assert!(generate_sources(&[("a.rs", &source)]).is_err());
}
#[test]
fn rejects_unsupported_contract_field_attributes() {
    let source = format!(
        "{ACCOUNT} api_contract! {{ pub struct Input {{ #[serde(with = \"server::codec\")] pub value: String }} }}"
    );
    assert!(generate_sources(&[("account.rs", &source)]).is_err());
}

#[test]
fn registered_routes_match_declared_inventory() {
    use syn::visit::Visit;
    #[derive(Default)]
    struct Registrations(Vec<String>);
    impl<'ast> Visit<'ast> for Registrations {
        fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
            if let syn::Expr::Path(path) = &*call.func {
                let segments = &path.path.segments;
                if segments.len() == 3 && segments[0].ident == "api" && segments[2].ident == "route"
                {
                    self.0.push(segments[1].ident.to_string());
                }
            }
            syn::visit::visit_expr_call(self, call);
        }
        // guarded! contains route expressions inside macro tokens.
        fn visit_macro(&mut self, item: &'ast syn::Macro) {
            if item.path.is_ident("guarded") {
                let args = item
                    .parse_body_with(
                        syn::punctuated::Punctuated::<syn::Expr, Token![,]>::parse_terminated,
                    )
                    .unwrap();
                for arg in &args {
                    self.visit_expr(arg);
                }
            }
        }
    }
    fn declarations(path: &std::path::Path, markers: &mut Vec<String>) {
        for entry in std::fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                declarations(&path, markers);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                for item in syn::parse_file(&std::fs::read_to_string(path).unwrap())
                    .unwrap()
                    .items
                {
                    if let Item::Macro(item) = item
                        && item.mac.path.is_ident("api_endpoint")
                    {
                        markers.push(
                            syn::parse2::<Declaration>(item.mac.tokens)
                                .unwrap()
                                .marker
                                .to_string(),
                        );
                    }
                }
            }
        }
    }
    let routes =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../server/src/server/routes");
    let source = std::fs::read_to_string(routes.join("mod.rs")).unwrap();
    let mut registered = Registrations::default();
    registered.visit_file(&syn::parse_file(&source).unwrap());
    let mut declared = vec![];
    declarations(&routes, &mut declared);
    registered.0.sort();
    declared.sort();
    assert!(!declared.is_empty());
    assert_eq!(
        registered.0, declared,
        "each declared endpoint must be registered exactly once"
    );
}
