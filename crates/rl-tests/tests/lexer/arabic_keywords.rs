//! Tests for Arabic keyword aliases.
//! Each Arabic keyword must produce the same TokenType as its English counterpart.
//! Also tests complex Arabic code patterns, identifier edge cases, and operator interactions.

use super::super::common;
use rl_lexer::tokentypes::TokenType;

const ARABIC_KEYWORDS: [(&str, TokenType); 38] = [
    ("دالة", TokenType::Fn),
    ("لكل", TokenType::For),
    ("بينما", TokenType::While),
    ("أرجع", TokenType::Return),
    ("استمر", TokenType::Continue),
    ("توقف", TokenType::Break),
    ("استورد", TokenType::Get),
    ("من", TokenType::From),
    ("في", TokenType::In),
    ("أو", TokenType::Or),
    ("و", TokenType::And),
    ("فارغ", TokenType::Null),
    ("عدد", TokenType::Int),
    ("ثابت", TokenType::Const),
    ("عشري", TokenType::Float),
    ("منطقي", TokenType::Bool),
    ("نص", TokenType::String),
    ("بايت", TokenType::Byte),
    ("حرف", TokenType::Char),
    ("صحيح", TokenType::BoolLiteral(true)),
    ("ليس_صحيح", TokenType::BoolLiteral(false)),
    ("أعلن", TokenType::Dec),
    ("إذا", TokenType::If),
    ("وإلا", TokenType::Else),
    ("مصفوفة", TokenType::Array),
    ("بصفة", TokenType::As),
    ("خطأ", TokenType::Error),
    ("نتيجة", TokenType::Result),
    ("نجاح", TokenType::Ok),
    ("فشل", TokenType::Err),
    ("طابق", TokenType::Match),
    ("سجل", TokenType::Record),
    ("تنفيذ", TokenType::Impl),
    ("وسم", TokenType::Tag),
    ("خريطة", TokenType::Map),
    ("مجموعة", TokenType::Set),
    ("تكرار", TokenType::Loop),
    ("مقبض", TokenType::Handle),
];

// ---------------------------------------------------------------------------
// Core keyword tests
// ---------------------------------------------------------------------------

#[test]
fn arabic_keywords_lex_to_correct_token_types() {
    for (arabic, expected_type) in ARABIC_KEYWORDS {
        let tokens = common::lex(arabic);
        assert_eq!(
            tokens[0].token, expected_type,
            "Arabic keyword '{}' should lex to {:?}",
            arabic, expected_type
        );
        assert_eq!(
            tokens[0].lexeme, arabic,
            "Arabic keyword '{}' lexeme should be preserved",
            arabic
        );
    }
}

#[test]
fn arabic_keywords_in_function_declaration() {
    let tokens = common::lex("دالة جمع(أ: عدد, ب: عدد) -> عدد { أ + ب }");
    assert_eq!(tokens[0].token, TokenType::Fn);
    assert_eq!(tokens[0].lexeme, "دالة");
    assert_eq!(tokens[1].token, TokenType::Identifier("جمع".into()));
    assert_eq!(tokens[5].token, TokenType::Int);
    assert_eq!(tokens[5].lexeme, "عدد");
}

#[test]
fn arabic_keywords_in_if_else() {
    let tokens = common::lex("إذا صحيح { 1 } وإلا { 0 }");
    assert_eq!(tokens[0].token, TokenType::If);
    assert_eq!(tokens[0].lexeme, "إذا");
    assert_eq!(tokens[1].token, TokenType::BoolLiteral(true));
    assert_eq!(tokens[1].lexeme, "صحيح");
    assert_eq!(tokens[5].token, TokenType::Else);
    assert_eq!(tokens[5].lexeme, "وإلا");
}

#[test]
fn arabic_keywords_in_tag_declaration() {
    let tokens = common::lex("وسم الحالة { متاح مشغول }");
    assert_eq!(tokens[0].token, TokenType::Tag);
    assert_eq!(tokens[0].lexeme, "وسم");
    assert_eq!(tokens[1].token, TokenType::Identifier("الحالة".into()));
}

#[test]
fn arabic_keywords_in_for_loop() {
    let tokens = common::lex("لكل أ في [1, 2, 3] { أ }");
    assert_eq!(tokens[0].token, TokenType::For);
    assert_eq!(tokens[0].lexeme, "لكل");
    assert_eq!(tokens[2].token, TokenType::In);
    assert_eq!(tokens[2].lexeme, "في");
}

#[test]
fn mixed_english_arabic_identifiers_are_identifiers() {
    let tokens = common::lex("متغير_englishعربي");
    assert_eq!(tokens[0].token, TokenType::Identifier("متغير_englishعربي".into()));
}

#[test]
fn arabic_and_english_keywords_can_be_mixed_in_program() {
    let source = "fn خلص() -> نص { \"مرحبا\" }";
    let tokens = common::lex(source);
    assert_eq!(tokens[0].token, TokenType::Fn);
    assert_eq!(tokens[0].lexeme, "fn");
    assert_eq!(tokens[1].token, TokenType::Identifier("خلص".into()));
    assert_eq!(tokens[5].token, TokenType::String);
    assert_eq!(tokens[5].lexeme, "نص");
}

// ---------------------------------------------------------------------------
// Control flow
// ---------------------------------------------------------------------------

#[test]
fn arabic_while_loop() {
    let tokens = common::lex("بينما صحيح { توقف }");
    assert_eq!(tokens[0].token, TokenType::While);
    assert_eq!(tokens[0].lexeme, "بينما");
    assert_eq!(tokens[1].token, TokenType::BoolLiteral(true));
    assert_eq!(tokens[3].token, TokenType::Break);
    assert_eq!(tokens[3].lexeme, "توقف");
}

#[test]
fn arabic_return_statement() {
    let tokens = common::lex("أرجع 42");
    assert_eq!(tokens[0].token, TokenType::Return);
    assert_eq!(tokens[0].lexeme, "أرجع");
    assert_eq!(tokens[1].token, TokenType::NumberLiteral(42));
}

#[test]
fn arabic_continue_statement() {
    let tokens = common::lex("استمر");
    assert_eq!(tokens[0].token, TokenType::Continue);
    assert_eq!(tokens[0].lexeme, "استمر");
}

#[test]
fn arabic_break_statement() {
    let tokens = common::lex("توقف");
    assert_eq!(tokens[0].token, TokenType::Break);
    assert_eq!(tokens[0].lexeme, "توقف");
}

#[test]
fn arabic_loop_keyword() {
    let tokens = common::lex("تكرار { أ += 1 }");
    assert_eq!(tokens[0].token, TokenType::Loop);
    assert_eq!(tokens[0].lexeme, "تكرار");
}

// ---------------------------------------------------------------------------
// Declarations
// ---------------------------------------------------------------------------

#[test]
fn arabic_const_declaration() {
    let tokens = common::lex("ثابت PI = 3.14");
    assert_eq!(tokens[0].token, TokenType::Const);
    assert_eq!(tokens[0].lexeme, "ثابت");
    assert_eq!(tokens[1].token, TokenType::Identifier("PI".into()));
}

#[test]
fn arabic_dec_declaration() {
    let tokens = common::lex("أعلن اسم = \"أحمد\"");
    assert_eq!(tokens[0].token, TokenType::Dec);
    assert_eq!(tokens[0].lexeme, "أعلن");
    assert_eq!(tokens[1].token, TokenType::Identifier("اسم".into()));
}

#[test]
fn arabic_record_declaration() {
    let tokens = common::lex("سجل نقطة(س: عشري, ص: عشري)");
    assert_eq!(tokens[0].token, TokenType::Record);
    assert_eq!(tokens[0].lexeme, "سجل");
    assert_eq!(tokens[1].token, TokenType::Identifier("نقطة".into()));
}

#[test]
fn arabic_impl_block() {
    let tokens = common::lex("تنفيذ نقطة { دالة مسافة(self) -> عشري { 0 } }");
    assert_eq!(tokens[0].token, TokenType::Impl);
    assert_eq!(tokens[0].lexeme, "تنفيذ");
    assert_eq!(tokens[1].token, TokenType::Identifier("نقطة".into()));
    assert_eq!(tokens[3].token, TokenType::Fn);
    assert_eq!(tokens[3].lexeme, "دالة");
}

#[test]
fn arabic_import_statement() {
    let tokens = common::lex("استورد io من std");
    assert_eq!(tokens[0].token, TokenType::Get);
    assert_eq!(tokens[0].lexeme, "استورد");
    assert_eq!(tokens[1].token, TokenType::Identifier("io".into()));
    assert_eq!(tokens[2].token, TokenType::From);
    assert_eq!(tokens[2].lexeme, "من");
    assert_eq!(tokens[3].token, TokenType::Identifier("std".into()));
}

// ---------------------------------------------------------------------------
// Match and pattern matching
// ---------------------------------------------------------------------------

#[test]
fn arabic_match_expression() {
    let tokens = common::lex("طابق حالة { نجاح => 1, فشل => 0 }");
    assert_eq!(tokens[0].token, TokenType::Match);
    assert_eq!(tokens[0].lexeme, "طابق");
    assert_eq!(tokens[1].token, TokenType::Identifier("حالة".into()));
}

#[test]
fn arabic_match_with_fat_arrow() {
    let tokens = common::lex("طابق x { أ => 1, ب => 2 }");
    assert_eq!(tokens[0].token, TokenType::Match);
    assert_eq!(tokens[4].token, TokenType::FatArrow);
}

// ---------------------------------------------------------------------------
// Type keywords
// ---------------------------------------------------------------------------

#[test]
fn arabic_type_keywords_all() {
    let tokens = common::lex("عدد عشري منطقي نص بايت حرف مصفوفة خريطة مجموعة");
    assert_eq!(tokens[0].token, TokenType::Int);
    assert_eq!(tokens[1].token, TokenType::Float);
    assert_eq!(tokens[2].token, TokenType::Bool);
    assert_eq!(tokens[3].token, TokenType::String);
    assert_eq!(tokens[4].token, TokenType::Byte);
    assert_eq!(tokens[5].token, TokenType::Char);
    assert_eq!(tokens[6].token, TokenType::Array);
    assert_eq!(tokens[7].token, TokenType::Map);
    assert_eq!(tokens[8].token, TokenType::Set);
}

#[test]
fn arabic_result_ok_err_types() {
    let tokens = common::lex("نتيجة نجاح فشل خطأ");
    assert_eq!(tokens[0].token, TokenType::Result);
    assert_eq!(tokens[0].lexeme, "نتيجة");
    assert_eq!(tokens[1].token, TokenType::Ok);
    assert_eq!(tokens[1].lexeme, "نجاح");
    assert_eq!(tokens[2].token, TokenType::Err);
    assert_eq!(tokens[2].lexeme, "فشل");
    assert_eq!(tokens[3].token, TokenType::Error);
    assert_eq!(tokens[3].lexeme, "خطأ");
}

#[test]
fn arabic_handle_keyword() {
    let tokens = common::lex("مقبض");
    assert_eq!(tokens[0].token, TokenType::Handle);
    assert_eq!(tokens[0].lexeme, "مقبض");
}

#[test]
fn arabic_as_keyword() {
    let tokens = common::lex("بصفة");
    assert_eq!(tokens[0].token, TokenType::As);
    assert_eq!(tokens[0].lexeme, "بصفة");
}

#[test]
fn arabic_null_literal() {
    let tokens = common::lex("فارغ");
    assert_eq!(tokens[0].token, TokenType::Null);
    assert_eq!(tokens[0].lexeme, "فارغ");
}

#[test]
fn arabic_or_and_operators() {
    let tokens = common::lex("أ أو ب و ج");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Or);
    assert_eq!(tokens[1].lexeme, "أو");
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
    assert_eq!(tokens[3].token, TokenType::And);
    assert_eq!(tokens[3].lexeme, "و");
    assert_eq!(tokens[4].token, TokenType::Identifier("ج".into()));
}

// ---------------------------------------------------------------------------
// Complex multi-line programs
// ---------------------------------------------------------------------------

#[test]
fn full_arabic_function_with_if_else() {
    let source = r#"دالة مرحبا(اسم: نص) -> نص {
    إذا اسم != فارغ {
        أرجع اسم
    } وإلا {
        أرجع "زائر"
    }
}"#;
    let tokens = common::lex(source);
    let tokens: Vec<_> = tokens.iter()
        .filter(|t| t.token != TokenType::Newline)
        .collect();

    assert_eq!(tokens[0].token, TokenType::Fn);
    assert_eq!(tokens[0].lexeme, "دالة");
    assert_eq!(tokens[1].token, TokenType::Identifier("مرحبا".into()));
    assert_eq!(tokens[5].token, TokenType::String);
    assert_eq!(tokens[5].lexeme, "نص");
    assert_eq!(tokens[10].token, TokenType::If);
    assert_eq!(tokens[10].lexeme, "إذا");
    assert_eq!(tokens[15].token, TokenType::Return);
    assert_eq!(tokens[15].lexeme, "أرجع");
    assert_eq!(tokens[18].token, TokenType::Else);
    assert_eq!(tokens[18].lexeme, "وإلا");
    assert_eq!(tokens[20].token, TokenType::Return);
    assert_eq!(tokens[20].lexeme, "أرجع");
}

#[test]
fn arabic_for_loop_with_range() {
    let tokens = common::lex("لكل عداد في 0..10 { عداد }");
    assert_eq!(tokens[0].token, TokenType::For);
    assert_eq!(tokens[0].lexeme, "لكل");
    assert_eq!(tokens[1].token, TokenType::Identifier("عداد".into()));
    assert_eq!(tokens[3].token, TokenType::NumberLiteral(0));
    assert_eq!(tokens[4].token, TokenType::DotDot);
    assert_eq!(tokens[5].token, TokenType::NumberLiteral(10));
}

#[test]
fn arabic_while_with_comparison() {
    let tokens = common::lex("بينما العداد < 10 { العداد += 1 }");
    assert_eq!(tokens[0].token, TokenType::While);
    assert_eq!(tokens[0].lexeme, "بينما");
    assert_eq!(tokens[1].token, TokenType::Identifier("العداد".into()));
    assert_eq!(tokens[2].token, TokenType::Less);
    assert_eq!(tokens[4].token, TokenType::LeftBrace);
}

#[test]
fn arabic_record_with_multiple_fields() {
    let tokens = common::lex("سجل شخص(الاسم: نص, العمر: عدد, نشط: منطقي)");
    assert_eq!(tokens[0].token, TokenType::Record);
    assert_eq!(tokens[0].lexeme, "سجل");
    assert_eq!(tokens[1].token, TokenType::Identifier("شخص".into()));
    assert_eq!(tokens[3].token, TokenType::Identifier("الاسم".into()));
    assert_eq!(tokens[4].token, TokenType::Colon);
    assert_eq!(tokens[5].token, TokenType::String);
    assert_eq!(tokens[7].token, TokenType::Identifier("العمر".into()));
    assert_eq!(tokens[8].token, TokenType::Colon);
    assert_eq!(tokens[9].token, TokenType::Int);
    assert_eq!(tokens[11].token, TokenType::Identifier("نشط".into()));
    assert_eq!(tokens[12].token, TokenType::Colon);
    assert_eq!(tokens[13].token, TokenType::Bool);
}

#[test]
fn arabic_impl_with_two_methods() {
    let source = "تنفيذ نقطة { دالة أx(self) -> عشري { 0 } دالة أy(self) -> عشري { 0 } }";
    let tokens = common::lex(source);
    let fns: Vec<_> = tokens.iter()
        .filter(|t| t.token == TokenType::Fn && t.lexeme == "دالة")
        .collect();
    assert_eq!(fns.len(), 2);
}

#[test]
fn arabic_tag_with_data_variants() {
    let source = "وسم نتيجة { نجاح(قيمة: عدد) فشل(سبب: نص) }";
    let tokens = common::lex(source);
    assert_eq!(tokens[0].token, TokenType::Tag);
    assert_eq!(tokens[0].lexeme, "وسم");
    assert_eq!(tokens[1].token, TokenType::Result);
    assert_eq!(tokens[1].lexeme, "نتيجة");
    assert_eq!(tokens[3].token, TokenType::Ok);
    assert_eq!(tokens[3].lexeme, "نجاح");
    assert_eq!(tokens[9].token, TokenType::Err);
    assert_eq!(tokens[9].lexeme, "فشل");
}

// ---------------------------------------------------------------------------
// Identifier patterns
// ---------------------------------------------------------------------------

#[test]
fn arabic_identifier_with_underscores() {
    let tokens = common::lex("اسم_المتغير عدد_العناصر");
    assert_eq!(tokens[0].token, TokenType::Identifier("اسم_المتغير".into()));
    assert_eq!(tokens[1].token, TokenType::Identifier("عدد_العناصر".into()));
}

#[test]
fn arabic_identifier_with_numbers() {
    let tokens = common::lex("متغير2");
    assert_eq!(tokens[0].token, TokenType::Identifier("متغير2".into()));
}

#[test]
fn arabic_identifier_next_to_operator() {
    let tokens = common::lex("أ+ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Plus);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

#[test]
fn arabic_identifier_next_to_bracket() {
    let tokens = common::lex("قائمة[0]");
    assert_eq!(tokens[0].token, TokenType::Identifier("قائمة".into()));
    assert_eq!(tokens[1].token, TokenType::LeftBracket);
    assert_eq!(tokens[2].token, TokenType::NumberLiteral(0));
    assert_eq!(tokens[3].token, TokenType::RightBracket);
}

#[test]
fn arabic_identifier_with_dot_access() {
    let tokens = common::lex("نقطة.س");
    assert_eq!(tokens[0].token, TokenType::Identifier("نقطة".into()));
    assert_eq!(tokens[1].token, TokenType::Dot);
    assert_eq!(tokens[2].token, TokenType::Identifier("س".into()));
}

#[test]
fn arabic_identifier_with_double_colon() {
    let tokens = common::lex("قائمة::طول");
    assert_eq!(tokens[0].token, TokenType::Identifier("قائمة".into()));
    assert_eq!(tokens[1].token, TokenType::ColonColon);
    assert_eq!(tokens[2].token, TokenType::Identifier("طول".into()));
}

#[test]
fn arabic_identifier_with_question_mark() {
    let tokens = common::lex("النتيجة?");
    assert_eq!(tokens[0].token, TokenType::Identifier("النتيجة".into()));
    assert_eq!(tokens[1].token, TokenType::Question);
}

#[test]
fn arabic_identifier_with_bang_hash() {
    let tokens = common::lex("!#مدخل");
    assert_eq!(tokens[0].token, TokenType::BangHash);
    assert_eq!(tokens[1].token, TokenType::Identifier("مدخل".into()));
}

#[test]
fn arabic_identifier_with_hash() {
    let tokens = common::lex("#مدخل");
    assert_eq!(tokens[0].token, TokenType::Hash);
    assert_eq!(tokens[1].token, TokenType::Identifier("مدخل".into()));
}

#[test]
fn arabic_identifier_with_arrow() {
    let tokens = common::lex("دالة تحول(أ: عدد) -> نص");
    assert_eq!(tokens[0].token, TokenType::Fn);
    assert_eq!(tokens[7].token, TokenType::Arrow);
    assert_eq!(tokens[8].token, TokenType::String);
    assert_eq!(tokens[8].lexeme, "نص");
}

#[test]
fn arabic_identifier_with_fat_arrow() {
    let tokens = common::lex("حالة => قيمة");
    assert_eq!(tokens[0].token, TokenType::Identifier("حالة".into()));
    assert_eq!(tokens[1].token, TokenType::FatArrow);
    assert_eq!(tokens[2].token, TokenType::Identifier("قيمة".into()));
}

#[test]
fn arabic_identifier_with_range() {
    let tokens = common::lex("أ..ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::DotDot);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

#[test]
fn arabic_identifier_with_minus_arrow() {
    let tokens = common::lex("أ -> ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Arrow);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

#[test]
fn arabic_identifier_with_wildcard() {
    let tokens = common::lex("أ _ ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Wildcard);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

// ---------------------------------------------------------------------------
// Consecutive identifiers
// ---------------------------------------------------------------------------

#[test]
fn consecutive_arabic_identifiers() {
    let tokens = common::lex("أ ب ج د ه");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Identifier("ب".into()));
    assert_eq!(tokens[2].token, TokenType::Identifier("ج".into()));
    assert_eq!(tokens[3].token, TokenType::Identifier("د".into()));
    assert_eq!(tokens[4].token, TokenType::Identifier("ه".into()));
}

#[test]
fn arabic_keyword_not_confused_with_identifier() {
    let tokens = common::lex("إذا حالة == صحيح");
    assert_eq!(tokens[0].token, TokenType::If);
    assert_eq!(tokens[0].lexeme, "إذا");
    assert_eq!(tokens[1].token, TokenType::Identifier("حالة".into()));
}

#[test]
fn arabic_identifier_in_string_not_keyword() {
    let tokens = common::lex("\"إذا صحيح\"");
    assert_eq!(tokens[0].token, TokenType::StringLiteral("إذا صحيح".into()));
}

// ---------------------------------------------------------------------------
// Data structure contexts
// ---------------------------------------------------------------------------

#[test]
fn arabic_identifier_in_tuple() {
    let tokens = common::lex("(أ, ب, ج)");
    assert_eq!(tokens[0].token, TokenType::LeftParen);
    assert_eq!(tokens[1].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[3].token, TokenType::Identifier("ب".into()));
    assert_eq!(tokens[5].token, TokenType::Identifier("ج".into()));
}

#[test]
fn arabic_identifier_in_array_literal() {
    let tokens = common::lex("[أ, ب, ج]");
    assert_eq!(tokens[0].token, TokenType::LeftBracket);
    assert_eq!(tokens[1].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[3].token, TokenType::Identifier("ب".into()));
    assert_eq!(tokens[5].token, TokenType::Identifier("ج".into()));
}

#[test]
fn arabic_identifier_in_map_literal() {
    let tokens = common::lex("{أ: 1, ب: 2}");
    assert_eq!(tokens[0].token, TokenType::LeftBrace);
    assert_eq!(tokens[1].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[2].token, TokenType::Colon);
    assert_eq!(tokens[3].token, TokenType::NumberLiteral(1));
}

#[test]
fn arabic_identifier_after_colon_type_annotation() {
    let tokens = common::lex("أ: عدد");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Colon);
    assert_eq!(tokens[2].token, TokenType::Int);
    assert_eq!(tokens[2].lexeme, "عدد");
}

#[test]
fn arabic_identifier_with_field_name() {
    let tokens = common::lex("الاسم = \"أحمد\"");
    assert_eq!(tokens[0].token, TokenType::Identifier("الاسم".into()));
    assert_eq!(tokens[1].token, TokenType::Assign);
    assert_eq!(tokens[2].token, TokenType::StringLiteral("أحمد".into()));
}

// ---------------------------------------------------------------------------
// Comparison operators with Arabic identifiers
// ---------------------------------------------------------------------------

#[test]
fn arabic_identifiers_with_comparison_operators() {
    let tokens = common::lex("أ == ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Compare);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

#[test]
fn arabic_identifiers_with_not_equal() {
    let tokens = common::lex("أ != ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::BangEqual);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

#[test]
fn arabic_identifiers_with_less() {
    let tokens = common::lex("أ < ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Less);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

#[test]
fn arabic_identifiers_with_greater() {
    let tokens = common::lex("أ > ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Greater);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

#[test]
fn arabic_identifiers_with_less_equal() {
    let tokens = common::lex("أ <= ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::LessEqual);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

#[test]
fn arabic_identifiers_with_greater_equal() {
    let tokens = common::lex("أ >= ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::GreaterEqual);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

// ---------------------------------------------------------------------------
// Assignment operators with Arabic identifiers
// ---------------------------------------------------------------------------

#[test]
fn arabic_identifiers_with_plus_equal() {
    let tokens = common::lex("أ += 1");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::PlusEqual);
    assert_eq!(tokens[2].token, TokenType::NumberLiteral(1));
}

#[test]
fn arabic_identifiers_with_minus_equal() {
    let tokens = common::lex("أ -= 1");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::MinusEqual);
    assert_eq!(tokens[2].token, TokenType::NumberLiteral(1));
}

#[test]
fn arabic_identifiers_with_star_equal() {
    let tokens = common::lex("أ *= 2");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::StarEqual);
    assert_eq!(tokens[2].token, TokenType::NumberLiteral(2));
}

#[test]
fn arabic_identifiers_with_slash_equal() {
    let tokens = common::lex("أ /= 2");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::SlashEqual);
    assert_eq!(tokens[2].token, TokenType::NumberLiteral(2));
}

// ---------------------------------------------------------------------------
// Unary operators with Arabic identifiers
// ---------------------------------------------------------------------------

#[test]
fn arabic_identifier_with_minus_unary() {
    let tokens = common::lex("-أ");
    assert_eq!(tokens[0].token, TokenType::Minus);
    assert_eq!(tokens[1].token, TokenType::Identifier("أ".into()));
}

#[test]
fn arabic_identifier_with_bang_unary() {
    let tokens = common::lex("!ب");
    assert_eq!(tokens[0].token, TokenType::Bang);
    assert_eq!(tokens[1].token, TokenType::Identifier("ب".into()));
}

// ---------------------------------------------------------------------------
// Semicolon, comma, colon, question with Arabic identifiers
// ---------------------------------------------------------------------------

#[test]
fn arabic_identifier_with_comma_separator() {
    let tokens = common::lex("أ, ب, ج");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Comma);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
    assert_eq!(tokens[3].token, TokenType::Comma);
    assert_eq!(tokens[4].token, TokenType::Identifier("ج".into()));
}

#[test]
fn arabic_identifier_with_semicolon() {
    let tokens = common::lex("أ = 1;");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Assign);
    assert_eq!(tokens[2].token, TokenType::NumberLiteral(1));
    assert_eq!(tokens[3].token, TokenType::Semicolon);
}

#[test]
fn arabic_identifier_with_colon() {
    let tokens = common::lex("أ: ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Colon);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

#[test]
fn arabic_identifier_with_question_colon_ternary() {
    let tokens = common::lex("أ ? ب : ج");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Question);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
    assert_eq!(tokens[3].token, TokenType::Colon);
    assert_eq!(tokens[4].token, TokenType::Identifier("ج".into()));
}

// ---------------------------------------------------------------------------
// Braces, brackets, parens with Arabic identifiers
// ---------------------------------------------------------------------------

#[test]
fn arabic_identifier_with_left_brace() {
    let tokens = common::lex("أ { ب }");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::LeftBrace);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
    assert_eq!(tokens[3].token, TokenType::RightBrace);
}

#[test]
fn arabic_identifier_with_left_bracket() {
    let tokens = common::lex("أ[ب]");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::LeftBracket);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
    assert_eq!(tokens[3].token, TokenType::RightBracket);
}

#[test]
fn arabic_identifier_with_parentheses() {
    let tokens = common::lex("أ(ب)");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::LeftParen);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
    assert_eq!(tokens[3].token, TokenType::RightParen);
}

// ---------------------------------------------------------------------------
// Arithmetic operators with Arabic identifiers
// ---------------------------------------------------------------------------

#[test]
fn arabic_identifiers_with_plus() {
    let tokens = common::lex("أ + ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Plus);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

#[test]
fn arabic_identifiers_with_minus() {
    let tokens = common::lex("أ - ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Minus);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

#[test]
fn arabic_identifiers_with_star() {
    let tokens = common::lex("أ * ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Star);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

#[test]
fn arabic_identifiers_with_slash() {
    let tokens = common::lex("أ / ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
    assert_eq!(tokens[1].token, TokenType::Slash);
    assert_eq!(tokens[2].token, TokenType::Identifier("ب".into()));
}

// ---------------------------------------------------------------------------
// Identifier naming patterns
// ---------------------------------------------------------------------------

#[test]
fn arabic_identifier_with_consecutive_letters() {
    let tokens = common::lex("أبجد");
    assert_eq!(tokens[0].token, TokenType::Identifier("أبجد".into()));
}

#[test]
fn arabic_identifier_with_underscore_between_letters() {
    let tokens = common::lex("أ_ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ_ب".into()));
}

#[test]
fn arabic_identifier_with_number_between_letters() {
    let tokens = common::lex("أ1ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ1ب".into()));
}

#[test]
fn arabic_identifier_with_mixed_underscores_numbers() {
    let tokens = common::lex("أ_1_ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ_1_ب".into()));
}

#[test]
fn arabic_identifier_with_consecutive_underscores() {
    let tokens = common::lex("أ__ب");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ__ب".into()));
}

#[test]
fn arabic_identifier_starting_with_underscore() {
    let tokens = common::lex("_أ");
    assert_eq!(tokens[0].token, TokenType::Identifier("_أ".into()));
}

#[test]
fn arabic_identifier_ending_with_underscore() {
    let tokens = common::lex("أ_");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ_".into()));
}

#[test]
fn arabic_identifier_single_letter() {
    let tokens = common::lex("أ");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ".into()));
}

#[test]
fn arabic_identifier_wildcard_is_not_identifier() {
    let tokens = common::lex("_");
    assert_eq!(tokens[0].token, TokenType::Wildcard);
}

#[test]
fn arabic_identifier_with_long_string() {
    let tokens = common::lex("أبجد.getZen.cba");
    assert_eq!(
        tokens[0].token,
        TokenType::Identifier("أبجد".into())
    );
    assert_eq!(tokens[1].token, TokenType::Dot);
    assert_eq!(tokens[2].token, TokenType::Identifier("getZen".into()));
    assert_eq!(tokens[3].token, TokenType::Dot);
    assert_eq!(tokens[4].token, TokenType::Identifier("cba".into()));
}

#[test]
fn arabic_identifier_starting_with_number_is_number_then_identifier() {
    let tokens = common::lex("1أ");
    assert_eq!(tokens[0].token, TokenType::NumberLiteral(1));
    assert_eq!(tokens[1].token, TokenType::Identifier("أ".into()));
}

#[test]
fn arabic_identifier_ending_with_number() {
    let tokens = common::lex("أ1");
    assert_eq!(tokens[0].token, TokenType::Identifier("أ1".into()));
}

#[test]
fn arabic_identifier_only_underscores() {
    let tokens = common::lex("___");
    assert_eq!(tokens[0].token, TokenType::Identifier("___".into()));
}

#[test]
fn arabic_identifier_underscore_number_mix() {
    let tokens = common::lex("_1_2_3");
    assert_eq!(tokens[0].token, TokenType::Identifier("_1_2_3".into()));
}

// ---------------------------------------------------------------------------
// Full complex program
// ---------------------------------------------------------------------------

#[test]
fn full_arabic_record_impl_for_loop_program() {
    let source = r#"سجل نقطة(س: عشري, ص: عشري)
تنفيذ نقطة {
    دالة مسافة(self, أخرى: نقطة) -> عشري {
       	self.س + أخرى.ص
    }
}
ثابت أ = 0
لكل عداد في 0..5 {
    أ += عداد
}"#;
    let tokens = common::lex(source);
    let filtered: Vec<_> = tokens.iter()
        .filter(|t| t.token != TokenType::Newline)
        .collect();

    // First line: سجل نقطة(س: عشري, ص: عشري)
    assert_eq!(filtered[0].token, TokenType::Record);
    assert_eq!(filtered[0].lexeme, "سجل");
    assert_eq!(filtered[1].token, TokenType::Identifier("نقطة".into()));

    // Find implement block
    let impl_idx = filtered.iter().position(|t| t.token == TokenType::Impl).unwrap();
    assert_eq!(filtered[impl_idx].token, TokenType::Impl);
    assert_eq!(filtered[impl_idx].lexeme, "تنفيذ");
    assert_eq!(filtered[impl_idx + 1].token, TokenType::Identifier("نقطة".into()));

    // Find const
    let const_idx = filtered.iter().position(|t| t.token == TokenType::Const).unwrap();
    assert_eq!(filtered[const_idx].lexeme, "ثابت");

    // Find for loop
    let for_idx = filtered.iter().position(|t| t.token == TokenType::For).unwrap();
    assert_eq!(filtered[for_idx].lexeme, "لكل");
    assert_eq!(filtered[for_idx + 1].token, TokenType::Identifier("عداد".into()));
}

// ---------------------------------------------------------------------------
// Arabic in a string literal with Arabic content
// ---------------------------------------------------------------------------

#[test]
fn arabic_string_literal_preserves_content() {
    let tokens = common::lex("\"مرحبا بالعالم\"");
    assert_eq!(
        tokens[0].token,
        TokenType::StringLiteral("مرحبا بالعالم".into())
    );
}

#[test]
fn arabic_char_literal() {
    let tokens = common::lex("'أ'");
    assert_eq!(tokens[0].token, TokenType::CharacterLiteral('أ'));
}

#[test]
#[allow(clippy::approx_constant)]
fn arabic_float_literal() {
    let tokens = common::lex("3.14");
    assert_eq!(tokens[0].token, TokenType::FloatLiteral(3.14));
}

#[test]
fn arabic_number_literal() {
    let tokens = common::lex("42");
    assert_eq!(tokens[0].token, TokenType::NumberLiteral(42));
}

#[test]
fn arabic_bool_true() {
    let tokens = common::lex("صحيح");
    assert_eq!(tokens[0].token, TokenType::BoolLiteral(true));
    assert_eq!(tokens[0].lexeme, "صحيح");
}

#[test]
fn arabic_bool_false() {
    let tokens = common::lex("ليس_صحيح");
    assert_eq!(tokens[0].token, TokenType::BoolLiteral(false));
    assert_eq!(tokens[0].lexeme, "ليس_صحيح");
}

// ---------------------------------------------------------------------------
// Eof is always last
// ---------------------------------------------------------------------------

#[test]
fn arabic_lex_produces_eof_last() {
    let tokens = common::lex("أ + ب");
    assert_eq!(tokens.last().unwrap().token, TokenType::Eof);
}

#[test]
fn arabic_keyword_lex_produces_eof_last() {
    let tokens = common::lex("إذا");
    assert_eq!(tokens.last().unwrap().token, TokenType::Eof);
}

// ---------------------------------------------------------------------------
// Multiple Arabic keywords in sequence
// ---------------------------------------------------------------------------

#[test]
fn multiple_arabic_keywords_sequence() {
    let tokens = common::lex("إذا صحيح وإلا فارغ");
    assert_eq!(tokens[0].token, TokenType::If);
    assert_eq!(tokens[1].token, TokenType::BoolLiteral(true));
    assert_eq!(tokens[2].token, TokenType::Else);
    assert_eq!(tokens[3].token, TokenType::Null);
}

#[test]
fn arabic_loop_break_continue_sequence() {
    let tokens = common::lex("تكرار { إذا صح { توقف } وإلا { استمر } }");
    assert_eq!(tokens[0].token, TokenType::Loop);
    assert_eq!(tokens[2].token, TokenType::If);
    assert_eq!(tokens[3].token, TokenType::Identifier("صح".into()));
    assert_eq!(tokens[5].token, TokenType::Break);
    assert_eq!(tokens[7].token, TokenType::Else);
    assert_eq!(tokens[9].token, TokenType::Continue);
}

#[test]
fn arabic_fn_return_value() {
    let tokens = common::lex("دالة ج(أ: عدد) -> عدد { أرجع 42 }");
    assert_eq!(tokens[0].token, TokenType::Fn);
    assert_eq!(tokens[10].token, TokenType::Return);
    assert_eq!(tokens[11].token, TokenType::NumberLiteral(42));
}

#[test]
fn arabic_const_with_null() {
    let tokens = common::lex("ثابت x = فارغ");
    assert_eq!(tokens[0].token, TokenType::Const);
    assert_eq!(tokens[3].token, TokenType::Null);
}

#[test]
fn arabic_match_with_ok_and_err() {
    let tokens = common::lex("طابق ر { نجاح => 1, فشل => 0 }");
    assert_eq!(tokens[0].token, TokenType::Match);
    assert_eq!(tokens[3].token, TokenType::Ok);
    assert_eq!(tokens[7].token, TokenType::Err);
}
