use calamine::{open_workbook, Reader, Xls};

pub struct File {
    pub file_type: String,
    pub data: Vec<Vec<String>>,
}

fn read_xls(file_path: &str) -> Vec<Vec<String>> {
    let mut workbook: Xls<_> =
        open_workbook(file_path).expect("error.reader.read_xls.cannot_open_file");
    let mut data: Vec<Vec<String>> = Vec::new();
    for sheet in workbook.sheet_names().to_owned() {
        if let Ok(range) = workbook.worksheet_range(&sheet) {
            for row in range.rows() {
                let row_data: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
                data.push(row_data);
            }
        }
    }

    return data;
}

pub fn read_file(file_path: &str) -> File {
    let file_extension = std::path::Path::new(file_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str);

    if file_extension == Some("xls") {
        return File {
            file_type: String::from("xls"),
            data: read_xls(file_path),
        };
    }

    panic!("error.reader.read_file.unsupported_file_type");
}
