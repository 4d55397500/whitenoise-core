extern crate yarrow_validator;
use yarrow_validator::yarrow;

mod base;
mod utilities;
mod components;
mod algorithms;

use ndarray::prelude::*;

extern crate libc;
use libc::c_char;

use crate::base::execute_graph;

// useful tutorial for proto over ffi here:
// https://github.com/mozilla/application-services/blob/master/docs/howtos/passing-protobuf-data-over-ffi.md
unsafe fn get_buffer<'a>(data: *const u8, len: i32) -> &'a [u8] {
    assert!(len >= 0, "Bad buffer len: {}", len);
    if len == 0 {
        // This will still fail, but as a bad protobuf format.
        &[]
    } else {
        assert!(!data.is_null(), "Unexpected null data pointer");
        std::slice::from_raw_parts(data, len as usize)
    }
}

#[repr(C)]
#[allow(dead_code)]
struct ByteBuffer {
    len: i64,
    data: *mut u8,
}

#[no_mangle]
pub extern "C" fn release(
    dataset_ptr: *const u8, dataset_length: i32,
    analysis_ptr: *const u8, analysis_length: i32,
    release_ptr: *const u8, release_length: i32
) -> ffi_support::ByteBuffer {

    let dataset_buffer = unsafe {get_buffer(dataset_ptr, dataset_length)};
    let dataset: yarrow::Dataset = prost::Message::decode(dataset_buffer).unwrap();

    let analysis_buffer = unsafe {get_buffer(analysis_ptr, analysis_length)};
    let analysis: yarrow::Analysis = prost::Message::decode(analysis_buffer).unwrap();

    let release_buffer = unsafe {get_buffer(release_ptr, release_length)};
    let release: yarrow::Release = prost::Message::decode(release_buffer).unwrap();

    let response_release = execute_graph(&analysis, &release, &dataset);
    
    let mut out_buffer = Vec::new();
    match prost::Message::encode(&response_release, &mut out_buffer) {
        Ok(_t) => ffi_support::ByteBuffer::from_vec(out_buffer),
        Err(error) => {
            println!("Error encoding response protobuf.");
            println!("{:?}", error);
            ffi_support::ByteBuffer::new_with_size(0)
        }
    }
}

use std;
use std::ffi::CString;

#[no_mangle]
pub extern fn string_from_rust() -> *const std::os::raw::c_char {
    let s = CString::new("Hello ピカチュウ !").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

#[no_mangle]
pub extern fn test_sample_uniform(samples_buf: *mut f64, n_samples: u32) {

    let samples: Vec<f64> = (0..n_samples)
        .map(|_x| utilities::sample_uniform(0., 1.)).collect();

    unsafe {
        std::slice::from_raw_parts_mut(samples_buf, n_samples as usize)
            .copy_from_slice(&samples);
    }
}
#[no_mangle]
pub extern fn test_sample_laplace(samples_buf: *mut f64, n_samples: u32) {

    let samples: Vec<f64> = (0..n_samples)
        .map(|_x| utilities::sample_laplace(0., 1.)).collect();

    unsafe {
        std::slice::from_raw_parts_mut(samples_buf, n_samples as usize)
            .copy_from_slice(&samples);
    }
}

#[no_mangle]
pub extern fn test_ndarray() {

    // multiply with scalar
    let temp = Array::from_elem((2, 2, 3, 4), 2.2) * 2.3;
    println!("{:?}", temp);

    println!("FIRST: {:?}", temp.into_dyn().first());

    // multiply with zero dimensional array
//    let temp = Array::from_elem((2,), 2) * Array::from_elem((), 3);
//    println!("{:?}", temp);
//
//    // string datatype
//    let temp = Array::from_elem((1, 2), "test");
//    println!("{:?}", temp);

    //
//    let temp = Array::from_elem((1, 2), "test") * 2;
//    println!("{:?}", temp);
}

//ffi_support::implement_into_ffi_by_protobuf!(yarrow::Release);
ffi_support::define_bytebuffer_destructor!(dp_runtime_destroy_bytebuffer);
