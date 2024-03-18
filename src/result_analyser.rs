use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};

pub struct ResultAnalyser {
    result: ZusiResult,
}

impl ResultAnalyser {
    pub fn new(result: ZusiResult) -> ResultAnalyser {
        Self {
            result,
        }
    }

    // TODO: change Err type to enum
    pub fn distance(&self) -> Result<f32, String> {
        if self.result.value.len() > 0 {
            // TODO: filter for FahrtEintrag
            let first = self.result.value.first().unwrap();
            let last = self.result.value.last().unwrap();
            let first = match first {
                ResultValue::FahrtEintrag(first) => first.fahrt_weg,
                _ => {
                    return Err("Entry not valid.".into())
                }
            };
            let last = match last {
                ResultValue::FahrtEintrag(last) => last.fahrt_weg,
                _ => {
                    return Err("Entry not valid.".into())
                }
            };
            Ok(last - first)
        } else {
            Err("No entries found.".into())
        }
    }
}

#[cfg(test)]
pub mod test {
    use time::macros::datetime;
    use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};
    use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::{FahrtEintrag, FahrtTyp};
    use crate::result_analyser::ResultAnalyser;

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
        assert_eq!(analyser.distance(), Err("No entries found.".into()));
    }
}