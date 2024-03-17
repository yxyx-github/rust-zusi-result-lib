use zusi_xml_lib::xml::zusi::result::ZusiResult;

pub struct ResultAnalyser {
    result: ZusiResult,
}

impl ResultAnalyser {
    pub fn new(result: ZusiResult) -> ResultAnalyser {
        Self {
            result,
        }
    }
}