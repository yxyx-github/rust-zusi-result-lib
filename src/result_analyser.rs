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