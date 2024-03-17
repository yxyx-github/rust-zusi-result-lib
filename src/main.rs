use std::fs::File;
use std::io::Read;

use zusi_xml_lib::xml::zusi::{Zusi, ZusiValue};
use zusi_xml_lib::xml::zusi::result::ZusiResult;

use zusi_result_lib::result_analyser::ResultAnalyser;

fn main() {
    println!("Hello, world!");

    let mut input_file = File::open("./data/Ergebnis.result.xml").unwrap();
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).unwrap();

    let zusi = Zusi::from_xml(&contents).unwrap();

    for value in zusi.value {
        if let ZusiValue::Result(result) = value {
            analyse(result);
        }
    }
}

fn analyse(result: ZusiResult) {
    let analyser = ResultAnalyser::new(result);
    println!("distance: {}", analyser.distance().unwrap())
}
