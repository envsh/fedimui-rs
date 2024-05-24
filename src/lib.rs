
extern crate libc;
use libc::c_char;
use std::ffi::CStr;

use slint::slint;
use slint::ModelRc;
use slint::SharedString;
use slint::VecModel;
use slint::Model;
use std::rc::Rc;
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
 fn ffipxygocxrs_viastruct(cclen: usize, jscc : *const gostring) -> usize {

    0
 }

 #[no_mangle]
 pub extern "C" 
 fn ffipxygocxrs_viajson(cclen: usize, jscc : *const gostring) -> SJResult<()> {
    ffipxygocxrs(cclen, jscc)
 }

 #[no_mangle]
 pub extern "C" 
 fn ffipxygocxrs(cclen: usize, jscc : *const gostring) -> SJResult<()> {

    unsafe {
        // let s = std::str::from_utf8((*jscc).ptr);
        let xs = cstrfromu8ptr((*jscc).ptr, (*jscc).len);
    println!("hhhhh,cclen:{}, len2:{}, str:{}, len3:{},", cclen, (*jscc).len, xs, xs.len());
    let v: SJValue = serde_json::from_str(xs)?;

    let cmd  = &v["cmd"];
    // Access parts of the data by indexing with square brackets.
    println!("Please 呵呵呵 call {} at the number {}", v["cmd"], v["wtev"][0]);

        // why not work???
        // match v["cmd"] {
        //     "ping" => {

        //     }
        // }

        if cmd == "ping"{

        } else if cmd == "newmessage" {
            let uir = uintptrtouiwin(gvars.uip);
            let mut msg = MessageUist::default();
            // msg.body = "msgfromgo".into();
            let v  = v["args"][0].as_str().unwrap();
            msg.body = v.into();
            // uigetmsg2lstmdl(uir, msg);
            slint::invoke_from_event_loop(move || {
                pushmsg(msg);
            });
        }
    }


    Ok(())
 }

 fn pushmsg(msg : MessageUist) {
    unsafe {
    let uir = uintptrtouiwin(gvars.uip);
    uigetmsg2lstmdl(uir, msg);
    }
 }

#[no_mangle]
pub extern "C" 
fn ffipxygocxrs_dep(cclen: usize, jscc : *const gostring) -> SJResult<()> {
    if true {
        return  Ok(());
    }

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


#[cfg(target_os = "android")]
#[no_mangle]
fn android_mainrs(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    let _ = mainui();
}
// #[no_mangle]
// fn android_main(app: slint::android::AndroidApp) {
//     android_mainrs(app);
// }

// parse [[string]]
// fn uigetromlstmdl(ui : &AppWindow) -> ModelRc<ModelRc<slint::SharedString>> {
// fn uigetromlstmdl(ui : &AppWindow) -> &VecModel<SharedString> {
fn uigetcolumnsmdl(ui : &AppWindow) {
    let ui_handle = ui.as_weak();
    let ui2 = ui_handle.unwrap();
    let mdlrc = ui2.get_columns();
    let mdlany = mdlrc.as_any();
    let mdl2 = mdlany.downcast_ref::<VecModel<SharedString>>();
    let mdl = mdl2.expect("wekkkkkk");

    // mdl.push_back();
    eprintln!("columns cnt {}", mdl.row_count());
    // return mdl;
}
// parse [[string]]
fn uigetmsglstmdl(ui : &AppWindow) /*-> ModelRc<ModelRc<slint::SharedString>> */ {
    let ui_handle = ui.as_weak();
    let ui = ui_handle.unwrap();
    let mdlrc = ui.get_messages();
    let mdlany = mdlrc.as_any();
    let mdldcref = mdlany.downcast_ref::<VecModel<ModelRc<SharedString>>>();
    let mdl = mdldcref.expect("wokkkkkk");

    eprintln!("columns cnt {}", mdl.row_count());
    
    let mdl2rc = mdl.row_data(0).unwrap();
    let mdl2any = mdl2rc.as_any();
    let mdl2dcref = mdl2any.downcast_ref::<VecModel<SharedString>>();
    let mdl2 = mdl2dcref.expect("wokkkk");

    eprintln!("columns cnt {}", mdl2.row_count());
    // rlmdl.push_back();
    // return mdl;
}

fn uigetmsg2lstmdl(ui : &AppWindow, msg: MessageUist) /*-> &VecModel<MessageUist> */ {
    let ui_handle = ui.as_weak();
    let ui2 = ui_handle.unwrap();
    let mdlrc = ui2.get_messages2(); 
    let mdlany = mdlrc.as_any();
    // let x : ModelRc<MessageUist> = mdlrc;
    let mdl2 = mdlany.downcast_ref::<VecModel<MessageUist>>();
    let mdl = mdl2.expect("wekkkkkk");

    // mdl.push(v);
    let oc = mdl.row_count();
    // eprintln!("columns cnt {}", mdl.row_count());
    // let mut msg = MessageUist::default();
    // msg.body = "hehehhe".into();
    mdl.push(msg);
    // mdl.remove(); // VecModel.remove(),insert,push,
    eprintln!("msg2 cnt {} => {}", oc, mdl.row_count());
    // mdlrc.push(msg); // not work

    // mdl.as_weak();
    // return mdl;
}

fn uigetrom2lstmdl(ui : &AppWindow, msg: GroupchatUist) /*-> &VecModel<MessageUist> */ {
    let ui_handle = ui.as_weak();
    let ui2 = ui_handle.unwrap();
    let mdlrc = ui2.get_rooms2(); 
    let mdlany = mdlrc.as_any();
    // let x : ModelRc<MessageUist> = mdlrc;
    let mdl2 = mdlany.downcast_ref::<VecModel<GroupchatUist>>();
    let mdl = mdl2.expect("wekkkkkk");

    // mdl.push(v);
    let oc = mdl.row_count();
    // eprintln!("columns cnt {}", mdl.row_count());
    // let mut msg = MessageUist::default();
    // msg.body = "hehehhe".into();
    mdl.push(msg);
    // mdl.remove(); // VecModel.remove(),insert,push,
    // eprintln!("gc2 cnt {} => {}", oc, mdl.row_count());
    // mdlrc.push(msg); // not work

    // mdl.as_weak();
    // return mdl;
}

// #[derive(Sync)]
#[derive(Default)]
struct Somevars {
    // uip : &'a AppWindow,
    // uix : slint::Weak<AppWindow>,
    // mdl : Rc<i32>,
    uip : usize, // => &AppWindow
}

static mut gvars : Somevars = Somevars{uip: 0};

fn uiwintouintptr(uir : &AppWindow) -> usize {
    let n0 = uir as *const AppWindow;
    let n1 = n0 as *const libc::c_void;
    let n2 = n1 as usize;
    return n2;
}
fn uintptrtouiwin(v : usize) -> &'static AppWindow {
    let n1 = v as *const libc::c_void;
    let n0 = n1 as *const AppWindow as *mut AppWindow;
    
    // cast<&AppWindow>(n0)
    // n0 as &AppWindow
    let rv = unsafe { &*n0 } ;

    let nv = uiwintouintptr(rv);
    unsafe {
    eprintln!("nv {} {} ==? {}", nv == gvars.uip, nv, gvars.uip);
    }
    
    // let mut msg = MessageUist::default();
    // msg.body = "hehehhe".into();
    // uigetmsg2lstmdl(rv, msg);

    return rv;
}

fn dummy(v : usize, msg : MessageUist) { }


// fn mainui() -> Result<(), slint::PlatformError|> {
fn mainui() -> Result<(),slint::PlatformError> {
    
    let ui = AppWindow::new()?;
    let uir : &AppWindow = &ui;
    let uix : slint::Weak<AppWindow> = ui.as_weak();
    // ui.global::<varfn>().set_isandroid(1);
    // eprintln!("magic operation input: {}, os {}", 123, std::env::consts::OS);
    ui.global::<varfn>().set_isandroid(if std::env::consts::OS=="android" {1} else {0});
    // eprintln!("magic operation input: {}, os {}", 123, std::env::consts::OS);
    // ui.switchpage();
    ui.global::<varfn>().set_curpage("fffffff".into());

    let uih : usize;
    {
        let n0 = uir as *const AppWindow;
        let n1 = n0 as *const libc::c_void;
        let n2 = n1 as usize;
        uih = n2;
        unsafe { gvars.uip = n2; }
        eprintln!("uip integer {}", n2);
    }
    unsafe { eprintln!("uip integer {}", gvars.uip); }
    unsafe { uintptrtouiwin(gvars.uip); }

    // uigetcolumnsmdl(uip);
    // let msglstmdl = uigetmsglstmdl(uip);
    // uigetmsglstmdl(uip);
    let mut msg = MessageUist::default();
    msg.body = "hehehhe".into();
    uigetmsg2lstmdl(uir, msg);
    for i in 0..28 {
        let mut gco = GroupchatUist::default();
        gco.name = format!("gcname-{}",i).into();
        gco.author = format!("gcauth-{}", i).into();
        gco.gcid = format!("gciddddd-{}", i).into();
        gco.ctmstr = format!("ctmstr-{}", i).into();
        gco.lastmsg = format!("lastmsg-{}", i).into();
        uigetrom2lstmdl(uir, gco);    
    }
    let _ = slint::invoke_from_event_loop( move || {
        let mut msg = MessageUist::default();
        msg.body = "hehehheinvoke_from_event_loop".into();
        // uigetmsg2lstmdl(uir, msg);  // not works
        dummy(uih, msg); // works

    });

    // ui.global::<varfn>().on_isandroid(|| {
    //     eprintln!("magic operation input: {}, os {}", 123, std::env::consts::OS);
    //     if std::env::consts::OS == "android" {
    //         1
    //     }else {
    //         1
    //     }
    //     // eprintln!("magic operation input: {}", value);
    //     // value * 2
    // });

    let _ = ui.show();

    let _ = ui.run();
    Ok(())
}

// fn main(){}
