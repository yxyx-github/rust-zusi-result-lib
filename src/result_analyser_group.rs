use crate::result_analyser::{AnalyseError, ResultAnalyser};

pub struct ResultAnalyserGroup {
    analysers: Vec<ResultAnalyser>,
}

impl ResultAnalyserGroup {
    pub fn new(analysers: Vec<ResultAnalyser>) -> ResultAnalyserGroup {
        Self {
            analysers,
        }
    }

    pub fn distance(&self) -> Result<f32, (f32, Vec<AnalyseError>)> {
        let mut total_distance = 0.0;
        let mut errors = vec![];

        self.analysers.iter().for_each(|analyser| {
            match analyser.distance() {
                Ok(distance) => total_distance += distance,
                Err(error) => errors.push(error),
            };
        });

        if errors.len() > 0 {
            Err((total_distance, errors))
        } else {
            Ok(total_distance)
        }
    }
}

#[cfg(test)]
pub mod test {
    use time::macros::datetime;
    use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};
    use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::{FahrtEintrag, FahrtTyp};

    use crate::result_analyser::{AnalyseError, ResultAnalyser};
    use crate::result_analyser_group::ResultAnalyserGroup;

    fn result(empty: bool, fahrt_weg_1: f32, fahrt_weg_2: f32) -> ZusiResult {
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
    fn test_distance() {
        let analyser_group = ResultAnalyserGroup::new(vec![
            ResultAnalyser::new(result(false, 2.33, 22.43)),
            ResultAnalyser::new(result(false, 7.33, 72.43)),
        ]);
        assert_eq!(analyser_group.distance().unwrap(), 85.2);
    }

    #[test]
    fn test_distance_with_single_error() {
        let analyser_group = ResultAnalyserGroup::new(vec![
            ResultAnalyser::new(result(true, 2.33, 22.43)),
            ResultAnalyser::new(result(false, 7.33, 72.43)),
        ]);
        assert_eq!(
            analyser_group.distance(),
            Err((65.1, vec![AnalyseError::NoEntriesFound]))
        );
    }

    #[test]
    fn test_distance_with_all_error() {
        let analyser_group = ResultAnalyserGroup::new(vec![
            ResultAnalyser::new(result(true, 2.33, 22.43)),
            ResultAnalyser::new(result(true, 7.33, 72.43)),
        ]);
        assert_eq!(
            analyser_group.distance(),
            Err((0.0, vec![AnalyseError::NoEntriesFound, AnalyseError::NoEntriesFound]))
        );
    }
}