#[no_mangle]
pub fn pointer_to_five() -> *const u32 {
    let v = vec![5];
    v.as_ptr()
}

#[no_mangle]
pub fn pointer_to_six() -> *const u32 {
    // Note: if you comment the following line or remove the `map(|x| x)` this function returns the
    // correct result.
    let _unused = vec![0].into_iter().map(|x| x).collect::<Vec<u32>>();
    let v = vec![6];
    v.as_ptr()
}
