//! Demonstration of the 🛠️ pipefit glyph parsing
//! 
//! Shows how the FlameLang lexer and parser handle the pipefit glyph

use flamelang::{Lexer, Parser};

fn main() {
    println!("🔥 FlameLang Pipefit Glyph Demo");
    println!("{}", "=".repeat(70));
    
    // Example FlameLang code with pipefit glyph
    let code = r#"
        🛠️ pipefit(ALG-001)
        🛠️ pipefit(ALG-016, target: ALG-001)
    "#;
    
    println!("\n📝 FlameLang Code:");
    println!("{}", code);
    
    println!("\n🔍 Lexer Token Stream:");
    println!("{}", "=".repeat(70));
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        println!("  {:?}", token);
        if matches!(token, flamelang::lexer::Token::Eof) {
            break;
        }
        tokens.push(token);
    }
    
    println!("\n🌳 Parser AST:");
    println!("{}", "=".repeat(70));
    let mut parser = Parser::new(code);
    let ast = parser.parse_program();
    println!("  {:?}", ast);
    
    println!("\n✨ Pipefit glyph successfully parsed!");
    println!("   The compiler can now compute cortex node properties at compile-time");
    println!("   using the stdlib::pipefitter module functions.");
}
