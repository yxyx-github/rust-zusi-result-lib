use time::Duration;

#[derive(PartialEq, Debug)]
pub(super) struct AnalyserGroupCache {
    pub(super) total_distance: Option<f32>,
    pub(super) average_distance: Option<f32>,
    pub(super) average_speed: Option<f32>,
    pub(super) pure_average_speed: Option<f32>,
    pub(super) total_driving_time: Option<Duration>,
    pub(super) total_pure_driving_time: Option<Duration>,
}

impl AnalyserGroupCache {
    pub fn new() -> AnalyserGroupCache {
        Self {
            total_distance: None,
            average_distance: None,
            average_speed: None,
            pure_average_speed: None,
            total_driving_time: None,
            total_pure_driving_time: None,
        }
    }
}