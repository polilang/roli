mod roli;
use roli::*;

fn main() {
    let test = r#"
u32: sum (u32: a, u32: b)
  return a + b
    "#;

    let lexer = lexer(&mut test.chars());
    
    for stuff in lexer {
        println!("{:#?}", stuff)
    }
}
