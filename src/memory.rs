#[no_mangle]
pub unsafe extern "C" fn allocate(size: i32) -> *const u8 {
    let buffer = Vec::with_capacity(size as usize);
    let buffer = std::mem::ManuallyDrop::new(buffer);
    buffer.as_ptr() as *const u8
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(pointer: *mut u8, size: i32) {
    drop(Vec::from_raw_parts(pointer, size as usize, size as usize));
}

pub fn host_result(ok: bool, value: String) -> i32 {
    let mut result_vec = vec![0; 1 * 3];
    result_vec[0 * 3 + 2] = value.len() as i32;
    result_vec[0 * 3] = std::mem::ManuallyDrop::new(value).as_ptr() as i32;
    result_vec[0 * 3 + 1] = 31;
    let result_vec = std::mem::ManuallyDrop::new(result_vec);
    let mut rvec = vec![0 as u8; 9];
    rvec[0] = match ok {
        true => 0,
        false => 1,
    };
    rvec.splice(1..5, (result_vec.as_ptr() as i32).to_le_bytes());
    rvec.splice(5..9, (1 as i32).to_le_bytes());
    std::mem::ManuallyDrop::new(rvec).as_ptr() as i32
}
