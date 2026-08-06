use rl_ast::{
    nodes::ExpressionKind,
    statements::{StatementKind, TypeAnnotation},
};

use crate::common::{self, span_of, span_up_to, span_whole};
use crate::{assert_array_decl, assert_decl};

#[test]
fn dec_int() {
    let source = "dec int x = 1000";
    assert_decl!(
        source,
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Int,
        value: ExpressionKind::Integer(1000), span_of(source, "1000"),
        span: span_whole(source),
    );
}

#[test]
fn const_int() {
    let source = "CONST int x = 1000";
    assert_decl!(
        source,
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CInt,
        value: ExpressionKind::Integer(1000), span_of(source, "1000"),
        span: span_whole(source),
    );
}

#[test]
fn dec_float() {
    let source = "dec float x = 1000.0";
    assert_decl!(
        source,
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Float,
        value: ExpressionKind::Float(1000.0), span_of(source, "1000.0"),
        span: span_whole(source),
    );
}

/* out of scope and assigned to namish
/*
#[test]
fn dec_uint() {
    assert_decl!(
        "dec uint x = 1000 as uint",
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::UInt,
        value: ExpressionKind::UInt(1000),
    );
}
*/
/*
#[test]
fn const_uint() {
    assert_decl!(
        "CONST uint x = 1000 as uint",
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CUInt,
        value: ExpressionKind::UInt(1000),
    );
}
*/
*/

#[test]
fn dec_small_int() {
    assert_decl!(
        "dec small int x = 1000 as small int",
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::SInt,
        value: ExpressionKind::SInt(1000),
    );
}

#[test]
fn const_small_int() {
    assert_decl!(
        "CONST small int x = 1000 as small int",
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CSInt,
        value: ExpressionKind::SInt(1000),
    );
}

/* out of scope
#[test]
fn dec_small_uint() {
    assert_decl!(
        "dec small uint x = 1000 as small uint",
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::SUInt,
        value: ExpressionKind::SUInt(1000),
    );
}

#[test]
fn const_small_uint() {
    assert_decl!(
        "CONST small uint x = 1000 as small uint",
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CSUInt,
        value: ExpressionKind::SUInt(1000),
    );
}

#[test]
fn dec_big_byte() {
    assert_decl!(
        "dec big byte x = 1000 as big byte",
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::BByte,
        value: ExpressionKind::BByte(1000),
    );
}

#[test]
fn const_big_byte() {
    assert_decl!(
        "CONST big byte x = 1000 as big byte",
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CBByte,
        value: ExpressionKind::BByte(1000),
    );
}
*/

#[test]
fn dec_big_sbyte() {
    assert_decl!(
        "dec big sbyte x = 1000 as big sbyte",
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::BSByte,
        value: ExpressionKind::BSByte(1000),
    );
}

#[test]
fn const_big_sbyte() {
    assert_decl!(
        "CONST big sbyte x = 1000 as big sbyte",
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CBSByte,
        value: ExpressionKind::BSByte(1000),
    );
}

#[test]
fn dec_sbyte() {
    assert_decl!(
        "dec sbyte x = 100 as sbyte",
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::SByte,
        value: ExpressionKind::SByte(100),
    );
}

#[test]
fn const_sbyte() {
    assert_decl!(
        "CONST sbyte x = 100 as sbyte",
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CSByte,
        value: ExpressionKind::SByte(100),
    );
}

#[test]
fn const_float() {
    let source = "CONST float x = 1000.0";
    assert_decl!(
        source,
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CFloat,
        value: ExpressionKind::Float(1000.0), span_of(source, "1000.0"),
        span: span_whole(source),
    );
}

#[test]
fn dec_string() {
    let source = "dec string x = \"hi\"";
    assert_decl!(
        source,
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::String,
        value: ExpressionKind::String("hi".to_string()), span_of(source, "\"hi\""),
        span: span_whole(source),
    );
}

#[test]
fn const_string() {
    let source = "CONST string x = \"hi\"";
    assert_decl!(
        source,
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CString,
        value: ExpressionKind::String("hi".to_string()), span_of(source, "\"hi\""),
        span: span_whole(source),
    );
}

#[test]
fn dec_char() {
    let source = "dec char x = 'x'";
    assert_decl!(
        source,
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Char,
        value: ExpressionKind::Character('x'), span_of(source, "'x'"),
        span: span_whole(source),
    );
}

#[test]
fn const_char() {
    let source = "CONST char x = 'x'";
    assert_decl!(
        source,
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CChar,
        value: ExpressionKind::Character('x'), span_of(source, "'x'"),
        span: span_whole(source),
    );
}

#[test]
fn dec_bool() {
    let source = "dec bool x = true";
    assert_decl!(
        source,
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Bool,
        value: ExpressionKind::Bool(true), span_of(source, "true"),
        span: span_whole(source),
    );
}

#[test]
fn const_bool() {
    let source = "CONST bool x = false";
    assert_decl!(
        source,
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CBool,
        value: ExpressionKind::Bool(false), span_of(source, "false"),
        span: span_whole(source),
    );
}

#[test]
fn dec_byte() {
    let source = "dec byte x = 65 as byte";
    assert_decl!(
        source,
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Byte,
        value: ExpressionKind::Byte(65), span_of(source, "65"),
        span: span_up_to(source, "65"),
    );
}

#[test]
fn const_byte() {
    let source = "CONST byte x = 65 as byte";
    assert_decl!(
        source,
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::CByte,
        value: ExpressionKind::Byte(65), span_of(source, "65"),
        span: span_up_to(source, "65"),
    );
}

#[test]
fn dec_array() {
    let source = "dec arr[int] x = [1]";
    assert_array_decl!(
        source,
        StatementKind::Array,
        name: "x",
        type_annotation: TypeAnnotation::Int,
        item: ExpressionKind::Integer(1), span_of(source, "1"),
        span: span_whole(source),
    );
}

#[test]
fn const_array() {
    let source = "CONST arr[int] x = [1]";
    assert_array_decl!(
        source,
        StatementKind::ConstantArray,
        name: "x",
        type_annotation: TypeAnnotation::Int,
        item: ExpressionKind::Integer(1), span_of(source, "1"),
        span: span_whole(source),
    );
}

// Empty lambda body/params means there's no nested ExprId to worry about,
// so this one *can* go through assert_decl! directly.
#[test]
fn dec_fn() {
    let source = "dec fn x = fn(){}";
    assert_decl!(
        source,
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Fn,
        value: ExpressionKind::Lambda { params: vec![], return_type: None, body: vec![] }, span_of(source, "fn(){}"),
        span: span_whole(source),
    );
}

#[test]
fn const_fn() {
    let source = "CONST fn x = fn(){}";
    assert_decl!(
        source,
        StatementKind::ConstantDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Fn,
        value: ExpressionKind::Lambda { params: vec![], return_type: None, body: vec![] }, span_of(source, "fn(){}"),
        span: span_whole(source),
    );
}
