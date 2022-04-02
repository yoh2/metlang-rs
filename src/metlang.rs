use libbf::prelude::*;

pub const TOKEN_SPEC: SimpleTokenSpec<
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
> = SimpleTokenSpec {
    ptr_inc: "にゃうね",
    ptr_dec: "にゃん",
    data_inc: "にゃう",
    data_dec: "にゃ？",
    output: "これになりたい",
    input: "これすき",
    loop_head: "ポチった",
    loop_tail: "ねる",
};

pub fn tokenizer() -> SimpleTokenizer {
    TOKEN_SPEC.to_tokenizer()
}
