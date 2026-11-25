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
    pub eighth: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    // --- ReportFormat::from_str ---

    #[test]
    fn test_report_format_from_str_all_valid_variants() {
        assert!(matches!(
            "json".parse::<ReportFormat>(),
            Ok(ReportFormat::Json)
        ));
        assert!(matches!(
            "csv".parse::<ReportFormat>(),
            Ok(ReportFormat::Csv)
        ));
        assert!(matches!(
            "yaml".parse::<ReportFormat>(),
            Ok(ReportFormat::Yaml)
        ));
        assert!(matches!(
            "html".parse::<ReportFormat>(),
            Ok(ReportFormat::Html)
        ));
        assert!(matches!(
            "pdf".parse::<ReportFormat>(),
            Ok(ReportFormat::Pdf)
        ));
        assert!(matches!(
            "text".parse::<ReportFormat>(),
            Ok(ReportFormat::Text)
        ));
    }

    #[test]
    fn test_report_format_from_str_is_case_insensitive() {
        assert!(matches!(
            "JSON".parse::<ReportFormat>(),
            Ok(ReportFormat::Json)
        ));
        assert!(matches!(
            "CSV".parse::<ReportFormat>(),
            Ok(ReportFormat::Csv)
        ));
        assert!(matches!(
            "YAML".parse::<ReportFormat>(),
            Ok(ReportFormat::Yaml)
        ));
        assert!(matches!(
            "HTML".parse::<ReportFormat>(),
            Ok(ReportFormat::Html)
        ));
        assert!(matches!(
            "PDF".parse::<ReportFormat>(),
            Ok(ReportFormat::Pdf)
        ));
        assert!(matches!(
            "TEXT".parse::<ReportFormat>(),
            Ok(ReportFormat::Text)
        ));
    }

    #[test]
    fn test_report_format_from_str_unknown_returns_error() {
        assert!("xml".parse::<ReportFormat>().is_err());
        assert!("".parse::<ReportFormat>().is_err());
        assert!("markdown".parse::<ReportFormat>().is_err());
    }

    // --- Clone and Debug ---

    #[test]
    fn test_report_format_clone() {
        let original = ReportFormat::Json;
        let cloned = original.clone();
        assert!(matches!(cloned, ReportFormat::Json));
    }

    #[test]
    fn test_report_format_debug_is_non_empty() {
        let fmt = format!("{:?}", ReportFormat::Csv);
        assert!(!fmt.is_empty());
    }

    // --- ParseReportFormatError Display ---

    #[test]
    fn test_parse_report_format_error_display() {
        let err = ParseReportFormatError;
        assert_eq!(format!("{}", err), "invalid report format");
    }

    #[test]
    fn test_parse_report_format_error_debug_is_non_empty() {
        let err = ParseReportFormatError;
        assert!(!format!("{:?}", err).is_empty());
    }

    // --- ReportGenerationFailure Display and Error ---

    #[test]
    fn test_report_generation_failure_display() {
        let err = ReportGenerationFailure;
        assert_eq!(format!("{}", err), "Failed to generate report");
    }

    #[test]
    fn test_report_generation_failure_is_error_trait() {
        let err = ReportGenerationFailure;
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_report_generation_failure_debug_is_non_empty() {
        let err = ReportGenerationFailure;
        assert!(!format!("{:?}", err).is_empty());
    }
}
