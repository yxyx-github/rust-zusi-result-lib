use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};

#[derive(PartialEq, Debug)]
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

    pub fn average_speed(&self) -> Result<f32, AnalyseError> {
        if self.result.value.len() > 1 {
            let mut weighted_speed_sum = 0.0;
            for i in 0..self.result.value.len() - 1 {
                let ResultValue::FahrtEintrag(current) = self.result.value.get(i).unwrap();
                let ResultValue::FahrtEintrag(next) = self.result.value.get(i + 1).unwrap();
                let local_average_speed = (current.fahrt_speed + next.fahrt_speed) / 2.0;
                let local_distance = next.fahrt_weg - current.fahrt_weg;
                weighted_speed_sum += local_distance * local_average_speed;
            }
            Ok(weighted_speed_sum / self.distance()?)
        } else if self.result.value.len() > 0 {
            let ResultValue::FahrtEintrag(first) = self.result.value.first().unwrap();
            Ok(first.fahrt_speed)
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
    fn test_distance() {
        let analyser = ResultAnalyser::new(distance_result(false));
        assert_eq!(analyser.distance().unwrap(), 20.1);
    }

    #[test]
    fn test_distance_with_empty_result() {
        let analyser = ResultAnalyser::new(distance_result(true));
        assert_eq!(analyser.distance(), Err(AnalyseError::NoEntriesFound));
    }

    #[test]
    fn test_average_speed() {
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
}