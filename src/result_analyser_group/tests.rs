use time::macros::datetime;
use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::{FahrtEintrag, FahrtTyp};

use crate::result_analyser::{AnalyseError, ResultAnalyser};
use crate::result_analyser_group::ResultAnalyserGroup;

fn total_distance_result(empty: bool, fahrt_weg_1: f32, fahrt_weg_2: f32) -> ZusiResult {
    ZusiResult {
        zugnummer: "12345".into(),
        tf_nummer: "67890".into(),
        datum: datetime!(2019-01-01 23:14),
        verbrauch: 0.0,
        bemerkung: "".to_string(),
        schummel: false,
        schwierigkeitsgrad: 0,
        energie_vorgabe: 0.0,
        value: if empty {
            vec![]
        } else {
            vec![
                ResultValue::FahrtEintrag(FahrtEintrag {
                    fahrt_typ: FahrtTyp::Standard,
                    fahrt_weg: fahrt_weg_1,
                    fahrt_zeit: datetime!(2019-01-01 23:18),
                    fahrt_speed: 0.0,
                    fahrt_speed_strecke: 0.0,
                    fahrt_speed_signal: 0.0,
                    fahrt_speed_zugsicherung: 0.0,
                    fahrt_autopilot: false,
                    fahrt_km: 0.0,
                    fahrt_hl_druck: 0.0,
                    fahrt_parameter: 0,
                    fahrt_fpl_ank: None,
                    fahrt_fpl_abf: None,
                    fahrt_fb_schalter: 0,
                }),
                ResultValue::FahrtEintrag(FahrtEintrag {
                    fahrt_typ: FahrtTyp::Standard,
                    fahrt_weg: fahrt_weg_2,
                    fahrt_zeit: datetime!(2019-01-02 1:07),
                    fahrt_speed: 0.0,
                    fahrt_speed_strecke: 0.0,
                    fahrt_speed_signal: 0.0,
                    fahrt_speed_zugsicherung: 0.0,
                    fahrt_autopilot: false,
                    fahrt_km: 0.0,
                    fahrt_hl_druck: 0.0,
                    fahrt_parameter: 0,
                    fahrt_fpl_ank: None,
                    fahrt_fpl_abf: None,
                    fahrt_fb_schalter: 0,
                }),
            ]
        },
    }
}

#[test]
fn test_caching() {
    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(total_distance_result(false, 2.33, 22.43)),
        ResultAnalyser::new(total_distance_result(false, 7.33, 72.43)),
    ]);
    for _ in 0..2 {
        assert_eq!(analyser_group.total_distance().unwrap(), 85.2);
        assert_eq!(analyser_group.average_distance().unwrap(), 42.6);
    }
}

#[test]
fn test_total_distance() {
    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(total_distance_result(false, 2.33, 22.43)),
        ResultAnalyser::new(total_distance_result(false, 7.33, 72.43)),
    ]);
    assert_eq!(analyser_group.total_distance().unwrap(), 85.2);
}

#[test]
fn test_total_distance_with_error() {
    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(total_distance_result(true, 2.33, 22.43)),
        ResultAnalyser::new(total_distance_result(false, 7.33, 72.43)),
    ]);
    assert_eq!(
        analyser_group.total_distance(),
        Err(AnalyseError::NoEntriesFound)
    );
}

#[test]
fn test_average_distance() {
    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(total_distance_result(false, 2.33, 22.43)),
        ResultAnalyser::new(total_distance_result(false, 7.33, 72.43)),
    ]);
    assert_eq!(analyser_group.average_distance().unwrap(), 42.6);
}

#[test]
fn test_average_distance_with_error() {
    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(total_distance_result(true, 2.33, 22.43)),
        ResultAnalyser::new(total_distance_result(false, 7.33, 72.43)),
    ]);
    assert_eq!(
        analyser_group.average_distance(),
        Err(AnalyseError::NoEntriesFound)
    );
}