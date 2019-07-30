use tantivy::schema::*;
use tantivy::*;

dtor!(tantivy, index, Index);

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_create_in_ram(schema: *mut Schema) -> *mut Index {
    let schema = Box::from_raw(schema);

    box_new_into_raw!(Index::create_in_ram(*schema))
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
