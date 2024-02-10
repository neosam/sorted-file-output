use std::io::Result as IoResult;
use std::io::Write;
use std::{
    fs::{self, File},
    path::PathBuf,
};

fn get_all_filenames(path: &str) -> Vec<String> {
    return fs::read_dir(path)
        .unwrap()
        .map(|x| x.unwrap().path().display().to_string())
        .collect();
}

fn sort_files_by_leading_number(files: &mut Vec<String>) {
    files.sort_by(|a, b| {
        let a_number = extract_leading_number(a);
        let b_number = extract_leading_number(b);
        a_number.cmp(&b_number)
    })
}

fn extract_leading_number(path: &str) -> u64 {
    path.splitn(2, |c: char| !c.is_digit(10))
        .next()
        .unwrap_or("0")
        .parse()
        .unwrap_or(0)
}

fn write_vec_to_file(lines: Vec<String>, filename: &str) -> IoResult<()> {
    let mut file = File::create(filename)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn main() {
    let mut all_files = get_all_filenames("./");
    sort_files_by_leading_number(&mut all_files);
    write_vec_to_file(all_files, "./sorted.txt").expect("Error while writing to file");
}

#[test]
fn extract_leading_number_test() {
    assert_eq!(extract_leading_number("a"), 0);
    assert_eq!(extract_leading_number("3a"), 3);
    assert_eq!(extract_leading_number("10 a"), 10);
    assert_eq!(extract_leading_number("123.a"), 123);
}

#[test]
fn sort_files_by_leading_number_test() {
    let mut data: Vec<String> = vec!["1.rs".into(), "3.rs".into(), "2.rs".into()];
    sort_files_by_leading_number(&mut data);
    assert_eq!(
        data,
        vec!["1.rs".to_string(), "2.rs".to_string(), "3.rs".to_string()]
    );
}
