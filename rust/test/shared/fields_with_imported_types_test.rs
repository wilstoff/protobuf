// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

/// Tests covering that fields with types that are defined in imported .proto
/// files are generated. In particular where the imported .proto file is part of
/// a separate proto_library target.
use googletest::prelude::*;

#[test]
fn test_message_field_generated() {
    use fields_with_imported_types_proto::MsgWithFieldsWithImportedTypes;
    use imported_types_proto::ImportedMessageView;

    let msg = MsgWithFieldsWithImportedTypes::new();
    assert_that!(msg.imported_message_field(), matches_pattern!(ImportedMessageView { .. }));
}

#[test]
fn test_enum_field_generated() {
    use fields_with_imported_types_proto::MsgWithFieldsWithImportedTypes;
    use imported_types_proto::ImportedEnum;

    let msg = MsgWithFieldsWithImportedTypes::new();
    assert_that!(msg.imported_enum_field(), eq(ImportedEnum::Unknown));
}

#[test]
fn test_oneof_message_field_generated() {
    use fields_with_imported_types_proto::MsgWithFieldsWithImportedTypes;
    use fields_with_imported_types_proto::MsgWithFieldsWithImportedTypes_::ImportedTypesOneof::not_set;
    use imported_types_proto::ImportedEnum;
    use imported_types_proto::ImportedMessageView;

    let msg = MsgWithFieldsWithImportedTypes::new();
    assert_that!(msg.imported_message_oneof(), matches_pattern!(ImportedMessageView { .. }));
    assert_that!(msg.imported_enum_oneof(), eq(ImportedEnum::Unknown));
    assert_that!(msg.imported_types_oneof(), matches_pattern!(not_set(_)));
}
