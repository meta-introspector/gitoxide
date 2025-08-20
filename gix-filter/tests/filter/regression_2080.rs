use std::io::Read;
use gix_filter::{
    driver::{apply, Operation},
    Driver,
};

/// This test verifies that filters with commands like 'cat' don't hang even with large data.
/// It demonstrates the fix for issue #2080.
#[test]
fn large_data_through_cat_filter_does_not_hang() {
    let mut state = gix_filter::driver::State::default();
    
    // Create a driver that uses `cat` - this used to cause hangs
    let driver = Driver {
        name: "cat".into(),
        clean: Some("cat".into()),
        smudge: Some("cat".into()),
        process: None,
        required: true,
    };
    
    // Test with various sizes to ensure robustness
    for size in [1000, 10_000, 100_000] {
        let data = "A".repeat(size);
        
        let context = apply::Context {
            rela_path: "test.txt".into(),
            ref_name: None,
            treeish: None,
            blob: None,
        };
        
        // Test both clean and smudge operations
        for operation in [Operation::Clean, Operation::Smudge] {
            let mut filtered = state
                .apply(&driver, &mut data.as_bytes(), operation, context)
                .expect("filter should not hang or fail")
                .expect("filter should be applied");
                
            let mut result = Vec::new();
            filtered.read_to_end(&mut result).expect("should read result");
            
            // cat should return input unchanged
            assert_eq!(
                result, 
                data.as_bytes(), 
                "cat filter should return input unchanged for {} bytes", 
                size
            );
        }
    }
}