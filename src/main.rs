pub mod ir;
pub mod llvm;
pub mod types;

fn main() {
    println!("{:#?}", llvm::grammar::FunctionParser::new().parse(r#"define i32 @get_inode_block_size(ptr %address) {
    %i_size_ptr = getelementptr i8, ptr %address, i32 4
    %i_size_swapped = load i32, ptr %i_size_ptr
    %i_size = call i32 @reverse_word(i32 %i_size_swapped)

    %block_size = load i32, ptr @block_size

    %0 = add i32 %block_size, %i_size
    %1 = sub i32 1, %0

    %size = udiv i32 %1, %block_size
    ret i32 %size
}"#).unwrap());
}
