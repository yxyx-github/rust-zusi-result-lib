use std::fs::File;
use std::io::Read;

use zusi_xml_lib::xml::zusi::{Zusi, ZusiValue};
use zusi_xml_lib::xml::zusi::result::ZusiResult;

use zusi_result_lib::result_analyser::ResultAnalyser;
use zusi_result_lib::result_analyser_group::ResultAnalyserGroup;

fn main() {
    println!("Hello, world!");

    let mut results = vec![];

    for i in 0..4 {
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
    let mut analyser_group = ResultAnalyserGroup::new(
        results.into_iter().map(|r| ResultAnalyser::new(r)).collect()
    ).unwrap();
    println!("total distance: {} m", analyser_group.total_distance().unwrap());
    println!("average distance: {} m", analyser_group.average_distance().unwrap());
    let average_speed = analyser_group.average_speed().unwrap();
    println!("average speed: {} m/s = {} km/h", average_speed, average_speed * 3.6);
    let pure_average_speed = analyser_group.pure_average_speed().unwrap();
    println!("pure average speed: {} m/s = {} km/h", pure_average_speed, pure_average_speed * 3.6);
    println!("total driving time: {}", analyser_group.total_driving_time().unwrap());
    println!("total pure driving time: {}", analyser_group.total_pure_driving_time().unwrap());
}
