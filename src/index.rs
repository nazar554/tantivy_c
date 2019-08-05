use tantivy::schema::*;
use tantivy::*;

dtor!(tantivy, index, Index);

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_create_in_ram(schema: *const Schema) -> *mut Index {
    box_new_into_raw!(Index::create_in_ram((&*schema).clone()))
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_create_from_tempdir(
    schema: *const Schema,
    out_error: *mut *mut tantivy::TantivyError,
) -> *mut Index {
    crate::map_result(Index::create_from_tempdir((&*schema).clone()), out_error)
}

dtor!(tantivy_index, index_writer, IndexWriter);

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_writer(
    index: *const Index,
    overall_heap_size_in_bytes: usize,
    out_error: *mut *mut tantivy::TantivyError,
) -> *mut IndexWriter {
    crate::map_result((&*index).writer(overall_heap_size_in_bytes), out_error)
}

dtor!(tantivy_index, index_reader, IndexReader);

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_reader(
    index: *const Index,
    out_error: *mut *mut tantivy::TantivyError,
) -> *mut IndexReader {
    crate::map_result((&*index).reader(), out_error)
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_writer_with_num_threads(
    index: *const Index,
    num_threads: usize,
    overall_heap_size_in_bytes: usize,
    out_error: *mut *mut tantivy::TantivyError,
) -> *mut IndexWriter {
    crate::map_result(
        (&*index).writer_with_num_threads(num_threads, overall_heap_size_in_bytes),
        out_error,
    )
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_set_multithread_executor(
    index: *mut Index,
    num_threads: usize,
) {
    (&mut *index).set_multithread_executor(num_threads)
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_set_default_multithread_executor(index: *mut Index) {
    (&mut *index).set_default_multithread_executor()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_schema(index: *const Index) -> *mut Schema {
    box_new_into_raw!((&*index).schema())
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_create_in_dir(
    path: *const u8,
    path_len: usize,
    schema: *const Schema,
    out_error: *mut *mut tantivy::TantivyError,
) -> *mut Index {
    debug_assert!(!schema.is_null());
    let path = crate::str_from_slice_parts(path, path_len);

    match Index::create_in_dir(path, (&*schema).clone()) {
        Ok(index) => box_new_into_raw!(index),
        Err(e) => {
            if !out_error.is_null() {
                *out_error = box_new_into_raw!(e);
            }
            std::ptr::null_mut()
        }
    }
}
