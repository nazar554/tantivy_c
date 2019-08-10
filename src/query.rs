use tantivy::query::*;
use tantivy::schema::*;
use tantivy::*;

dtor!(tantivy, query_parser, QueryParser);

#[no_mangle]
pub unsafe extern "C" fn tantivy_query_parser_for_index(
    index: *const Index,
    fields: *const u32,
    fields_len: usize,
) -> *mut QueryParser {
    debug_assert!(!index.is_null());
    debug_assert!(!fields.is_null() || fields_len == 0);

    let fields: Vec<Field> = std::slice::from_raw_parts(fields, fields_len)
        .into_iter()
        .cloned()
        .map(Field)
        .collect();

    box_new_into_raw!(QueryParser::for_index(&*index, fields))
}

dtor!(tantivy, boxed_dyn_query, Box<dyn Query>);
dtor!(tantivy, query_parser_error, QueryParserError);

#[no_mangle]
pub unsafe extern "C" fn tantivy_query_parser_parse_query(
    parser: *const QueryParser,
    query: *const u8,
    query_len: usize,
    out_error: *mut *mut QueryParserError,
) -> *mut Box<dyn Query> {
    debug_assert!(!parser.is_null());

    let query = crate::str_from_slice_parts(query, query_len);

    crate::map_result_boxed((&*parser).parse_query(query), out_error)
}
