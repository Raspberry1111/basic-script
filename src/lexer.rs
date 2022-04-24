use logos::Logos;

/*
var
let
if
else
goto
label
func
return
using
*/

#[derive(Logos, Debug, PartialEq)]
enum Token {
	#[token("var")]
	Var,
	#[token("let")]
	Let,
	#[token("if")]
	If,
	#[token("else")]
	Else,
	#[token("goto")]
	Goto,
	#[token("label")]
	Label,
	#[token("func")]
	Func,
	#[token("return")]
	Return,
	#[token("using")]
	Using,
	#[token(";")]
	Semicolon,
	#[token(":")]
	Colon,
	#[token(",")]
	Comma,
	#[token("\n")]
	Newln,
	#[regex("[0-9.]+")]
	Num,
	#[regex("[_/a-zA-Z]+")]
	Text,
	#[token("+")]
	Add,
	#[token("-")]
	Sub,
	#[token("%")]
	Mod,
	#[token("=")]
	Equ,
	#[token(">")]
	Great,
	#[token("<")]
	Less,
	#[regex(r#"("[^"]*")|('[^']*')"#)]
	String,

	#[error]
	// We can also use this variant to define whitespace,
	// or any other matches we wish to skip.
	#[regex(r"(//.*)|(/\*[^*]*\*/)", logos::skip)]
	#[regex(r"[ \t\n\f]+", logos::skip)]
	Error,
}

#[cfg(test)]
mod test{
	use super::*;
	#[test]
	fn test_lexer() {
		let mut lex = Token::lexer(include_str!("spec.bs"));

		for i in lex {
			println!("{i:?}")
		}
	}
}
