use std::path::Path;
use witx;

#[test]
fn validate_wasi_unstable_preview0() {
    witx::load(Path::new(
        "../../phases/unstable/witx/wasi_unstable_preview0.witx",
    ))
    .unwrap_or_else(|e| panic!("failed to parse: {}", e));
}

#[test]
fn validate_wasi_ephemeral_preview() {
    witx::load(Path::new(
        "../../phases/ephemeral/witx/wasi_ephemeral_preview.witx",
    ))
    .unwrap_or_else(|e| panic!("failed to parse: {}", e));
}

#[test]
fn validate_wasi_old_preview0() {
    witx::load(Path::new("../../phases/old/witx/wasi_unstable.witx"))
        .unwrap_or_else(|e| panic!("failed to parse: {}", e));
}

#[test]
fn render_roundtrip() {
    let doc = witx::load(Path::new(
        "../../phases/unstable/witx/wasi_unstable_preview0.witx",
    ))
    .unwrap_or_else(|e| panic!("failed to parse: {}", e));

    let back_to_sexprs = format!("{}", doc);

    let doc2 = witx::parse(&back_to_sexprs)
        .map_err(|e| e.report_with(&witx::MockFs::new(&[("-", &back_to_sexprs)])))
        .unwrap();

    // I'd just assert_eq, but when it fails the debug print is thousands of lines long and impossible
    // to figure out where they are unequal.
    if doc != doc2 {
        for type_ in doc.datatypes() {
            let type2 = doc2.datatype(&type_.name).expect("doc2 missing datatype");
            assert_eq!(type_, type2);
        }
        for mod_ in doc.modules() {
            let mod2 = doc2.module(&mod_.name).expect("doc2 missing module");
            for import in mod_.imports() {
                let import2 = mod2.import(&import.name).expect("mod2 missing import");
                assert_eq!(import, import2);
            }
            for func in mod_.funcs() {
                let func2 = mod2.func(&func.name).expect("mod2 missing func");
                assert_eq!(func, func2);
            }
        }
    }
    // This should be equivelant to the above, but just in case some code changes where it isnt:
    assert_eq!(doc, doc2)
}
