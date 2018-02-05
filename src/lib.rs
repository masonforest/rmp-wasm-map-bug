extern crate rmp;
extern crate serde;
extern crate rmp_serde as rmps;

use serde::{Serialize};
use rmps::{Serializer};
use std::mem;

#[no_mangle]
pub fn pointer_to_five() -> *const u32 {
    let v = 5;
    let mut buf = Vec::new();
    v.serialize(&mut Serializer::new(&mut buf)).unwrap();
    let buf_ptr: *const u32 = buf.as_ptr() as *mut u32;
    mem::forget(buf);
    buf_ptr
}

#[no_mangle]
pub fn pointer_to_six() -> *const u32 {
    vec![0].into_iter().map(|x| x).collect::<Vec<u32>>();

    let v = 6;
    let mut buf = Vec::new();
    v.serialize(&mut Serializer::new(&mut buf)).unwrap();
    let buf_ptr: *const u32 = buf.as_ptr() as *mut u32;
    mem::forget(buf);
    buf_ptr
}
