use time::Duration;
use time::macros::datetime;
use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::{FahrtEintrag, FahrtTyp};

use crate::result_analyser::{AnalyseError, ResultAnalyser};

fn distance_result(empty: bool) -> ZusiResult {
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
                    fahrt_weg: 2.33,
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
                    fahrt_weg: 22.43,
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
fn test_distance_2() {
    let analyser = ResultAnalyser::new(distance_result(false));
    assert_eq!(analyser.distance().unwrap(), 20.1);
}

#[test]
fn test_distance_0() {
    let analyser = ResultAnalyser::new(distance_result(true));
    assert_eq!(analyser.distance(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_average_speed_3() {
    let result = ZusiResult {
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
                fahrt_weg: 5.0,
                fahrt_zeit: datetime!(2019-01-01 23:18),
                fahrt_speed: 10.0,
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
                fahrt_weg: 15.0,
                fahrt_zeit: datetime!(2019-01-01 23:18),
                fahrt_speed: 30.0,
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
                fahrt_weg: 35.0,
                fahrt_zeit: datetime!(2019-01-02 1:07),
                fahrt_speed: 100.0,
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

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.average_speed().unwrap(), 50.0);
}

#[test]
fn test_average_speed_2() {
    let result = ZusiResult {
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
                fahrt_weg: 5.0,
                fahrt_zeit: datetime!(2019-01-01 23:18),
                fahrt_speed: 10.0,
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
                fahrt_weg: 15.0,
                fahrt_zeit: datetime!(2019-01-01 23:18),
                fahrt_speed: 30.0,
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

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.average_speed().unwrap(), 20.0);
}

#[test]
fn test_average_speed_1() {
    let result = ZusiResult {
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
                fahrt_weg: 5.0,
                fahrt_zeit: datetime!(2019-01-01 23:18),
                fahrt_speed: 10.0,
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

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.average_speed(), Err(AnalyseError::ZeroDistance));
}

#[test]
fn test_average_speed_0() {
    let result = ZusiResult {
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

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.average_speed(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_driving_time_2() {
    let result = ZusiResult {
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
                fahrt_weg: 5.0,
                fahrt_zeit: datetime!(2019-01-01 23:18),
                fahrt_speed: 10.0,
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
                fahrt_weg: 15.0,
                fahrt_zeit: datetime!(2019-01-02 0:38),
                fahrt_speed: 30.0,
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

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.driving_time().unwrap(), Duration::minutes(80));
}

#[test]
fn test_driving_time_0() {
    let result = ZusiResult {
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

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.distance(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_pure_driving_time() {
    let result = ZusiResult {
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
                fahrt_weg: 5.0,
                fahrt_zeit: datetime!(2019-01-01 20:00),
                fahrt_speed: 10.0,
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
                fahrt_weg: 15.0,
                fahrt_zeit: datetime!(2019-01-01 23:00),
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
                fahrt_weg: 15.0,
                fahrt_zeit: datetime!(2019-01-02 0:00),
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
                fahrt_weg: 15.0,
                fahrt_zeit: datetime!(2019-01-02 0:30),
                fahrt_speed: 30.0,
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
                fahrt_weg: 15.0,
                fahrt_zeit: datetime!(2019-01-02 0:45),
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
                fahrt_weg: 15.0,
                fahrt_zeit: datetime!(2019-01-02 1:05),
                fahrt_speed: 30.0,
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

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_driving_time().unwrap(), Duration::minutes(245));
}

#[test]
fn test_pure_driving_time_1() {
    let result = ZusiResult {
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
                fahrt_weg: 15.0,
                fahrt_zeit: datetime!(2019-01-02 1:05),
                fahrt_speed: 30.0,
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

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_driving_time().unwrap(), Duration::seconds(0));
}

#[test]
fn test_pure_driving_time_0() {
    let result = ZusiResult {
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

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_driving_time(), Err(AnalyseError::NoEntries));
}