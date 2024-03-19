use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};

#[derive(Clone, PartialEq, Debug)]
pub enum AnalyseError {
    NoEntriesFound,
}

pub struct ResultAnalyser {
    result: ZusiResult,
}

impl ResultAnalyser {
    pub fn new(result: ZusiResult) -> ResultAnalyser {
        Self {
            result,
        }
    }

    pub fn distance(&self) -> Result<f32, AnalyseError> {
        if self.result.value.len() > 0 {
            let ResultValue::FahrtEintrag(first) = self.result.value.first().unwrap();
            let ResultValue::FahrtEintrag(last) = self.result.value.last().unwrap();
            Ok(last.fahrt_weg - first.fahrt_weg)
        } else {
            Err(AnalyseError::NoEntriesFound)
        }
    }
}

#[cfg(test)]
pub mod test {
    use time::macros::datetime;
    use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};
    use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::{FahrtEintrag, FahrtTyp};
    use crate::result_analyser::{AnalyseError, ResultAnalyser};

    fn result(empty: bool) -> ZusiResult {
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
    fn test_distance() {
        let analyser = ResultAnalyser::new(result(false));
        assert_eq!(analyser.distance().unwrap(), 20.1);
    }

    #[test]
    fn test_distance_with_empty_result() {
        let analyser = ResultAnalyser::new(result(true));
        assert_eq!(analyser.distance(), Err(AnalyseError::NoEntriesFound));
    }
}