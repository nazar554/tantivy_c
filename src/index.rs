use tantivy::schema::*;
use tantivy::*;

dtor!(tantivy, index, Index);

#[no_mangle]
pub unsafe extern "C" fn tantivy_index_create_in_ram(schema: *mut Schema) -> *mut Index {
    let schema = Box::from_raw(schema);

    box_new_into_raw!(Index::create_in_ram(*schema))
}
