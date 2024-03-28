//! This library provides some basic analysis for `.result.xml` files generated by [Zusi 3](https://www.zusi.de/).
//! For parsing the xml [zusi_xml_lib] is used.

/// Contains everything for analysing a single `.result.xml` file.
pub mod result_analyser;

/// Contains everything for analysing multiple `.result.xml` files by aggregating the single results.
pub mod result_analyser_group;