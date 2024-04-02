use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // TODO: support for identifiers with quoted names and escape sequences in identifier names
    #[regex("[%@][-a-zA-Z$._][-a-zA-Z$._0-9]*")]
    #[regex("[%@][0-9]+")]
    Identifier,
}

fn main() {
    println!("Hello, world!");
}
