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