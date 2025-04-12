use regex_syntax::ast::{parse::Parser, Ast};
use arbitrary::{Arbitrary, Unstructured};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn process_ast(input_bytes: *const c_char) -> *mut c_char {
    // Convert C string to Rust string
    let c_str = unsafe { CStr::from_ptr(input_bytes) };
    let input = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(), // Return NULL on error
    };

    // Parse the input as an AST
    /*
    let parser = Parser::new();
    let ast = match parser.parse(input) {
        Ok(ast) => ast,
        Err(_) => return std::ptr::null_mut(), // Return NULL if parsing fails
    };

    // Serialize AST to a regex string
    let output_string = format!("{}", ast);
    */



    let mut unstructured = Unstructured::new(&(input.as_bytes()));

    // Generate a random AST from the bytes
    match Ast::arbitrary(&mut unstructured) {
        Ok(ast) => {
            // Serialize AST to a string
            let mut ast_string = format!("{}", ast);
            println!("Generated AST: {}", ast_string);
            ast_string.retain(|c| c != '\0');
            if ast_string.is_empty() {
                
                return std::ptr::null_mut();
            }
            // Convert Rust string to C string and return
            let c_string = CString::new(ast_string).expect("CString::new failed");
            return c_string.into_raw() // Return pointer to C string

        }
        Err(err) => {
            eprintln!("Failed to parse AST: {:?}", err);
        }
    }
    println!("Failed to do the thing...");
    std::ptr::null_mut()
    
}

#[unsafe(no_mangle)]
pub extern "C" fn free_c_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe { CString::from_raw(ptr) }; // Free memory
}

