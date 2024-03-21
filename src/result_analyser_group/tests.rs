use time::Duration;
use time::macros::datetime;
use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::FahrtEintrag;

use crate::result_analyser::{AnalyseError, ResultAnalyser};
use crate::result_analyser_group::{CreateAnalyserGroupError, ResultAnalyserGroup};

#[test]
fn test_caching() {
    let result1 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(0.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(8.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(3.)
                .fahrt_zeit(datetime!(2019-01-01 23:28))
                .fahrt_speed(8.)
                .build()),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(0.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(4.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(9.)
                .fahrt_zeit(datetime!(2019-01-01 23:33))
                .fahrt_speed(4.)
                .build()),
        ])
        .build();

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    for _ in 0..2 {
        assert_eq!(analyser_group.total_distance().unwrap(), 12.);
        assert_eq!(analyser_group.average_distance().unwrap(), 6.);
        assert_eq!(analyser_group.average_speed().unwrap(), 5.);
        assert_eq!(analyser_group.total_driving_time().unwrap(), Duration::minutes(25));
    }
}

#[test]
fn test_create_analyser_group_error() {
    let analyser_group = ResultAnalyserGroup::new(vec![]);
    assert_eq!(analyser_group, Err(CreateAnalyserGroupError::NoAnalysers));
}

#[test]
fn test_total_distance() {
    let result1 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(22.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(7.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(72.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
        ])
        .build();

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.total_distance().unwrap(), 85.2);
}

#[test]
fn test_total_distance_with_error() {
    let result1 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(7.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(72.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(
        analyser_group.total_distance(),
        Err(AnalyseError::NoEntries)
    );
}

#[test]
fn test_average_distance() {
    let result1 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(22.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(7.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(72.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
        ])
        .build();

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.average_distance().unwrap(), 42.6);
}

#[test]
fn test_average_distance_with_error() {
    let result1 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(22.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(
        analyser_group.average_distance(),
        Err(AnalyseError::NoEntries)
    );
}

#[test]
fn test_average_speed_2() {
    let result1 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(0.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(8.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(3.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(8.)
                .build()),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(0.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(4.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(9.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(4.)
                .build()),
        ])
        .build();

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.average_speed().unwrap(), 5.);
}

#[test]
fn test_average_speed_1() {
    let result1 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(3.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(8.)
                .build()),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(9.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(4.)
                .build()),
        ])
        .build();

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.average_speed(), Err(AnalyseError::ZeroDistance));
}

#[test]
fn test_average_speed_0() {
    let result1 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();
    let result2 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.average_speed(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_total_driving_time() {
    let result1 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:28))
                .build()),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:33))
                .build()),
        ])
        .build();

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.total_driving_time().unwrap(), Duration::minutes(25));
}

#[test]
fn test_total_driving_time_with_error() {
    let result1 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:28))
                .build()),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(
        analyser_group.total_distance(),
        Err(AnalyseError::NoEntries)
    );
}