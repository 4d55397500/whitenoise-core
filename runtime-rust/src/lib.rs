extern crate libc;

use libc::{c_char, strdup};
use std::ffi::CStr;
use std::str;

use std::path::Path;

#[macro_use]
extern crate arrow;

use arrow::array::{BinaryArray, Float64Array};
use arrow::csv;
use std::fs::File;

mod base;
use base::burdock;
use arrow::datatypes::ToByteSlice;
use ffi_support::ByteBuffer;

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


#[no_mangle]
pub extern "C" fn release(
    analysis_ptr: *const u8, analysis_length: i32,
    release_ptr: *const u8, release_length: i32,
    data_path: *const c_char, _data_path_length: i32,
    header: *const c_char, _header_length: i32) -> ffi_support::ByteBuffer {

    let analysis_buffer = unsafe {get_buffer(analysis_ptr, analysis_length)};
    let analysis: burdock::Analysis = prost::Message::decode(analysis_buffer).unwrap();

    let release_buffer = unsafe {get_buffer(release_ptr, release_length)};
    let release: burdock::Release = prost::Message::decode(release_buffer).unwrap();

    let c_str_data_path: &str = unsafe { CStr::from_ptr(data_path) }.to_str().unwrap();

    let data_file = File::open(Path::new(c_str_data_path)).unwrap();
    let builder = csv::ReaderBuilder::new()
        .has_headers(true)
        .infer_schema(Some(100));

    let mut csv = builder.build(data_file).unwrap();
//    println!("{:?}", csv);
    let batch = csv.next().unwrap().unwrap();
    println!(
        "Loaded {} rows containing {} columns",
        batch.num_rows(),
        batch.num_columns()
    );

    println!("Inferred schema: {:?}", batch.schema());

    println!("proto analysis: {:?}", analysis);
    println!("proto release : {:?}", release);

    let mut out_buffer = Vec::new();
    prost::Message::encode(&release, &mut out_buffer);
    ffi_support::ByteBuffer::from_vec(out_buffer)
}

#[no_mangle]
pub extern "C" fn release_array(
    analysis_buffer: *const c_char, _analysis_length: i32,
    release_buffer: *const c_char, _release_length: i32,
    m: i32, n: i32, data: *const*const f64,
    header: *const c_char, _header_length: i32) -> *const c_char {

    let c_str_analysis: &CStr = unsafe { CStr::from_ptr(analysis_buffer) };
    let analysis = <burdock::Analysis as prost::Message>::decode(c_str_analysis.to_bytes().to_vec());

    let c_str_release: &CStr = unsafe { CStr::from_ptr(release_buffer) };
    let release = <burdock::Analysis as prost::Message>::decode(c_str_release.to_bytes().to_vec());

    return unsafe { strdup(release_buffer) };
}

#[no_mangle]
pub extern "C" fn free_ptr(buffer: *const c_char) {
//    unsafe { libc::free(buffer)};
}

//ffi_support::implement_into_ffi_by_protobuf!(burdock::Release);
ffi_support::define_bytebuffer_destructor!(dp_runtime_destroy_bytebuffer);