#[cfg(test)]
mod tests;
mod analyser_group_cache;

use time::Duration;
use crate::result_analyser::{AnalyseError, ResultAnalyser};
use crate::result_analyser_group::analyser_group_cache::AnalyserGroupCache;

#[derive(PartialEq, Debug)]
pub enum CreateAnalyserGroupError {
    NoAnalysers,
}

#[derive(PartialEq, Debug)]
pub struct ResultAnalyserGroup {
    analysers: Vec<ResultAnalyser>,
    cache: AnalyserGroupCache,
}

impl ResultAnalyserGroup {
    pub fn new(analysers: Vec<ResultAnalyser>) -> Result<ResultAnalyserGroup, CreateAnalyserGroupError> {
        if analysers.len() == 0 {
            Err(CreateAnalyserGroupError::NoAnalysers)
        } else {
            Ok(Self {
                analysers,
                cache: AnalyserGroupCache::new(),
            })
        }
    }

    pub fn total_distance(&mut self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.total_distance {
            return Ok(*value);
        }

        let mut total_distance = 0.;

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

        let mut weighted_speed_sum = 0.;
        for analyser in self.analysers.iter() {
            weighted_speed_sum += analyser.distance()? * analyser.average_speed()?;
        }

        let average_speed = weighted_speed_sum / self.total_distance()?;

        self.cache.average_speed = Some(average_speed);
        Ok(average_speed)
    }

    pub fn pure_average_speed(&mut self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.pure_average_speed {
            return Ok(*value);
        }

        let mut weighted_speed_sum = 0.;
        for analyser in self.analysers.iter() {
            weighted_speed_sum += analyser.distance()? * analyser.pure_average_speed()?;
        }

        let pure_average_speed = weighted_speed_sum / self.total_distance()?;

        self.cache.pure_average_speed = Some(pure_average_speed);
        Ok(pure_average_speed)
    }

    pub fn total_driving_time(&mut self) -> Result<Duration, AnalyseError> {
        if let Some(value) = &self.cache.total_driving_time {
            return Ok(*value);
        }

        let mut total_driving_time = Duration::seconds(0);

        for analyser in self.analysers.iter() {
            total_driving_time += analyser.driving_time()?;
        }

        self.cache.total_driving_time = Some(total_driving_time);
        Ok(total_driving_time)
    }

    pub fn total_pure_driving_time(&mut self) -> Result<Duration, AnalyseError> {
        if let Some(value) = &self.cache.total_pure_driving_time {
            return Ok(*value);
        }

        let mut total_pure_driving_time = Duration::seconds(0);

        for analyser in self.analysers.iter() {
            total_pure_driving_time += analyser.pure_driving_time()?;
        }

        self.cache.total_pure_driving_time = Some(total_pure_driving_time);
        Ok(total_pure_driving_time)
    }
}