pub mod ir;
pub mod llvm;
pub mod types;

fn main() {
    println!("{:#?}", llvm::grammar::BasicBlockParser::new().parse("call i32 @puts(ptr @.str)\nret i32 0").unwrap());
}
