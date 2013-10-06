extern mod wx;

use std::rt::start_on_main_thread;
use std::vec;
use std::libc::types::common::c95::c_void;
use std::libc::types::os::arch::c95::c_schar;

use wx::*;
use wx::native::*;


static nullptr: *mut c_void = 0 as *mut c_void;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    start_on_main_thread(argc, argv, crate_map, on_main)
}

#[fixed_stack_segment]
fn on_main() {
    unsafe {
        let closure = wxClosure_Create(wx_main as *mut c_void, nullptr);
        let args: ~[*i32] = ~[];
        ELJApp_InitializeC(closure, args.len() as i32, vec::raw::to_ptr(args) as *mut *mut c_schar);
    }
}

extern "C"
fn wx_main() {
    unsafe {
        // not mandatory
        // ELJApp_InitAllImageHandlers();
        let idAny = -1;
        let defaultFrameStyle = 536878656 | 4194304;
        do "Hello, wxRust!".to_c_str().with_ref |s| {
            let title = wxString_CreateUTF8(s as *u8);
            let frame = wxFrame_Create(nullptr, idAny, title as *mut c_void, -1, -1, -1, -1, defaultFrameStyle);
            println("OK");
            wxWindow_Show(frame);
//            wxWindow_Raise(frame);
        }
    }
}

