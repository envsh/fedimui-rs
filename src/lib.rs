#[no_mangle]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

extern crate libc;
use libc::c_char;
use std::ffi::CStr;

use serde_json::{Result as SJResult, Value as SJValue};
// use serde_json::Value;

slint::include_modules!();

// linktime error
// extern "C" { fn ffipxyrscxgo(); }

#[repr(C)]
pub struct gostring {
    ptr: *const u8,
    len: usize,
}

pub fn cstrfromu8ptr(ptr :*const u8, len: usize) -> &'static str {
    unsafe {
    let vcc  = core::slice::from_raw_parts(ptr, len+1);
    // println!("xxxx {}", vcc.len());
    let x = CStr::from_bytes_with_nul_unchecked(vcc);
    let xs = x.to_str().to_owned().unwrap();
    
    return xs
    }
 }
 pub fn cstrfromu8ptr2(ptr :*const u8, len: usize) -> String {
    unsafe {
    let xs = cstrfromu8ptr(ptr, len);
    
    return xs.to_string()
    }
 }

#[no_mangle]
pub extern "C" 
fn ffipxygocxrs(cclen: usize, jscc : *const gostring) -> SJResult<()> {
    unsafe {
        // let s = std::str::from_utf8((*jscc).ptr);
        let xs = cstrfromu8ptr((*jscc).ptr, (*jscc).len);
    println!("hhhhh,cclen:{}, len2:{}, str:{}, len3:{},", cclen, (*jscc).len, xs, xs.len());
    let v: SJValue = serde_json::from_str(xs)?;

    let cmd = &v["cmd"];
    // Access parts of the data by indexing with square brackets.
    println!("Please 呵呵呵 call {} at the number {}", v["cmd"], v["wtev"][0]);

    }
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: SJValue = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please 呵呵呵 call {} at the number {}", v["name"], v["phones"][0]);

    unsafe {
        // let f = ffipxyrscxgo as fn();
        // f();
        ffipxyrscxgo(data);
    }

    Ok(())
}

fn dummyfff(v:&str) {}
static mut ffipxyrscxgo : fn(v:&str) = dummyfff;

#[no_mangle]
pub extern "C"
fn ffipxyrscxgoset(fnptr : fn(v:&str) ) {
    unsafe { ffipxyrscxgo = fnptr; }
    // ffipxyrscxgo = fnptr;
}

// #[no_mangle]
// pub extern "C"
// fn ffipxyrscxgo() {

// }


#[no_mangle]
pub fn runui() { 
    let res = mainui();
    // println!("{}",res.unwrap());
}

// fn mainui() -> Result<(), slint::PlatformError|> {
fn mainui() -> Result<(),slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 2);
            let cnt = ui.get_counter();
            let cntstr = cnt .to_string();
            let pfx : slint::SharedString = "nammme".into();
            ui.set_fdname( pfx + &cntstr);
        }
    });
    ui.on_justrunafunc({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            println!("ffffff");
            // ui.set_counter(ui.get_counter() + 2);
            // let cnt = ui.get_counter();
            // let cntstr = cnt .to_string();
            // let pfx : slint::SharedString = "nammme".into();
            // ui.set_fdname( pfx + &cntstr);
        }
        // println("ffff");
    });

    ui.run();
    Ok(())
}

// fn main(){}
