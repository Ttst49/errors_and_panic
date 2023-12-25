fn main() {
    result_training()
}


pub fn panic_training(){
    //calling macro panic! directly
    //panic!("ALED")

    //make the program panic by a code error
    let v = vec![12,14,13];

    v[23];
}

use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn result_training(){
    #[allow(unused_variables)]
    let f = File::open("hello.txt");

    //using match
    #[allow(unused_variables)]
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Erreur de création du fichier : {:?}", e),
            },
            other=>{
                panic!("Erreur d'ouverture du fichier : {:?}", other)
            }
        },
    };

    //better way to do it
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Erreur de création du fichier : {:?}", error);
            })
        } else {
            panic!("Erreur d'ouverture du fichier : {:?}", error);
        }
    });


    //unwrap show automatically the ok variable and automatically use panic! macro
    let f = File::open("hello.txt").unwrap();

    //expect can define a personalized message for errors
    let f = File::open("Hello.rs").expect("Oups problème");

}

//example function
fn read_nickname_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//alternative way
fn read_nickname_from_file_with_interrogation() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

//last way in order to shorter the code
fn read_nickname() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

use std::fs;
//way to read a file and put it value in a string in a single method
fn all_in_one()->Result<String,io::Error>{
    fs::read_to_string("hello.txt")
}