use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Skip whitespace
pub enum Token {
    // Keywords
    #[token("var")]
    Var,
    #[token("mut")]
    Mut,
    #[token("fn")]
    Fn,
    #[token("const")]
    Const,
    #[token("struct")]
    Struct,
    #[token("enum")]
    Enum,
    #[token("union")]
    Union,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("loop")]
    Loop,
    #[token("match")]
    Match,
    #[token("mod")]
    Mod,
    #[token("return")]
    Return,
    #[token("panic")]
    Panic,

    // Built-in Types
    #[token("i8")]
    I8,
    #[token("i16")]
    I16,
    #[token("i32")]
    I32,
    #[token("i64")]
    I64,
    #[token("i128")]
    I128,
    #[token("u8")]
    U8,
    #[token("u16")]
    U16,
    #[token("u32")]
    U32,
    #[token("u64")]
    U64,
    #[token("u128")]
    U128,
    #[token("f32")]
    F32,
    #[token("f64")]
    F64,
    #[token("isize")]
    Isize,
    #[token("usize")]
    Usize,
    #[token("bool")]
    Bool,
    #[token("char")]
    Char,
    #[token("str")]
    Str,

    // Literals
    #[regex(r"[0-9]+")]
    IntegerLiteral,
    #[regex(r"[0-9]+\.[0-9]+")]
    FloatLiteral,
    #[regex(r#""[^"]*""#)]
    StringLiteral,
    #[regex(r"'[^']'")]
    CharLiteral,
    #[token("true")]
    True,
    #[token("false")]
    False,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token("!=")]
    NotEq,
    #[token("<")]
    Lt,
    #[token("<=")]
    LtEq,
    #[token(">")]
    Gt,
    #[token(">=")]
    GtEq,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,
    #[token("&")]
    BitAnd,
    #[token("|")]
    BitOr,
    #[token("^")]
    BitXor,
    #[token("~")]
    BitNot,
    #[token("<<")]
    Shl,
    #[token(">>")]
    Shr,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("->")]
    Arrow,

    #[regex(r"//[^\n]*")]
    SingleLineComment,
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    MultiLineComment,

    #[regex(r"#\[[^\]]*\]")]
    Attribute,

    #[token("@")]
    MacroInvoke,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Var
            | Token::Mut
            | Token::Fn
            | Token::Const
            | Token::Struct
            | Token::Enum
            | Token::Union
            | Token::If
            | Token::Else
            | Token::While
            | Token::For
            | Token::Loop
            | Token::Match
            | Token::Mod
            | Token::Return
            | Token::Panic
            | Token::True
            | Token::False => write!(f, "{:?}", self),

            Token::I8
            | Token::I16
            | Token::I32
            | Token::I64
            | Token::I128
            | Token::U8
            | Token::U16
            | Token::U32
            | Token::U64
            | Token::U128
            | Token::F32
            | Token::F64
            | Token::Isize
            | Token::Usize
            | Token::Bool
            | Token::Char
            | Token::Str => write!(f, "{:?}", self),

            Token::IntegerLiteral => f.write_str("IntegerLiteral"),
            Token::FloatLiteral => f.write_str("FloatLiteral"),
            Token::StringLiteral => f.write_str("StringLiteral"),
            Token::CharLiteral => f.write_str("CharLiteral"),
            Token::Identifier => f.write_str("Identifier"),

            Token::Plus => f.write_str("+"),
            Token::Minus => f.write_str("-"),
            Token::Star => f.write_str("*"),
            Token::Slash => f.write_str("/"),
            Token::Percent => f.write_str("%"),
            Token::Assign => f.write_str("="),
            Token::Eq => f.write_str("=="),
            Token::NotEq => f.write_str("!="),
            Token::Lt => f.write_str("<"),
            Token::LtEq => f.write_str("<="),
            Token::Gt => f.write_str(">"),
            Token::GtEq => f.write_str(">="),
            Token::And => f.write_str("&&"),
            Token::Or => f.write_str("||"),
            Token::Not => f.write_str("!"),
            Token::BitAnd => f.write_str("&"),
            Token::BitOr => f.write_str("|"),
            Token::BitXor => f.write_str("^"),
            Token::BitNot => f.write_str("~"),
            Token::Shl => f.write_str("<<"),
            Token::Shr => f.write_str(">>"),

            Token::LParen => f.write_str("("),
            Token::RParen => f.write_str(")"),
            Token::LBrace => f.write_str("{"),
            Token::RBrace => f.write_str("}"),
            Token::LBracket => f.write_str("["),
            Token::RBracket => f.write_str("]"),
            Token::Semicolon => f.write_str(";"),
            Token::Colon => f.write_str(":"),
            Token::Comma => f.write_str(","),
            Token::Dot => f.write_str("."),
            Token::Arrow => f.write_str("->"),

            Token::SingleLineComment => f.write_str("SingleLineComment"),
            Token::MultiLineComment => f.write_str("MultiLineComment"),
            Token::Attribute => f.write_str("Attribute"),
            Token::MacroInvoke => f.write_str("@"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    #[test]
    fn test_keywords() {
        let mut lex = Token::lexer(
            "var fn const struct enum union if else while for loop match
 mod return panic",
        );
        assert_eq!(lex.next(), Some(Ok(Token::Var)));
        assert_eq!(lex.next(), Some(Ok(Token::Fn)));
        assert_eq!(lex.next(), Some(Ok(Token::Const)));
        assert_eq!(lex.next(), Some(Ok(Token::Struct)));
        assert_eq!(lex.next(), Some(Ok(Token::Enum)));
        assert_eq!(lex.next(), Some(Ok(Token::Union)));
        assert_eq!(lex.next(), Some(Ok(Token::If)));
        assert_eq!(lex.next(), Some(Ok(Token::Else)));
        assert_eq!(lex.next(), Some(Ok(Token::While)));
        assert_eq!(lex.next(), Some(Ok(Token::For)));
        assert_eq!(lex.next(), Some(Ok(Token::Loop)));
        assert_eq!(lex.next(), Some(Ok(Token::Match)));
        assert_eq!(lex.next(), Some(Ok(Token::Mod)));
        assert_eq!(lex.next(), Some(Ok(Token::Return)));
        assert_eq!(lex.next(), Some(Ok(Token::Panic)));
    }

    #[test]
    fn test_types() {
        let mut lex = Token::lexer("i32 f64 bool char str");
        assert_eq!(lex.next(), Some(Ok(Token::I32)));
        assert_eq!(lex.next(), Some(Ok(Token::F64)));
        assert_eq!(lex.next(), Some(Ok(Token::Bool)));
        assert_eq!(lex.next(), Some(Ok(Token::Char)));
        assert_eq!(lex.next(), Some(Ok(Token::Str)));
    }

    #[test]
    fn test_literals() {
        let mut lex = Token::lexer(r#"42 3.14 "hello" 'c' true false"#);
        assert_eq!(lex.next(), Some(Ok(Token::IntegerLiteral)));
        assert_eq!(lex.next(), Some(Ok(Token::FloatLiteral)));
        assert_eq!(lex.next(), Some(Ok(Token::StringLiteral)));
        assert_eq!(lex.next(), Some(Ok(Token::CharLiteral)));
        assert_eq!(lex.next(), Some(Ok(Token::True)));
        assert_eq!(lex.next(), Some(Ok(Token::False)));
    }

    #[test]
    fn test_identifiers() {
        let mut lex = Token::lexer("variable_name _test test123");
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    }

    #[test]
    fn test_operators() {
        let mut lex = Token::lexer(
            "+ - * / % = == != < <= > 
>= && || ! & | ^ ~ << >>",
        );
        assert_eq!(lex.next(), Some(Ok(Token::Plus)));
        assert_eq!(lex.next(), Some(Ok(Token::Minus)));
        assert_eq!(lex.next(), Some(Ok(Token::Star)));
        assert_eq!(lex.next(), Some(Ok(Token::Slash)));
        assert_eq!(lex.next(), Some(Ok(Token::Percent)));
        assert_eq!(lex.next(), Some(Ok(Token::Assign)));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::NotEq)));
        assert_eq!(lex.next(), Some(Ok(Token::Lt)));
        assert_eq!(lex.next(), Some(Ok(Token::LtEq)));
        assert_eq!(lex.next(), Some(Ok(Token::Gt)));
        assert_eq!(lex.next(), Some(Ok(Token::GtEq)));
        assert_eq!(lex.next(), Some(Ok(Token::And)));
        assert_eq!(lex.next(), Some(Ok(Token::Or)));
        assert_eq!(lex.next(), Some(Ok(Token::Not)));
        assert_eq!(lex.next(), Some(Ok(Token::BitAnd)));
        assert_eq!(lex.next(), Some(Ok(Token::BitOr)));
        assert_eq!(lex.next(), Some(Ok(Token::BitXor)));
        assert_eq!(lex.next(), Some(Ok(Token::BitNot)));
        assert_eq!(lex.next(), Some(Ok(Token::Shl)));
        assert_eq!(lex.next(), Some(Ok(Token::Shr)));
    }

    #[test]
    fn test_delimiters() {
        let mut lex = Token::lexer("( ) { } [ ] ; : , . ->");
        assert_eq!(lex.next(), Some(Ok(Token::LParen)));
        assert_eq!(lex.next(), Some(Ok(Token::RParen)));
        assert_eq!(lex.next(), Some(Ok(Token::LBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::RBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::LBracket)));
        assert_eq!(lex.next(), Some(Ok(Token::RBracket)));
        assert_eq!(lex.next(), Some(Ok(Token::Semicolon)));
        assert_eq!(lex.next(), Some(Ok(Token::Colon)));
        assert_eq!(lex.next(), Some(Ok(Token::Comma)));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Arrow)));
    }

    #[test]
    fn test_comments() {
        let mut lex = Token::lexer(
            "// This is a single line comment\n/* This is 
a\nmultiline comment */",
        );
        assert_eq!(lex.next(), Some(Ok(Token::SingleLineComment)));
        assert_eq!(lex.next(), Some(Ok(Token::MultiLineComment)));
    }

    #[test]
    fn test_attributes_and_macros() {
        let mut lex = Token::lexer("#[inline] @macro_call");
        assert_eq!(lex.next(), Some(Ok(Token::Attribute)));
        assert_eq!(lex.next(), Some(Ok(Token::MacroInvoke)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    }

    #[test]
    fn test_complex_code() {
        let code = r#"
fn main() -> i32 {
    var mut count: i32 = 0;
    // Count up to 10
    while count < 10 {
        count = count + 1;
    }
    return count;
}
"#;
        let lex = Token::lexer(code);
        let tokens: Vec<Token> = lex.collect::<Result<_, _>>().unwrap();

        // cargo test -- --nocapture
        println!("hi!");
        println!("{:?}", tokens);

        assert!(!tokens.is_empty());

        assert!(tokens.contains(&Token::Fn));
        assert!(tokens.contains(&Token::LBrace));
        assert!(tokens.contains(&Token::RBrace));
        assert!(tokens.contains(&Token::I32));
        assert!(tokens.contains(&Token::Return));
    }
}
