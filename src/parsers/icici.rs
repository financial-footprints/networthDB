use super::Parser;
use crate::reader::FileData;

pub fn get_parser() -> Parser {
    fn identify(file_data: &FileData) -> bool {
        if file_data.file_type != "xls" {
            return false;
        }

        if let Some(first_row) = file_data.data.first() {
            if let Some(first_cell) = first_row.first() {
                if first_cell.contains("ICICI") {
                    return true;
                }
            }
        }

        false
    }

    Parser {
        id: "icicind".to_string(),
        identify,
    }
}
