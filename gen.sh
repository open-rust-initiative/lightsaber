set -e

cargo run --bin g4gen
cd src/gen
rm ls*.rs
java -jar antlr4-4.8-2-SNAPSHOT-complete.jar -visitor -Dlanguage=Rust lsLexer.g4
java -jar antlr4-4.8-2-SNAPSHOT-complete.jar -visitor -Dlanguage=Rust lsParser.g4
rm ls*.interp ls*.tokens
cd ../..
cargo run --bin lightsaber


# 1. 给出 lightsaber.yaml 运行 g4 generator
# 2. java 运行 antlr4 (antlr4rust.jar) 生成 parser.rs
# 3. parser.rs + lib.rs = 用户可以使用的 crate
