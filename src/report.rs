/*
 * File: report.rs
 * Description: The definition of the ReportFormat struct. Contains the format of the report
 * Author: dherslof
 * Created: 13-12-2024
 * License: MIT
 */

use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// Enum to represent the format of the report
#[derive(Clone, Debug)]
pub enum ReportFormat {
    Json,
    Csv,
    Yaml,
    Html,
    Pdf,
    Text,
}

/// Parse error for ReportFormat
#[derive(Debug)]
pub struct ParseReportFormatError;

/// Implement Display trait for ParseReportFormatError
impl fmt::Display for ParseReportFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid report format")
    }
}

/// Implement FromStr trait for ReportFormat
impl FromStr for ReportFormat {
    type Err = ParseReportFormatError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "json" => Ok(ReportFormat::Json),
            "csv" => Ok(ReportFormat::Csv),
            "yaml" => Ok(ReportFormat::Yaml),
            "html" => Ok(ReportFormat::Html),
            "pdf" => Ok(ReportFormat::Pdf),
            "text" => Ok(ReportFormat::Text),
            _ => Err(ParseReportFormatError),
        }
    }
}

/// Parse error for ReportFormat
#[derive(Debug)]
pub struct ReportGenerationFailure;

/// Implement Display trait for ReportGenerationFailure
impl fmt::Display for ReportGenerationFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to generate report")
    }
}

/// Implement Error trait for ReportGenerationFailure
impl Error for ReportGenerationFailure {}

// Todo: Container structs can maybe be done better.
//       Use a common container struct, and have 1 entry on format "foo:bar:42"
//       Then split on ":" and return a vector of strings which can be used to
//       as columns.

/// Struct to hold the column definitions in a project report
#[derive(Debug)]
pub struct ProjectReportColumns {
    pub first: String,
    pub second: String,
    pub third: String,
    pub fourth: String,
}

/// Struct to hold the column definitions in a project report
#[derive(Debug)]
pub struct WeekReportColumns {
    pub first: String,
    pub second: String,
    pub third: String,
    pub fourth: String,
    pub fifth: String,
    pub sixth: String,
    pub seventh: String,
}

#[derive(Debug)]
pub struct MonthReportColumns {
    pub first: String,
    pub second: String,
    pub third: String,
    pub fourth: String,
    pub fifth: String,
    pub sixth: String,
    pub seventh: String,
    pub eighth: String,
}

// Currently not used. Created for future implementation of a more detailed report summary.
/*
#[derive(Debug)]
pub enum ReportType {
    Full,
    Short,
}
*/
