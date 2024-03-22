use time::Duration;
use time::macros::datetime;
use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::FahrtEintrag;

use crate::result_analyser::{AnalyseError, ResultAnalyser};

#[test]
fn test_distance_2() {
    let result = ZusiResult::builder()
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

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.distance().unwrap(), 20.1);
}

#[test]
fn test_distance_0() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.distance(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_pure_average_speed_3() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(5.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(10.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(15.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(30.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(35.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(100.)
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_average_speed().unwrap(), 50.);
}

#[test]
fn test_pure_average_speed_2() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(5.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(10.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(15.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(30.)
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_average_speed().unwrap(), 20.);
}

#[test]
fn test_pure_average_speed_1() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(5.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(10.)
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_average_speed(), Err(AnalyseError::ZeroDistance));
}

#[test]
fn test_pure_average_speed_0() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_average_speed(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_driving_time_2() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(10.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:38))
                .fahrt_speed(30.)
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.driving_time().unwrap(), Duration::minutes(80));
}

#[test]
fn test_driving_time_0() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.distance(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_pure_driving_time() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 20:00))
                .fahrt_speed(30.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:00))
                .fahrt_speed(0.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:00))
                .fahrt_speed(0.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:30))
                .fahrt_speed(30.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:45))
                .fahrt_speed(0.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 1:05))
                .fahrt_speed(30.)
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_driving_time().unwrap(), Duration::minutes(245));
}

#[test]
fn test_pure_driving_time_1() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 1:05))
                .fahrt_speed(30.)
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_driving_time().unwrap(), Duration::seconds(0));
}

#[test]
fn test_pure_driving_time_0() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_driving_time(), Err(AnalyseError::NoEntries));
}