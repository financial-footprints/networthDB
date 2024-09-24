use super::types::{BankId, Parser, Statement};

mod hdfcind;
mod hdfcind_test;
mod icicind;

pub fn get_all_parsers() -> Vec<Parser> {
    let mut parsers = Vec::new();

    // Add New Parsers Here
    parsers.push(hdfcind::get_parser());
    parsers.push(icicind::get_parser());

    return parsers;
}
