mod ast;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub pancake);

fn main() {
    let f = "fun asd() { var rx_return = 1; }";
    println!("{:?}", pancake::FunctionParser::new().parse(f));
}

#[test]
fn expr_test() {
    assert!(pancake::ExprParser::new().parse("22").is_ok());
    assert!(pancake::ExprParser::new().parse("23 == 23").is_ok());
    assert!(pancake::ExprParser::new().parse("23 >= (23 + 12)").is_ok());
}

#[test]
fn param_test() {
    assert!(pancake::ParamParser::new().parse("1 rx_return").is_ok());
}


#[test]
fn params_test() {
    assert!(pancake::ParamsParser::new().parse("1 rx_return , 1 tx_return").is_ok());
}


#[test]
fn stmt_test() {
    assert!(pancake::StmtParser::new().parse("var rx_return = 1;").is_ok());
}


#[test]
fn fn_test() {
}
