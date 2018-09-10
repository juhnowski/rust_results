use std::fs::File;

fn main() {

    let _f = File::open("hello1.txt").unwrap();//expect("File not found");
/*
    let foo = match f {
        Ok(file) => file,
        Err(error) => panic!("file not found"),
    };
    */
}
