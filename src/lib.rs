use regex_syntax::ast::{parse::Parser, Ast};
use arbitrary::{Arbitrary, Unstructured};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use hex::encode;

/*
#[unsafe(no_mangle)]
pub extern "C" fn process_ast(input_bytes: *const c_char) -> *mut c_char {
    // Convert C string to Rust string
    let c_str = unsafe { CStr::from_ptr(input_bytes) };



*/



/*
#[repr(C)]
pub struct Buffer {
    ptr: *mut u8,
    size: usize,
}
*/



#[unsafe(no_mangle)]
pub extern "C" fn process_ast(input_bytes: *const u8, input_size: usize) -> *mut c_char {
    if input_bytes.is_null() || input_size == 0 {
        return std::ptr::null_mut(); // Return NULL if input is invalid
    }

    // Convert input bytes to a Rust slice
    let input_slice = unsafe { std::slice::from_raw_parts(input_bytes, input_size) };
    /*
    let input_thing = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(), // Return NULL on error
    };
    */

    //input_slice = input_slice.as_byte_slice();

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



    // let mut unstructured = Unstructured::new(&(input.as_bytes()));

    // input_slice.as_byte_slice();

    //let hex_string = encode(&input_slice);
    //println!("Here are the bytes stuff: {}", hex_string);

    let mut unstructured = Unstructured::new(&input_slice);

    // Generate a random AST from the bytes
    match Ast::arbitrary_take_rest(unstructured) {
        Ok(ast) => {
            // Serialize AST to a string
            let mut ast_string = format!("{}", ast);
            //println!("Generated AST: {}", ast_string);
            ast_string.retain(|c| c != '\0');
            if ast_string.is_empty() {

                return std::ptr::null_mut();
            }
            // Convert Rust string to C string and return
            let c_string = CString::new(ast_string).expect("fail"); // .expect("CString::new failed");
            return c_string.into_raw() // Return pointer to C string

        }
        Err(err) => {
            //eprintln!("Failed to parse AST: {:?}", err);
        }
    }
    //println!("Failed to do the thing...");
    std::ptr::null_mut()
    
}

#[unsafe(no_mangle)]
pub extern "C" fn free_c_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe { CString::from_raw(ptr) }; // Free memory
}

