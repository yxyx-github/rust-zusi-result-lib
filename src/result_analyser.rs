#[cfg(test)]
mod tests;

use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};

#[derive(PartialEq, Debug)]
pub enum AnalyseError {
    NoEntriesFound,
    ZeroDistance,
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
        if self.distance()? == 0.0 {
            Err(AnalyseError::ZeroDistance)
        } else if self.result.value.len() > 1 {
            let mut weighted_speed_sum = 0.0;
            for i in 0..self.result.value.len() - 1 {
                let ResultValue::FahrtEintrag(current) = self.result.value.get(i).unwrap();
                let ResultValue::FahrtEintrag(next) = self.result.value.get(i + 1).unwrap();
                let local_average_speed = (current.fahrt_speed + next.fahrt_speed) / 2.0;
                let local_distance = next.fahrt_weg - current.fahrt_weg;
                weighted_speed_sum += local_distance * local_average_speed;
            }
            Ok(weighted_speed_sum / self.distance()?)
        } else {
            Err(AnalyseError::NoEntriesFound)
        }
    }

    // TODO: add average_moving_speed
}