
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use rts::bytes::Bytes;
use rts::RTS;
use rts::structure::{StructureMut, StructureType};
use rts::token::{Token, TokenType};

fn main() 
{
  let rts: RTS = RTS::new(String::from("project"));

  rts.newStructure(
    String::from("test"),
    StructureMut::Constant,
    StructureType::String,
    vec![
      Token::new(
        TokenType::String,
        Bytes::new(String::from("test var"))
      )
    ]
  );
  
  rts.newStructure(
    String::from("method"),
    StructureMut::Constant,
    StructureType::Method,
    vec![
      Token::new(
        TokenType::Native,
        RTS::getNative(nativeTest)
      )
    ]
  );
  rts.run(r#"
println(project.test, type(project.test), mut(project.test))
println('-')

println(project.method, type(project.method), mut(project.method))
println('-')

project.method(10+10, 321)
"#);
}

extern "C" fn nativeTest(args: &[Token]) {
  println!("Received {} arguments:", args.len());
  for (i, arg) in args.iter().enumerate() {
    println!("  Arg {}: {}", i, arg.getData().toString().unwrap());
  }
}
