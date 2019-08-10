use tantivy::schema::*;
use tantivy::*;

dtor!(tantivy, index, Index);

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_create_in_ram_copy(schema: *const Schema) -> *mut Index {
    debug_assert!(!schema.is_null());
    box_new_into_raw!(Index::create_in_ram((&*schema).clone()))
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_create_in_ram_move(schema: *mut Schema) -> *mut Index {
    debug_assert!(!schema.is_null());
    box_new_into_raw!(Index::create_in_ram(*Box::from_raw(schema)))
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_create_from_tempdir_copy(
    schema: *const Schema,
    out_error: *mut *mut TantivyError,
) -> *mut Index {
    debug_assert!(!schema.is_null());
    crate::map_result_boxed(Index::create_from_tempdir((&*schema).clone()), out_error)
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_create_from_tempdir_move(
    schema: *mut Schema,
    out_error: *mut *mut TantivyError,
) -> *mut Index {
    debug_assert!(!schema.is_null());
    crate::map_result_boxed(
        Index::create_from_tempdir(*Box::from_raw(schema)),
        out_error,
    )
}

dtor!(tantivy_index, index_writer, IndexWriter);

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_writer(
    index: *const Index,
    overall_heap_size_in_bytes: usize,
    out_error: *mut *mut TantivyError,
) -> *mut IndexWriter {
    debug_assert!(!index.is_null());
    crate::map_result_boxed((&*index).writer(overall_heap_size_in_bytes), out_error)
}

dtor!(tantivy_index, index_reader, IndexReader);

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_reader(
    index: *const Index,
    out_error: *mut *mut TantivyError,
) -> *mut IndexReader {
    debug_assert!(!index.is_null());
    crate::map_result_boxed((&*index).reader(), out_error)
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_writer_with_num_threads(
    index: *const Index,
    num_threads: usize,
    overall_heap_size_in_bytes: usize,
    out_error: *mut *mut TantivyError,
) -> *mut IndexWriter {
    debug_assert!(!index.is_null());
    crate::map_result_boxed(
        (&*index).writer_with_num_threads(num_threads, overall_heap_size_in_bytes),
        out_error,
    )
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_set_multithread_executor(
    index: *mut Index,
    num_threads: usize,
) {
    debug_assert!(!index.is_null());
    (&mut *index).set_multithread_executor(num_threads)
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_set_default_multithread_executor(index: *mut Index) {
    debug_assert!(!index.is_null());
    (&mut *index).set_default_multithread_executor()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_schema(index: *const Index) -> *mut Schema {
    debug_assert!(!index.is_null());
    box_new_into_raw!((&*index).schema())
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_create_in_dir_copy(
    path: *const u8,
    path_len: usize,
    schema: *const Schema,
    out_error: *mut *mut TantivyError,
) -> *mut Index {
    debug_assert!(!schema.is_null());
    let path = crate::str_from_slice_parts(path, path_len);
    crate::map_result_boxed(Index::create_in_dir(path, (&*schema).clone()), out_error)
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_create_in_dir_move(
    path: *const u8,
    path_len: usize,
    schema: *mut Schema,
    out_error: *mut *mut TantivyError,
) -> *mut Index {
    debug_assert!(!schema.is_null());
    let path = crate::str_from_slice_parts(path, path_len);
    crate::map_result_boxed(
        Index::create_in_dir(path, *Box::from_raw(schema)),
        out_error,
    )
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_writer_add_document_copy(
    writer: *mut IndexWriter,
    document: *const Document,
) -> u64 {
    debug_assert!(!writer.is_null());
    debug_assert!(!document.is_null());

    (&mut *writer).add_document((&*document).clone())
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_writer_add_document_move(
    writer: *mut IndexWriter,
    document: *mut Document,
) -> u64 {
    debug_assert!(!writer.is_null());
    debug_assert!(!document.is_null());

    (&mut *writer).add_document(*Box::from_raw(document))
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_writer_commit(
    writer: *mut IndexWriter,
    out_error: *mut *mut TantivyError,
) -> u64 {
    debug_assert!(!writer.is_null());

    crate::map_result((&mut *writer).commit(), out_error)
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_reader_searcher(
    reader: *const IndexReader,
) -> *mut LeasedItem<Searcher> {
    debug_assert!(!reader.is_null());

    box_new_into_raw!((&*reader).searcher())
}

dtor!(tantivy_index, leased_searcher, LeasedItem<Searcher>);
