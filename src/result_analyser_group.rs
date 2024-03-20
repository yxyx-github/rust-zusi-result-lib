#[cfg(test)]
mod tests;

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
        // TODO: check if analysers.len > 0
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

        if self.total_distance()? == 0.0 {
            // TODO: return error becaus of 0.0 / 0.0 = NaN
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