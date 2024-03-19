use std::fs::File;
use std::io::Read;

use zusi_xml_lib::xml::zusi::{Zusi, ZusiValue};
use zusi_xml_lib::xml::zusi::result::ZusiResult;

use zusi_result_lib::result_analyser::ResultAnalyser;
use zusi_result_lib::result_analyser_group::ResultAnalyserGroup;

fn main() {
    println!("Hello, world!");

    let mut results = vec![];

    for i in 1..4 {
        let mut input_file = File::open(format!("./data/Ergebnis{i}.result.xml")).unwrap();
        let mut contents = String::new();
        input_file.read_to_string(&mut contents).unwrap();

        let zusi = Zusi::from_xml(&contents).unwrap();

        for value in zusi.value {
            if let ZusiValue::Result(result) = value {
                results.push(result);
            }
        }
    }

    analyse(results);
}

fn analyse(results: Vec<ZusiResult>) {
    let mut analyser_group = ResultAnalyserGroup::new(results.into_iter().map(|r| ResultAnalyser::new(r)).collect());
    println!("distance: {}", analyser_group.distance().unwrap())
}
