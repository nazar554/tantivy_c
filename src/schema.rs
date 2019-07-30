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
    (&*options).is_stored()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_int_options_is_indexed(options: *const IntOptions) -> bool {
    (&*options).is_indexed()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_int_options_is_fast(options: *const IntOptions) -> bool {
    (&*options).is_fast()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_int_options_set_stored(options: *mut IntOptions) {
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
    take_mut::take(&mut *options, |options| {
        let cardinality: Option<_> = cardinality.into();
        options.set_fast(cardinality.unwrap())
    })
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_int_options_get_fastfield_cardinality(
    options: *const IntOptions,
) -> CCardinality {
    (&*options).get_fastfield_cardinality().into()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_schema_builder_add_u64_field(
    builder: *mut SchemaBuilder,
    field_name: *const u8,
    field_name_len: usize,
    field_options: *const IntOptions,
) -> u32 {
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
) -> *const TextFieldIndexing {
    match (&*options).get_indexing_options() {
        Some(reference) => reference,
        None => std::ptr::null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_text_options_is_stored(
    options: *const TextOptions,
) -> bool {
    (&*options).is_stored()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_text_options_set_stored(options: *mut TextOptions) {
    take_mut::take(&mut *options, |options| options.set_stored())
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_text_options_set_indexing_options(
    text_options: *mut TextOptions,
    options: *const TextFieldIndexing,
) {
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
    let tokenizer_name_slice = std::slice::from_raw_parts(tokenizer_name, tokenizer_name_len);
    let tokenizer_name_str = std::str::from_utf8_unchecked(tokenizer_name_slice);

    take_mut::take(&mut *options, |options| {
        options.set_tokenizer(tokenizer_name_str)
    })
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_text_field_indexing_tokenizer(
    options: *const TextFieldIndexing,
) -> crate::Span<u8> {
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
    take_mut::take(&mut *options, |options| {
        options.set_index_option(index_option.into())
    })
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_text_field_indexing_index_option(
    options: *mut TextFieldIndexing,
) -> CIndexRecordOption {
    (&*options).index_option().into()
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_schema_builder_add_text_field(
    builder: *mut SchemaBuilder,
    field_name: *const u8,
    field_name_len: usize,
    field_options: *const TextOptions,
) -> u32 {
    let field_name_slice = std::slice::from_raw_parts(field_name, field_name_len);

    let field = (&mut *builder).add_text_field(
        std::str::from_utf8_unchecked(field_name_slice),
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
    let field =
        (&mut *builder).add_bytes_field(crate::str_from_slice_parts(field_name, field_name_len));
    field.0
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_is_valid_field_name(
    field_name: *const u8,
    field_name_len: usize,
) -> bool {
    let field_name_slice = std::slice::from_raw_parts(field_name, field_name_len);

    is_valid_field_name(std::str::from_utf8_unchecked(field_name_slice))
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_schema_schema_builder_build(
    builder: *mut SchemaBuilder,
) -> *mut Schema {
    let builder = Box::from_raw(builder);

    box_new_into_raw!(builder.build())
}

dtor!(tantivy_schema, schema, Schema);
