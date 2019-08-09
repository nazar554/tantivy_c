use tantivy::schema::*;

ctor_dtor!(
    tantivy_schema,
    schema_builder,
    SchemaBuilder,
    Schema::builder()
);
ctor_dtor!(
    tantivy_schema,
    int_options,
    IntOptions,
    IntOptions::default()
);

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_int_options_is_stored(options: *const IntOptions) -> bool {
    debug_assert!(!options.is_null());
    (&*options).is_stored()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_int_options_is_indexed(options: *const IntOptions) -> bool {
    debug_assert!(!options.is_null());
    (&*options).is_indexed()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_int_options_is_fast(options: *const IntOptions) -> bool {
    debug_assert!(!options.is_null());
    (&*options).is_fast()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_int_options_set_stored(options: *mut IntOptions) {
    debug_assert!(!options.is_null());
    take_mut::take(&mut *options, |options| options.set_stored())
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_int_options_set_indexed(options: *mut IntOptions) {
    take_mut::take(&mut *options, |options| options.set_indexed())
}

#[repr(C)]
pub enum CCardinality {
    None,
    SingleValue,
    MultiValues,
}

impl From<Option<Cardinality>> for CCardinality {
    fn from(value: Option<Cardinality>) -> CCardinality {
        match value {
            Some(Cardinality::MultiValues) => CCardinality::MultiValues,
            Some(Cardinality::SingleValue) => CCardinality::SingleValue,
            None => CCardinality::None,
        }
    }
}

impl From<CCardinality> for Option<Cardinality> {
    fn from(value: CCardinality) -> Option<Cardinality> {
        match value {
            CCardinality::None => None,
            CCardinality::MultiValues => Some(Cardinality::MultiValues),
            CCardinality::SingleValue => Some(Cardinality::SingleValue),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_int_options_set_fast(
    options: *mut IntOptions,
    cardinality: CCardinality,
) {
    debug_assert!(!options.is_null());
    take_mut::take(&mut *options, |options| {
        let cardinality: Option<_> = cardinality.into();
        options.set_fast(cardinality.unwrap())
    })
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_int_options_get_fastfield_cardinality(
    options: *const IntOptions,
) -> CCardinality {
    debug_assert!(!options.is_null());
    (&*options).get_fastfield_cardinality().into()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_schema_builder_add_u64_field(
    builder: *mut SchemaBuilder,
    field_name: *const u8,
    field_name_len: usize,
    field_options: *const IntOptions,
) -> u32 {
    debug_assert!(!builder.is_null());
    debug_assert!(!field_options.is_null());

    let field = (&mut *builder).add_u64_field(
        crate::str_from_slice_parts(field_name, field_name_len),
        (&*field_options).clone(),
    );
    field.0
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_schema_builder_add_i64_field(
    builder: *mut SchemaBuilder,
    field_name: *const u8,
    field_name_len: usize,
    field_options: *const IntOptions,
) -> u32 {
    debug_assert!(!builder.is_null());
    debug_assert!(!field_options.is_null());

    let field = (&mut *builder).add_i64_field(
        crate::str_from_slice_parts(field_name, field_name_len),
        (&*field_options).clone(),
    );
    field.0
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_schema_builder_add_date_field(
    builder: *mut SchemaBuilder,
    field_name: *const u8,
    field_name_len: usize,
    field_options: *const IntOptions,
) -> u32 {
    debug_assert!(!builder.is_null());
    debug_assert!(!field_options.is_null());

    let field = (&mut *builder).add_date_field(
        crate::str_from_slice_parts(field_name, field_name_len),
        (&*field_options).clone(),
    );
    field.0
}

ctor_dtor!(
    tantivy_schema,
    text_options,
    TextOptions,
    TextOptions::default()
);

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_text_options_get_indexing_options(
    options: *const TextOptions,
) -> *mut TextFieldIndexing {
    debug_assert!(!options.is_null());

    match (&*options).get_indexing_options() {
        Some(reference) => box_new_into_raw!(reference.clone()),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_text_options_is_stored(
    options: *const TextOptions,
) -> bool {
    debug_assert!(!options.is_null());
    (&*options).is_stored()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_text_options_set_stored(options: *mut TextOptions) {
    debug_assert!(!options.is_null());
    take_mut::take(&mut *options, |options| options.set_stored())
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_text_options_set_indexing_options(
    text_options: *mut TextOptions,
    options: *const TextFieldIndexing,
) {
    debug_assert!(!text_options.is_null());
    debug_assert!(!options.is_null());
    take_mut::take(&mut *text_options, |text_options| {
        text_options.set_indexing_options((&*options).clone())
    })
}

ctor_dtor!(
    tantivy_schema,
    text_field_indexing,
    TextFieldIndexing,
    TextFieldIndexing::default()
);

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_text_field_indexing_set_tokenizer(
    options: *mut TextFieldIndexing,
    tokenizer_name: *const u8,
    tokenizer_name_len: usize,
) {
    debug_assert!(!options.is_null());

    take_mut::take(&mut *options, |options| {
        options.set_tokenizer(crate::str_from_slice_parts(
            tokenizer_name,
            tokenizer_name_len,
        ))
    })
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_text_field_indexing_tokenizer(
    options: *const TextFieldIndexing,
) -> crate::Span<u8> {
    debug_assert!(!options.is_null());
    (&*options).tokenizer().as_bytes().into()
}

#[repr(C)]
pub enum CIndexRecordOption {
    Basic,
    WithFreqs,
    WithFreqsAndPositions,
}

impl From<IndexRecordOption> for CIndexRecordOption {
    fn from(option: IndexRecordOption) -> CIndexRecordOption {
        match option {
            IndexRecordOption::Basic => CIndexRecordOption::Basic,
            IndexRecordOption::WithFreqs => CIndexRecordOption::WithFreqs,
            IndexRecordOption::WithFreqsAndPositions => CIndexRecordOption::WithFreqsAndPositions,
        }
    }
}

impl From<CIndexRecordOption> for IndexRecordOption {
    fn from(option: CIndexRecordOption) -> IndexRecordOption {
        match option {
            CIndexRecordOption::Basic => IndexRecordOption::Basic,
            CIndexRecordOption::WithFreqs => IndexRecordOption::WithFreqs,
            CIndexRecordOption::WithFreqsAndPositions => IndexRecordOption::WithFreqsAndPositions,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_text_field_indexing_set_index_option(
    options: *mut TextFieldIndexing,
    index_option: CIndexRecordOption,
) {
    debug_assert!(!options.is_null());
    take_mut::take(&mut *options, |options| {
        options.set_index_option(index_option.into())
    })
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_text_field_indexing_index_option(
    options: *mut TextFieldIndexing,
) -> CIndexRecordOption {
    debug_assert!(!options.is_null());
    (&*options).index_option().into()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_schema_builder_add_text_field(
    builder: *mut SchemaBuilder,
    field_name: *const u8,
    field_name_len: usize,
    field_options: *const TextOptions,
) -> u32 {
    debug_assert!(!builder.is_null());
    debug_assert!(!field_options.is_null());

    let field = (&mut *builder).add_text_field(
        crate::str_from_slice_parts(field_name, field_name_len),
        (&*field_options).clone(),
    );
    field.0
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_schema_builder_add_facet_field(
    builder: *mut SchemaBuilder,
    field_name: *const u8,
    field_name_len: usize,
) -> u32 {
    debug_assert!(!builder.is_null());
    let field =
        (&mut *builder).add_facet_field(crate::str_from_slice_parts(field_name, field_name_len));
    field.0
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_schema_builder_add_bytes_field(
    builder: *mut SchemaBuilder,
    field_name: *const u8,
    field_name_len: usize,
) -> u32 {
    debug_assert!(!builder.is_null());
    let field =
        (&mut *builder).add_bytes_field(crate::str_from_slice_parts(field_name, field_name_len));
    field.0
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_is_valid_field_name(
    field_name: *const u8,
    field_name_len: usize,
) -> bool {
    is_valid_field_name(crate::str_from_slice_parts(field_name, field_name_len))
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_schema_builder_build(
    builder: *mut SchemaBuilder,
) -> *mut Schema {
    debug_assert!(!builder.is_null());
    let builder = Box::from_raw(builder);

    box_new_into_raw!(builder.build())
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_schema_get_field_name(
    schema: *const Schema,
    field: u32,
) -> crate::Span<u8> {
    debug_assert!(!schema.is_null());
    (&*schema).get_field_name(Field(field)).as_bytes().into()
}

dtor!(tantivy_schema, schema, Schema);

ctor_dtor!(tantivy_schema, document, Document, Document::new());

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_document_len(document: *const Document) -> usize {
    debug_assert!(!document.is_null());
    (&*document).len()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_document_is_empty(document: *const Document) -> bool {
    debug_assert!(!document.is_null());
    (&*document).is_empty()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_document_filter_fields(
    document: *mut Document,
    func: extern "C" fn(u32) -> bool,
) {
    debug_assert!(!document.is_null());
    (&mut *document).filter_fields(|field| func(field.0))
}

// TODO: tantivy_schema_document_add_facet

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_document_add_text(
    document: *mut Document,
    field: u32,
    text: *const u8,
    len: usize,
) {
    debug_assert!(!document.is_null());
    (&mut *document).add_text(Field(field), crate::str_from_slice_parts(text, len))
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_document_add_u64(
    document: *mut Document,
    field: u32,
    value: u64,
) {
    debug_assert!(!document.is_null());
    (&mut *document).add_u64(Field(field), value)
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_document_add_i64(
    document: *mut Document,
    field: u32,
    value: i64,
) {
    debug_assert!(!document.is_null());
    (&mut *document).add_i64(Field(field), value)
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_document_add_date(
    document: *mut Document,
    field: u32,
    year: i32,
    ordinal: u32,
    secs: u32,
    nano: u32,
) {
    use chrono::{Utc, NaiveTime, TimeZone};

    debug_assert!(!document.is_null());

    let date = Utc.yo(year, ordinal);
    let time = NaiveTime::from_num_seconds_from_midnight(secs, nano);
    let date_time = date.and_time(time).unwrap();

    (&mut *document).add_date(Field(field), &date_time)
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_document_add_bytes(
    document: *mut Document,
    field: u32,
    buffer: *const u8,
    len: usize,
) {
    debug_assert!(!document.is_null());
    debug_assert!(!buffer.is_null() || len == 0);

    let bytes = std::slice::from_raw_parts(buffer, len);
    (&mut *document).add_bytes(Field(field), bytes.to_owned())
}

// TODO: tantivy_schema_document_add
// TODO: tantivy_schema_document_field_values
// TODO: tantivy_schema_document_get_sorted_field_values
// TODO: tantivy_schema_document_get_all
// TODO: tantivy_schema_document_get_first
