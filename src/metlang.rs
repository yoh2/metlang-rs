use libbf::prelude::*;

const TOKEN_SPEC: SimpleTokenSpec1<&str> = SimpleTokenSpec {
    ptr_inc: "にゃうね",
    ptr_dec: "にゃん",
    data_inc: "にゃう",
    data_dec: "にゃ？",
    output: "これになりたい",
    input: "これすき",
    loop_head: "ポチった",
    loop_tail: "ねる",
};

pub fn parser() -> Parser<SimpleTokenizer> {
    Parser::new(TOKEN_SPEC.to_tokenizer())
}
