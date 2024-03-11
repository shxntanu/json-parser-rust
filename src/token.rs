pub enum TokenType {
    // Structural Tokens
    OpenBrace,      // {
    CloseBrace,     // }
    OpenSqBracket,  // [
    CloseSqBracket, // ]
    Colon,          // :
    Comma,          // ,

    // Value Tokens
    String, // "string"
    Number, // 123 or 12.34
    True,   // true
    False,  // false
    Null,   // null
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}
