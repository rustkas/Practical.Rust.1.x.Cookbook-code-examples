use result_type_01::{create_file_path, read_file, read_file2};

fn main() {
    // Call the function and handle the result.
    let file_path = create_file_path(vec!["members","result-type-01"], "file.txt").unwrap();
    match read_file(file_path.as_str()) {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error: {}", e),
    }

    match read_file2(file_path.as_str()) {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error: {:?}", e),
    }
}
