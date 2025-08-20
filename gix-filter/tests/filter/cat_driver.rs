use std::io::Read;

use bstr::ByteSlice;
use gix_filter::{
    driver::{apply, Operation},
    Driver,
};

#[test]
fn cat_filter_does_not_hang_with_large_data() {
    let mut state = gix_filter::driver::State::default();
    
    // Create a driver that uses `cat` as both clean and smudge filters  
    let driver = Driver {
        name: "cat".into(),
        clean: Some("cat".into()),
        smudge: Some("cat".into()),
        process: None,
        required: true,
    };
    
    // Create data large enough to potentially cause deadlock (larger than typical pipe buffer)
    let large_data = "A".repeat(100_000);
    
    let context = apply::Context {
        rela_path: "test.txt".into(),
        ref_name: None,
        treeish: None,
        blob: None,
    };
    
    // This should not hang with the fix
    let mut filtered = state
        .apply(&driver, &mut large_data.as_bytes(), Operation::Smudge, context)
        .expect("filter should not hang")
        .expect("filter should be applied");
        
    let mut result = Vec::new();
    filtered.read_to_end(&mut result).expect("should read result");
    
    // Verify the output is correct (cat should return the input unchanged)
    assert_eq!(result.as_bstr(), large_data.as_bytes().as_bstr());
}

#[test]
fn cat_filter_works_with_small_data() {
    let mut state = gix_filter::driver::State::default();
    
    let driver = Driver {
        name: "cat".into(),
        clean: Some("cat".into()),
        smudge: Some("cat".into()),
        process: None,
        required: true,
    };
    
    let input = "hello world\n";
    
    let context = apply::Context {
        rela_path: "test.txt".into(),
        ref_name: None,
        treeish: None,
        blob: None,
    };
    
    let mut filtered = state
        .apply(&driver, &mut input.as_bytes(), Operation::Clean, context)
        .expect("filter should work")
        .expect("filter should be applied");
        
    let mut result = Vec::new();
    filtered.read_to_end(&mut result).expect("should read result");
    
    assert_eq!(result.as_bstr(), input.as_bytes().as_bstr());
}