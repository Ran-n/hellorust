#!
//#+ Autor:	Ran#
//#+ Creado:	01/05/2021 13:22:12
//#+ Editado:	01/05/2021 13:28:32

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let paths = fs::read_dir(args[1].clone()).unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
}
