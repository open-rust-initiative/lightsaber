mod gen;
use std::fs;

use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::tree::ParseTreeListener;
use antlr_rust::InputStream;
use antlr_rust::{common_token_stream::CommonTokenStream, tree::ParseTree};
use gen::lslexer::lsLexer;
use gen::lsparser::lsParser;

use crate::gen::lsparser::ruleNames;
use crate::gen::{
    lsparser::{lsParserContext, lsParserContextType},
    lsparserlistener::lsParserListener,
};

fn main() {
    let src_file = fs::read_to_string("test.ls").expect("cannot read file");

    let tf = CommonTokenFactory::default();
    let mut _lexer = lsLexer::new_with_token_factory(InputStream::new(src_file.as_str()), &tf);
    let token_source = CommonTokenStream::new(_lexer);

    let mut parser = lsParser::new(token_source);

    struct Listener {}
    impl<'input> ParseTreeListener<'input, lsParserContextType> for Listener {
        fn enter_every_rule(&mut self, ctx: &dyn lsParserContext<'input>) {
            println!(
                "rule entered {}",
                ruleNames.get(ctx.get_rule_index()).unwrap_or(&"error")
            )
        }
    }
    impl<'input> lsParserListener<'input> for Listener {}

    parser.add_parse_listener(Box::new(Listener {}));
    let result = parser.program();
    println!("{}", result.unwrap().to_string_tree(&*parser));
}
