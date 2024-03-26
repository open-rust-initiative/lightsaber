use serde_yaml;
use std::fs::File;
use std::io::{Read, Write};

fn gen_lexer_g4() {
    let lexer = r##"lexer grammar lsLexer;

LET: 'let';
BEGIN: 'begin';
END: 'end';

FUNC: 'func';
FN: 'fn';
DEF: 'def';

CONST: 'const';
INT: 'int';
VOID: 'void';
IF: 'if';
ELSE: 'else';
WHILE: 'while';
BREAK: 'break';
CONTINUE: 'continue';
RETURN: 'return';
PLUS: '+';
MINUS: '-';
MUL: '*';
DIV: '/';
MOD: '%';
ASSIGN: '=';
EQ: '==';
NEQ: '!=';
LT: '<';
GT: '>';
LE: '<=';
GE: '>=';
NOT: '!';
AND: '&&';
OR: '||';
L_PAREN: '(';
R_PAREN: ')';
L_BRACE: '{';
R_BRACE: '}';
L_BRACKT: '[';
R_BRACKT: ']';
COMMA: ',';
SEMICOLON: ';';

IDENT: [a-zA-Z_] [a-zA-Z0-9_]*;
INTEGER_CONST: ('0x' | '0X') [0-9a-fA-F]+
    | '0' [0-7]*
    | [1-9] [0-9]*;
WS: [ \r\n\t]+ -> skip;
LINE_COMMENT: '//' ~[\r\n]* -> skip;
MULTILINE_COMMENT: '/*' .*? '*/' -> skip;"##;

    let mut file = File::create("src/gen/lsLexer.g4").expect("Failed to create file");
    file.write_all(lexer.as_bytes())
        .expect("Failed to write to file");
}

fn gen_parser_g4(config: &serde_yaml::Value) {
    let var_decl = config["var_decl"].as_str().unwrap_or("var_decl missing");
    let var_decl = match var_decl {
        "let" => "varDecl: LET varDef SEMICOLON;",
        "var" => "varDecl: VAR varDef SEMICOLON;",
        "empty" => "",
        _ => panic!("Invalid value for var_decl"),
    };

    let block = config["block"].as_str().unwrap_or("block missing");
    let block = match block {
        "begin-end" => "block: BEGIN (blockItem)* END;",
        "brace" => "block: L_BRACE (blockItem)* R_BRACE;",
        _ => panic!("Invalid value for block"),
    };

    let func_def = config["func_def"].as_str().unwrap_or("func_def missing");
    let func_def = match func_def {
        "func" => "funcDef: FUNC IDENT L_PAREN (funcFParams)? R_PAREN block;",
        "fn" => "funcDef: FN IDENT L_PAREN (funcFParams)? R_PAREN block;",
        "def" => "funcDef: DEF IDENT L_PAREN (funcFParams)? R_PAREN block;",
        _ => panic!("Invalid value for func_def"),
    };

    let parser_template = r##"parser grammar lsParser;

options {
    tokenVocab = lsLexer;
}

program: compUnit;
compUnit: (funcDef | decl)+ EOF;

decl: varDecl;

bType: INT;

#var_decl#

varDef: (IDENT (L_BRACKT constExp R_BRACKT)*)
    | (IDENT (L_BRACKT constExp R_BRACKT)* ASSIGN initVal);

initVal: exp | (L_BRACE (initVal (COMMA initVal)*)? R_BRACE);

#func_def#

funcType: VOID | INT;

funcFParams: funcFParam (COMMA funcFParam)*;

funcFParam:
    bType IDENT (L_BRACKT R_BRACKT (L_BRACKT exp R_BRACKT)*)?;

#block#

blockItem: decl | stmt;

stmt:
    (lVal ASSIGN exp SEMICOLON)
    | (exp? SEMICOLON)
    | block
    | (IF L_PAREN cond R_PAREN stmt (ELSE stmt)?)
    | (WHILE L_PAREN cond R_PAREN stmt)
    | (BREAK SEMICOLON)
    | (CONTINUE SEMICOLON)
    | (RETURN exp? SEMICOLON);

exp:
    L_PAREN exp R_PAREN
    | lVal
    | number
    | IDENT L_PAREN funcRParams? R_PAREN
    | unaryOp exp
    | exp (MUL | DIV | MOD) exp
    | exp (PLUS | MINUS) exp;

cond:
    exp
    | cond (LT | GT | LE | GE) cond
    | cond (EQ | NEQ) cond
    | cond AND cond
    | cond OR cond;

lVal: IDENT (L_BRACKT exp R_BRACKT)*;

number: INTEGER_CONST;

unaryOp: PLUS | MINUS | NOT;

funcRParams: param (COMMA param)*;

param: exp;

constExp: exp;"##;

    let parser = parser_template
        .replace("#var_decl#", &var_decl)
        .replace("#block#", &block)
        .replace("#func_def#", &func_def);

    let mut file = File::create("src/gen/lsParser.g4").expect("Failed to create file");
    file.write_all(parser.as_bytes())
        .expect("Failed to write to file");
}

fn main() {
    let mut file = File::open("lightsaber.yaml").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let yaml: serde_yaml::Value = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

    let grammar_yaml: &serde_yaml::Value =
        yaml.get("grammar").expect("no grammar part in config file");

    println!("{:?}", grammar_yaml);

    gen_lexer_g4();
    gen_parser_g4(grammar_yaml);
}
