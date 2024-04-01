use time::Duration;
use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum AnalyseError {
    NoEntries,
    ZeroDistance,
    ZeroDrivingTime,
}

#[derive(PartialEq, Debug)]
pub struct ResultAnalyser {
    result: ZusiResult,
    // TODO: implement cache
}

impl ResultAnalyser {
    pub fn new(result: ZusiResult) -> ResultAnalyser {
        Self {
            result,
        }
    }

    /// Computes the distance for the whole route by using the `fahrt_weg` attribute.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    pub fn distance(&self) -> Result<f32, AnalyseError> {
        if self.result.value.len() > 0 {
            let ResultValue::FahrtEintrag(first) = self.result.value.first().unwrap();
            let ResultValue::FahrtEintrag(last) = self.result.value.last().unwrap();
            Ok(last.fahrt_weg - first.fahrt_weg)
        } else {
            Err(AnalyseError::NoEntries)
        }
    }

    /// Computes the average speed including idle times by using the overall driving time and distance.
    ///
    /// Throws [AnalyseError::ZeroDrivingTime] if the computed driving time is zero.
    pub fn average_speed(&self) -> Result<f32, AnalyseError> {
        let distance = self.distance()?;
        let driving_time = self.driving_time()?.as_seconds_f32();
        if driving_time == 0.0 {
            Err(AnalyseError::ZeroDrivingTime)
        } else {
            Ok(distance / driving_time)
        }
    }

    /// Computes the average speed excluding idle times.
    /// For each two [FahrtEintrag](ResultValue::FahrtEintrag) entries, the average speed between is computed.
    /// All these local average speeds will be averaged together weighted by their individual local distance.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    pub fn pure_average_speed(&self) -> Result<f32, AnalyseError> {
        if self.distance()? == 0. {
            Err(AnalyseError::ZeroDistance)
        } else if self.result.value.len() > 1 {
            let mut weighted_speed_sum = 0.;
            for i in 0..self.result.value.len() - 1 {
                let ResultValue::FahrtEintrag(current) = self.result.value.get(i).unwrap();
                let ResultValue::FahrtEintrag(next) = self.result.value.get(i + 1).unwrap();
                let local_average_speed = (current.fahrt_speed + next.fahrt_speed) / 2.;
                let local_distance = next.fahrt_weg - current.fahrt_weg;
                weighted_speed_sum += local_distance * local_average_speed;
            }
            Ok(weighted_speed_sum / self.distance()?)
        } else {
            Err(AnalyseError::NoEntries)
        }
    }

    /// Computes the whole driving time including idle times by using the `fahrt_zeit` attribute.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    pub fn driving_time(&self) -> Result<Duration, AnalyseError> {
        if self.result.value.len() > 0 {
            let ResultValue::FahrtEintrag(first) = self.result.value.first().unwrap();
            let ResultValue::FahrtEintrag(last) = self.result.value.last().unwrap();
            Ok(last.fahrt_zeit - first.fahrt_zeit)
        } else {
            Err(AnalyseError::NoEntries)
        }
    }

    /// Computes the whole driving time excluding idle times by omitting all periods with zero driving speed.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    pub fn pure_driving_time(&self) -> Result<Duration, AnalyseError> {
        if self.result.value.len() > 1 {
            let mut driving_time = Duration::seconds(0);
            for i in 0..self.result.value.len() - 1 {
                let ResultValue::FahrtEintrag(current) = self.result.value.get(i).unwrap();
                let ResultValue::FahrtEintrag(next) = self.result.value.get(i + 1).unwrap();
                if current.fahrt_speed > 0. || next.fahrt_speed > 0. {
                    driving_time += next.fahrt_zeit - current.fahrt_zeit;
                }
            }
            Ok(driving_time)
        } else if self.result.value.len() > 0 {
            Ok(Duration::seconds(0))
        } else {
            Err(AnalyseError::NoEntries)
        }
    }
}

impl AsRef<ResultAnalyser> for ResultAnalyser {
    fn as_ref(&self) -> &ResultAnalyser {
        &self
    }
}