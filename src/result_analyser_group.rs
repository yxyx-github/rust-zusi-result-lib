use crate::result_analyser::{AnalyseError, ResultAnalyser};

struct AnalysisCache {
    total_distance: Option<f32>,
    average_distance: Option<f32>,
    average_speed: Option<f32>,
}

impl AnalysisCache {
    pub fn new() -> AnalysisCache {
        Self {
            total_distance: None,
            average_distance: None,
            average_speed: None,
        }
    }
}

pub struct ResultAnalyserGroup {
    analysers: Vec<ResultAnalyser>,
    cache: AnalysisCache,
}

impl ResultAnalyserGroup {
    pub fn new(analysers: Vec<ResultAnalyser>) -> ResultAnalyserGroup {
        Self {
            analysers,
            cache: AnalysisCache::new(),
        }
    }

    pub fn total_distance(&mut self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.total_distance {
            return Ok(*value);
        }

        let mut total_distance = 0.0;

        for analyser in self.analysers.iter() {
            total_distance += analyser.distance()?;
        }

        self.cache.total_distance = Some(total_distance);
        Ok(total_distance)
    }

    pub fn average_distance(&mut self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.average_distance {
            return Ok(*value);
        }

        let average_distance = self.total_distance()? / self.analysers.len() as f32;

        self.cache.average_distance = Some(average_distance);
        Ok(average_distance)
    }

    pub fn average_speed(&mut self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.average_speed {
            return Ok(*value);
        }

        let mut weighted_speed_sum = 0.0;
        for analyser in self.analysers.iter() {
            weighted_speed_sum += analyser.distance()? * analyser.average_speed()?;
        }

        let average_speed = weighted_speed_sum / self.total_distance()?;

        self.cache.average_speed = Some(average_speed);
        Ok(average_speed)
    }
}

#[cfg(test)]
mod tests {
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
}