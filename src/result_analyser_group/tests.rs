use time::macros::datetime;
use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::{FahrtEintrag, FahrtTyp};

use crate::result_analyser::{AnalyseError, ResultAnalyser};
use crate::result_analyser_group::{CreateAnalyserGroupError, ResultAnalyserGroup};

#[test]
fn test_caching() {
    let result1 = ZusiResult {
        zugnummer: "12345".into(),
        tf_nummer: "67890".into(),
        datum: datetime!(2019-01-01 23:14),
        verbrauch: 0.0,
        bemerkung: "".to_string(),
        schummel: false,
        schwierigkeitsgrad: 0,
        energie_vorgabe: 0.0,
        value: vec![
            ResultValue::FahrtEintrag(FahrtEintrag {
                fahrt_typ: FahrtTyp::Standard,
                fahrt_weg: 0.0,
                fahrt_zeit: datetime!(2019-01-01 23:18),
                fahrt_speed: 8.0,
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
                fahrt_weg: 3.0,
                fahrt_zeit: datetime!(2019-01-01 23:18),
                fahrt_speed: 8.0,
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
        ],
    };
    let result2 = ZusiResult {
        zugnummer: "12345".into(),
        tf_nummer: "67890".into(),
        datum: datetime!(2019-01-01 23:14),
        verbrauch: 0.0,
        bemerkung: "".to_string(),
        schummel: false,
        schwierigkeitsgrad: 0,
        energie_vorgabe: 0.0,
        value: vec![
            ResultValue::FahrtEintrag(FahrtEintrag {
                fahrt_typ: FahrtTyp::Standard,
                fahrt_weg: 0.0,
                fahrt_zeit: datetime!(2019-01-01 23:18),
                fahrt_speed: 4.0,
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
                fahrt_weg: 9.0,
                fahrt_zeit: datetime!(2019-01-01 23:18),
                fahrt_speed: 4.0,
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
        ],
    };

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();
    for _ in 0..2 {
        assert_eq!(analyser_group.total_distance().unwrap(), 12.0);
        assert_eq!(analyser_group.average_distance().unwrap(), 6.0);
        assert_eq!(analyser_group.average_speed().unwrap(), 5.0);
    }
}

#[test]
fn test_create_analyser_group_error() {
    let analyser_group = ResultAnalyserGroup::new(vec![]);
    assert_eq!(analyser_group, Err(CreateAnalyserGroupError::NoAnalysers));
}

fn distance_result(empty: bool, fahrt_weg_1: f32, fahrt_weg_2: f32) -> ZusiResult {
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
                    fahrt_speed: 4.0,
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
                    fahrt_speed: 6.0,
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
fn test_total_distance() {
    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(distance_result(false, 2.33, 22.43)),
        ResultAnalyser::new(distance_result(false, 7.33, 72.43)),
    ]).unwrap();
    assert_eq!(analyser_group.total_distance().unwrap(), 85.2);
}

#[test]
fn test_total_distance_with_error() {
    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(distance_result(true, 2.33, 22.43)),
        ResultAnalyser::new(distance_result(false, 7.33, 72.43)),
    ]).unwrap();
    assert_eq!(
        analyser_group.total_distance(),
        Err(AnalyseError::NoEntries)
    );
}

#[test]
fn test_average_distance() {
    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(distance_result(false, 2.33, 22.43)),
        ResultAnalyser::new(distance_result(false, 7.33, 72.43)),
    ]).unwrap();
    assert_eq!(analyser_group.average_distance().unwrap(), 42.6);
}

#[test]
fn test_average_distance_with_error() {
    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(distance_result(true, 2.33, 22.43)),
        ResultAnalyser::new(distance_result(false, 7.33, 72.43)),
    ]).unwrap();
    assert_eq!(
        analyser_group.average_distance(),
        Err(AnalyseError::NoEntries)
    );
}

#[test]
fn test_average_speed_2() {
    let result1 = ZusiResult {
        zugnummer: "12345".into(),
        tf_nummer: "67890".into(),
        datum: datetime!(2019-01-01 23:14),
        verbrauch: 0.0,
        bemerkung: "".to_string(),
        schummel: false,
        schwierigkeitsgrad: 0,
        energie_vorgabe: 0.0,
        value: vec![
                ResultValue::FahrtEintrag(FahrtEintrag {
                    fahrt_typ: FahrtTyp::Standard,
                    fahrt_weg: 0.0,
                    fahrt_zeit: datetime!(2019-01-01 23:18),
                    fahrt_speed: 8.0,
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
                    fahrt_weg: 3.0,
                    fahrt_zeit: datetime!(2019-01-01 23:18),
                    fahrt_speed: 8.0,
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
            ],
    };
    let result2 = ZusiResult {
        zugnummer: "12345".into(),
        tf_nummer: "67890".into(),
        datum: datetime!(2019-01-01 23:14),
        verbrauch: 0.0,
        bemerkung: "".to_string(),
        schummel: false,
        schwierigkeitsgrad: 0,
        energie_vorgabe: 0.0,
        value: vec![
                ResultValue::FahrtEintrag(FahrtEintrag {
                    fahrt_typ: FahrtTyp::Standard,
                    fahrt_weg: 0.0,
                    fahrt_zeit: datetime!(2019-01-01 23:18),
                    fahrt_speed: 4.0,
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
                    fahrt_weg: 9.0,
                    fahrt_zeit: datetime!(2019-01-01 23:18),
                    fahrt_speed: 4.0,
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
            ],
    };

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();
    assert_eq!(analyser_group.average_speed().unwrap(), 5.0);
}

#[test]
fn test_average_speed_1() {
    let result1 = ZusiResult {
        zugnummer: "12345".into(),
        tf_nummer: "67890".into(),
        datum: datetime!(2019-01-01 23:14),
        verbrauch: 0.0,
        bemerkung: "".to_string(),
        schummel: false,
        schwierigkeitsgrad: 0,
        energie_vorgabe: 0.0,
        value: vec![
                ResultValue::FahrtEintrag(FahrtEintrag {
                    fahrt_typ: FahrtTyp::Standard,
                    fahrt_weg: 3.0,
                    fahrt_zeit: datetime!(2019-01-01 23:18),
                    fahrt_speed: 8.0,
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
            ],
    };
    let result2 = ZusiResult {
        zugnummer: "12345".into(),
        tf_nummer: "67890".into(),
        datum: datetime!(2019-01-01 23:14),
        verbrauch: 0.0,
        bemerkung: "".to_string(),
        schummel: false,
        schwierigkeitsgrad: 0,
        energie_vorgabe: 0.0,
        value: vec![
                ResultValue::FahrtEintrag(FahrtEintrag {
                    fahrt_typ: FahrtTyp::Standard,
                    fahrt_weg: 9.0,
                    fahrt_zeit: datetime!(2019-01-01 23:18),
                    fahrt_speed: 4.0,
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
            ],
    };

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.average_speed(), Err(AnalyseError::ZeroDistance));
}

#[test]
fn test_average_speed_0() {
    let result1 = ZusiResult {
        zugnummer: "12345".into(),
        tf_nummer: "67890".into(),
        datum: datetime!(2019-01-01 23:14),
        verbrauch: 0.0,
        bemerkung: "".to_string(),
        schummel: false,
        schwierigkeitsgrad: 0,
        energie_vorgabe: 0.0,
        value: vec![],
    };
    let result2 = ZusiResult {
        zugnummer: "12345".into(),
        tf_nummer: "67890".into(),
        datum: datetime!(2019-01-01 23:14),
        verbrauch: 0.0,
        bemerkung: "".to_string(),
        schummel: false,
        schwierigkeitsgrad: 0,
        energie_vorgabe: 0.0,
        value: vec![],
    };

    let mut analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    println!("avgspd: {:?}", analyser_group.average_speed());
    assert_eq!(analyser_group.average_speed(), Err(AnalyseError::NoEntries));
}