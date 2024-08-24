use crate::{
    ast::{assert_exhausted, match_next, unreachable_grammar, AstBuildResult, AstNode},
    parser::{PairsExt, Rule},
};
use lexical::{NumberFormatBuilder, ParseFloatOptions};
use pest::iterators::{Pair, Pairs};
use std::num::NonZeroU8;

#[derive(Debug)]
pub struct BoolLit {
    pub val: bool,
}

#[derive(Debug)]
pub struct FloatLit {
    pub val: f64,
}

#[derive(Debug)]
pub struct IntLit {
    pub neg: bool,
    pub val: u64,
}

#[derive(Debug)]
pub struct EscSeq {
    pub val: Box<str>,
}

#[derive(Debug)]
pub struct ChrLit {
    pub val: char,
}

#[derive(Debug)]
pub struct StrLit {
    pub val: String,
}

impl AstNode for BoolLit {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::bool).into_inner();

        let val = match pairs.next().as_ref().map(Pair::as_rule) {
            Some(Rule::kw_true) => true,
            Some(Rule::kw_false) => false,
            _ => unreachable_grammar!(Self),
        };

        assert_exhausted!(pairs, Self);
        Ok(Some(Self { val }))
    }
}

const NUM_FMT: NumberFormatBuilder = NumberFormatBuilder::new()
    .digit_separator(NonZeroU8::new(b'_'))
    .no_exponent_notation(true);

const HEX_FMT: u128 = NUM_FMT.radix(16).base_prefix(NonZeroU8::new(b'x')).build();
const DEC_FMT: u128 = NUM_FMT.radix(10).base_prefix(NonZeroU8::new(b'd')).build();
const OCT_FMT: u128 = NUM_FMT.radix(8).base_prefix(NonZeroU8::new(b'o')).build();
const BIN_FMT: u128 = NUM_FMT.radix(2).base_prefix(NonZeroU8::new(b'b')).build();

impl AstNode for FloatLit {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let pair = match_next!(pairs, Rule::float);
        let bytes = pair.as_str();

        let mut pairs = pair.into_inner();
        let _sign = pairs.next_if(Rule::num_sign);

        let opts = ParseFloatOptions::builder()
            .decimal_point(b'.')
            .lossy(true)
            .build()
            .unwrap();

        let parser = match pairs.peek().as_ref().map(Pair::as_rule) {
            Some(Rule::hex_prefix) => lexical::parse_with_options::<_, _, HEX_FMT>,
            Some(Rule::oct_prefix) => lexical::parse_with_options::<_, _, OCT_FMT>,
            Some(Rule::bin_prefix) => lexical::parse_with_options::<_, _, BIN_FMT>,
            _ => lexical::parse_with_options::<_, _, DEC_FMT>,
        };

        let val = parser(bytes, &opts).unwrap();

        Ok(Some(Self { val }))
    }
}

impl AstNode for IntLit {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::int).into_inner();

        let sign = pairs
            .next_if(Rule::num_sign)
            .map(Pair::into_inner)
            .as_mut()
            .and_then(Iterator::next)
            .as_ref()
            .map(Pair::as_rule);

        let neg = match sign {
            Some(Rule::num_neg) => true,
            Some(Rule::num_pos) | None => false,
            _ => unreachable_grammar!(Self),
        };

        let radix_prefix = match pairs.peek().as_ref().map(Pair::as_rule) {
            Some(Rule::hex_prefix) => 16,
            Some(Rule::dec_prefix) => 10,
            Some(Rule::oct_prefix) => 8,
            Some(Rule::bin_prefix) => 2,
            _ => 0,
        };

        let radix = if radix_prefix != 0 {
            pairs.next();
            radix_prefix
        } else {
            10
        };

        let val_str = pairs
            .next()
            .unwrap_or_else(|| unreachable_grammar!(Self))
            .as_str();

        let val = u64::from_str_radix(val_str, radix).unwrap();

        assert_exhausted!(pairs, Self);
        Ok(Some(Self { neg, val }))
    }
}

impl AstNode for EscSeq {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::esc_seq).into_inner();
        let pair = pairs.next().unwrap_or_else(|| unreachable_grammar!(Self));
        let val = match pair.as_rule() {
            Rule::esc_lit => match pair.as_str() {
                "t" => "\t".into(),
                "r" => "\r".into(),
                "n" => "\n".into(),
                "'" => "'".into(),
                "\"" => "\"".into(),
                "\\" => "\\".into(),
                _ => unreachable_grammar!(Self),
            },
            _ => unreachable_grammar!(Self),
        };

        assert_exhausted!(pairs, Self);
        Ok(Some(Self { val }))
    }
}

impl AstNode for ChrLit {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::chr).into_inner();

        let esc = EscSeq::parse(&mut pairs)?;
        let content = pairs.next_if(Rule::chr_content);

        let lit = esc
            .as_ref()
            .map(|esc| esc.val.as_ref())
            .or(content.as_ref().map(Pair::as_str))
            .unwrap_or_else(|| unreachable_grammar!(Self));

        let mut chars = lit.chars();
        let Some(val) = chars.next() else {
            unreachable_grammar!(Self);
        };

        debug_assert!(chars.next().is_none());
        assert_exhausted!(pairs, Self);

        Ok(Some(Self { val }))
    }
}

impl AstNode for StrLit {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let pairs = match_next!(pairs, Rule::str).into_inner();
        let mut val = String::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::str_content => val.push_str(pair.as_str()),
                Rule::esc_seq => {
                    let esc = EscSeq::expect(&mut Pairs::single(pair))?;
                    val.push_str(esc.val.as_ref());
                }
                _ => unreachable_grammar!(Self),
            }
        }

        Ok(Some(Self { val }))
    }
}
