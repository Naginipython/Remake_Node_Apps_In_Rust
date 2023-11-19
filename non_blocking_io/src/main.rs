use std::fs;
use std::process;


fn main() {
    let files: Vec<&str> = vec!["file1.txt", "file2.txt", "file3.txt", "file4.txt"];
    
    for f in files {
        let result = fs::read_to_string(f).unwrap_or_else(|_err| {
            eprintln!("Error: {f} does not exist. ");
            process::exit(1);
        });
        println!("{result}");
    }
}
