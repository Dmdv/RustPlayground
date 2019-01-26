use std::fs::File;

fn main() {

    let f = File::open("readme.txt");

    let file = match f {
        Ok(file) => file,
        Err(error) => {
            println!("File not found, but I will create it");
            let res = File::create("readme.txt");
            match res {
                Ok(res) => res,
                Err(error) => {
                    panic!("Couldn't create file")
                },
            }
        },
    };

    // unwrapping is the more right way to implement this

    let ufile = File::open("readme.txt").unwrap();

    // setting extectations

    let efile = File::open("readme2.txt").expect("We need this file");
}
