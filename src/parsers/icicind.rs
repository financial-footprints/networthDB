use super::{Parser, Statement};
use crate::reader::File;

pub fn get_parser() -> Parser {
    fn identify(file: &File) -> bool {
        if file.file_type != "xls" {
            return false;
        }

        if let Some(first_row) = file.data.first() {
            if let Some(first_cell) = first_row.first() {
                if first_cell.contains("ICICI") {
                    return true;
                }
            }
        }

        false
    }

    fn parse(file: &File) -> Vec<Statement> {
        let _ = file;
        return Vec::new();
    }

    Parser {
        id: "icicind".to_string(),
        identify,
        parse,
    }
}
