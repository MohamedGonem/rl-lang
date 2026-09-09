use rl_lexer::tokentypes::TokenType;

pub fn token_to_c_op(token: &TokenType) -> &'static str {
    match token {
        TokenType::Plus => "+",
        TokenType::Minus => "-",
        TokenType::Star => "*",
        TokenType::Slash => "/",
        TokenType::Compare => "==",
        TokenType::BangEqual => "!=",
        TokenType::Less => "<",
        TokenType::LessEqual => "<=",
        TokenType::Greater => ">",
        TokenType::GreaterEqual => ">=",
        TokenType::And => "&&",
        TokenType::Or => "||",
        TokenType::Assign => "=",
        TokenType::PlusEqual => "+=",
        TokenType::MinusEqual => "-=",
        TokenType::StarEqual => "*=",
        TokenType::SlashEqual => "/=",
        _ => "/* op */",
    }
}
