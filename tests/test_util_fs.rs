extern crate env_logger;
extern crate tempdir;

extern crate xflow;

use xflow::util::fs::{model_from_fs, model_to_fs};

use tempdir::TempDir;

mod helper;

#[test]
fn test_load_basic_project() {
    let _ = env_logger::init();
    if let Ok(model) = model_from_fs(&"resource/projects/basic") {
        assert_eq!(model.version, 1);
    } else {
        assert!(false);
    }
}

#[test]
fn test_model_to_and_from_fs() {
    let _ = env_logger::init();

    let model_a = model_from_fs(&"resource/projects/basic").unwrap();

    let root_b = TempDir::new("model_b");
    let root_b = root_b.ok().expect("Should have created a temp directory");

    let root_b_path = format!("{}", root_b.path().display());
    match model_to_fs(&model_a, &root_b_path) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }

    let model_b = model_from_fs(&root_b_path).unwrap();

    assert_eq!(model_a.id, model_b.id);
    assert_eq!(model_a.doc.xflows.len(), model_b.doc.xflows.len());
}