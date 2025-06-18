/*
 * File: report_manager.rs
 * Description: ReportManager contains the functionality to create the time reports.
 * Author: dherslof
 * Created: 18-12-2024
 * License: MIT
 */

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use chrono::{DateTime, Local};
use csv::Writer;
use maud::{html, Markup, PreEscaped};
use serde_json::json;
use serde_json::Value;

use crate::day::Day;
use crate::project::Project;
use crate::week::Week;
// TODO: way to bundle the report stuff together?
use crate::report::MonthReportColumns;
use crate::report::ParseReportFormatError;
use crate::report::ProjectReportColumns;
use crate::report::ReportFormat;
use crate::report::ReportGenerationFailure;
use crate::report::WeekReportColumns;

//TODO: Improvement - Can the report creating functions be done in smarter way, feels stupid to repeat the same code for each report type

/// Report manager to handle report generation and storage
pub struct ReportManager {
    /// Path to the report storage directory
    default_report_dir: String,
    /// Default report file name
    default_report_file_name: String,
}

/// Report manager implementation
impl ReportManager {
    pub fn new() -> Self {
        Self {
            default_report_dir: format!(
                "{}/.local/time-butler/.generated_reports",
                std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
            ),
            default_report_file_name: format!(
                "{}_time_report.",
                Local::now().format("%Y-%m-%d_%H-%M-%S")
            )
            .to_string(),
        }
    }

    //TODO: add extra parameter here deciding if it should be summary or regular report
    /// Main function to generate a project report
    pub fn generate_project_report(
        &self,
        format: ReportFormat,
        project: &Project,
    ) -> Result<(), ReportGenerationFailure> {
        tracing::debug!("Setting report suffix");
        let report_suffix = match self.get_report_suffix(format.clone()) {
            Ok(suffix) => suffix,
            Err(_) => return Err(ReportGenerationFailure),
        };

        let file_name = format!(
            "{}_{}{}",
            project.name(),
            self.default_report_file_name,
            report_suffix
        );
        let file_path = format!("{}/{}", self.default_report_dir, file_name);
        tracing::debug!("report file set to: {}", file_path);

        match format {
            ReportFormat::Csv => match self.write_csv_project_report(project, None, &file_path) {
                Ok(_) => {
                    tracing::info!("Created report: {}", file_path);
                }
                Err(e) => {
                    tracing::error!("Error writing report: {}", e);
                    return Err(ReportGenerationFailure);
                }
            },
            ReportFormat::Json => match self.write_json_project_report(project, None, &file_path) {
                Ok(_) => {
                    tracing::info!("Created report: {}", file_path);
                }
                Err(e) => {
                    tracing::error!("Error writing report: {}", e);
                    return Err(ReportGenerationFailure);
                }
            },
            ReportFormat::Yaml => match self.write_yaml_project_report(project, None, &file_path) {
                Ok(_) => {
                    tracing::info!("Created report: {}", file_path);
                }
                Err(e) => {
                    tracing::error!("Error writing report: {}", e);
                    return Err(ReportGenerationFailure);
                }
            },
            ReportFormat::Html => match self.write_html_project_report(project, None, &file_path) {
                Ok(_) => {
                    tracing::info!("Created report: {}", file_path);
                }
                Err(e) => {
                    tracing::error!("Error writing report: {}", e);
                    return Err(ReportGenerationFailure);
                }
            },
            _ => {
                tracing::error!("Unsupported report format");
                return Err(ReportGenerationFailure);
            }
        }

        Ok(())
    }

    /// Internal function to get the suffix for the report file
    fn get_report_suffix(&self, format: ReportFormat) -> Result<String, ParseReportFormatError> {
        #[allow(unreachable_patterns)] // Suppresses the warning, but keep "_=> Err" for safety
        match format {
            ReportFormat::Json => Ok("json".to_string()),
            ReportFormat::Csv => Ok("csv".to_string()),
            ReportFormat::Yaml => Ok("yaml".to_string()),
            ReportFormat::Html => Ok("html".to_string()),
            ReportFormat::Pdf => Ok("pdf".to_string()),
            ReportFormat::Text => Ok("txt".to_string()),
            _ => Err(ParseReportFormatError),
        }
    }

    /// Internal function to write a CSV project report file
    fn write_csv_project_report(
        &self,
        project: &Project,
        columns: Option<ProjectReportColumns>,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        tracing::debug!("Writing CSV report");

        // Creating the file
        let mut writer = Writer::from_path(file_path)?;

        // Decide how to set up the headers based on the columns provided
        match columns {
            Some(c) => {
                writer.write_record(&[
                    c.first.as_str(),
                    c.second.as_str(),
                    c.third.as_str(),
                    c.fourth.as_str(),
                ])?;
            }
            None => {
                for e in project.entries() {
                    writer.serialize(e)?;
                }
            }
        }

        writer.flush()?;

        Ok(())
    }

    /// Internal function to write a JSON project report file
    fn write_json_project_report(
        &self,
        project: &Project,
        columns: Option<ProjectReportColumns>,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        tracing::debug!("Writing JSON report");

        let report_file = File::create(file_path)?;

        match columns {
            #[allow(unused_variables)]
            Some(c) => {
                tracing::warn!("Customized columns not supported for JSON project reports");
                return Err(Box::new(ReportGenerationFailure));
            }
            None => {
                serde_json::to_writer(report_file, project)?;
            }
        }

        return Ok(());
    }

    /// Internal function to write a YAML project report file
    fn write_yaml_project_report(
        &self,
        project: &Project,
        columns: Option<ProjectReportColumns>,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        tracing::debug!("Writing YAML report");

        let report_file = File::create(file_path)?;

        match columns {
            #[allow(unused_variables)]
            Some(c) => {
                tracing::warn!("Customized columns not supported for YAML project reports");
                return Err(Box::new(ReportGenerationFailure));
            }
            None => {
                serde_yaml::to_writer(report_file, project)?;
            }
        }

        return Ok(());
    }

    /// Internal function to write an HTML project report file
    fn write_html_project_report(
        &self,
        project: &Project,
        columns: Option<ProjectReportColumns>,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        tracing::debug!("Writing HTML report");

        let report_columns = match columns {
            Some(c) => {
                tracing::debug!("Using custom report columns provided");
                c
            }
            None => {
                tracing::debug!("Using default report columns");
                ProjectReportColumns {
                    first: "hours".to_string(),
                    second: "description".to_string(),
                    third: "created".to_string(),
                    fourth: "id".to_string(),
                }
            }
        };

        let headers = vec![
            report_columns.first,
            report_columns.second,
            report_columns.third,
            report_columns.fourth,
        ];

        // Rows from the project entries
        let rows: Vec<Vec<String>> = project
            .entries()
            .iter()
            .map(|entry| {
                vec![
                    entry.hours().to_string(),
                    entry.description().to_string(),
                    entry.created().to_string(),
                    entry.id().to_string(),
                ]
            })
            .collect();

        // Build the HTML markup
        let markup: Markup = html! {
            table border="1" {
                thead {
                    tr {
                        @for header in &headers {
                            th { (header) }
                        }
                    }
                }
                tbody {
                    @for row in &rows {
                        tr {
                            @for cell in row {
                                td { (cell) }
                            }
                        }
                    }
                }
            }
        };

        let mut file = File::create(file_path)?;
        file.write_all(markup.into_string().as_bytes())?;

        Ok(())
    }

    /// Main function to generate a week report
    pub fn generate_week_report(
        &self,
        format: ReportFormat,
        week: &Week,
    ) -> Result<(), ReportGenerationFailure> {
        tracing::debug!("Setting report suffix");
        let report_suffix = match self.get_report_suffix(format.clone()) {
            Ok(suffix) => suffix,
            Err(_) => return Err(ReportGenerationFailure),
        };

        let file_name = format!(
            "week{}_{}{}",
            week.number(),
            self.default_report_file_name,
            report_suffix
        );
        let file_path = format!("{}/{}", self.default_report_dir, file_name);
        tracing::debug!("report file set to: {}", file_path);

        let columns = WeekReportColumns {
            first: "Week".to_string(),
            second: "Date".to_string(),
            third: "StartingTime".to_string(),
            fourth: "EndingTime".to_string(),
            fifth: "Hours".to_string(),
            sixth: "Description".to_string(),
            seventh: "Closed".to_string(),
        };

        match format {
            ReportFormat::Csv => {
                match self.write_csv_week_report(week, Some(columns), &file_path) {
                    Ok(_) => {
                        tracing::info!("Created report: {}", file_path);
                    }
                    Err(e) => {
                        tracing::error!("Error writing report: {}", e);
                        return Err(ReportGenerationFailure);
                    }
                }
            }
            ReportFormat::Json => {
                match self.write_json_week_report(week, Some(columns), &file_path) {
                    Ok(_) => {
                        tracing::info!("Created report: {}", file_path);
                    }
                    Err(e) => {
                        tracing::error!("Error writing report: {}", e);
                        return Err(ReportGenerationFailure);
                    }
                }
            }
            ReportFormat::Yaml => {
                match self.write_yaml_week_report(week, Some(columns), &file_path) {
                    Ok(_) => {
                        tracing::info!("Created report: {}", file_path);
                    }
                    Err(e) => {
                        tracing::error!("Error writing report: {}", e);
                        return Err(ReportGenerationFailure);
                    }
                }
            }
            ReportFormat::Html => {
                match self.write_html_week_report(week, Some(columns), &file_path) {
                    Ok(_) => {
                        tracing::info!("Created report: {}", file_path);
                    }
                    Err(e) => {
                        tracing::error!("Error writing report: {}", e);
                        return Err(ReportGenerationFailure);
                    }
                }
            }
            _ => {
                tracing::error!("Unsupported report format");
                return Err(ReportGenerationFailure);
            }
        }

        Ok(())
    }

    /// Internal function to write a CSV week report file
    fn write_csv_week_report(
        &self,
        week: &Week,
        columns: Option<WeekReportColumns>,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        tracing::debug!("Writing CSV report");

        // Creating the file
        let mut writer = Writer::from_path(file_path)?;

        // Decide how to set up the headers based on the columns provided
        match columns {
            Some(c) => {
                writer.write_record(&[
                    c.first.as_str(),
                    c.second.as_str(),
                    c.third.as_str(),
                    c.fourth.as_str(),
                    c.fifth.as_str(),
                    c.sixth.as_str(),
                    c.seventh.as_str(),
                ])?;

                for d in week.entries() {
                    let formatted_start_time =
                        self.format_datetime_to_report_string(d.starting_time());

                    let formatted_end_time = self.format_datetime_to_report_string(d.ending_time());

                    writer.write_record(&[
                        d.week().to_string(),
                        d.date().to_string(),
                        formatted_start_time,
                        formatted_end_time,
                        d.hours().to_string(),
                        d.extra_info().to_string(),
                        d.closed().to_string(),
                    ])?;
                }
            }
            None => {
                for d in week.entries() {
                    writer.serialize(d)?;
                }
            }
        }

        writer.flush()?;

        Ok(())
    }

    /// Internal function to write a JSON week report file
    fn write_json_week_report(
        &self,
        week: &Week,
        columns: Option<WeekReportColumns>,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        tracing::debug!("Writing JSON report");

        let mut report_file = File::create(file_path)?;

        match columns {
            Some(c) => {
                let mut days_in_json = String::from("[");

                for d in week.entries() {
                    let formatted_start_time =
                        self.format_datetime_to_report_string(d.starting_time());

                    let formatted_end_time = self.format_datetime_to_report_string(d.ending_time());

                    let json_struct = format!(
                        r#"{{"{}": "{}", "{}": "{}", "{}": "{}", "{}": "{}", "{}": "{}", "{}": "{}"}}"#,
                        c.second,
                        d.date().to_string(),
                        c.third,
                        formatted_start_time,
                        c.fourth,
                        formatted_end_time,
                        c.fifth,
                        d.hours(),
                        c.sixth,
                        d.extra_info(),
                        c.seventh,
                        d.closed(),
                    );

                    days_in_json.push_str(&json_struct);
                }

                days_in_json.push(']');
                let json_report = format!(
                    r#"{{"{}": "{}", "{}": {}}}"#,
                    c.first,
                    week.number(),
                    "Days".to_string(),
                    days_in_json
                );
                report_file.write_all(json_report.as_bytes())?;
            }
            None => {
                tracing::warn!("Using raw JSON serialization for week report");
                serde_json::to_writer(report_file, week)?;
            }
        }

        return Ok(());
    }

    /// Internal function to write a YAML week report file
    fn write_yaml_week_report(
        &self,
        week: &Week,
        columns: Option<WeekReportColumns>,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        tracing::debug!("Writing YAML report");

        let mut report_file = File::create(file_path)?;

        match columns {
            Some(c) => {
                let mut days_in_yaml = String::from("");

                for d in week.entries() {
                    let formatted_start_time =
                        self.format_datetime_to_report_string(d.starting_time());

                    let formatted_end_time = self.format_datetime_to_report_string(d.ending_time());

                    let yaml_struct = format!(
                        "\n- {}: \"{}\"\n  {}: \"{}\"\n  {}: \"{}\"\n  {}: \"{}\"\n  {}: \"{}\"\n  {}: \"{}\"\n",
                        c.second,
                        d.date().to_string(),
                        c.third,
                        formatted_start_time,
                        c.fourth,
                        formatted_end_time,
                        c.fifth,
                        d.hours(),
                        c.sixth,
                        d.extra_info(),
                        c.seventh,
                        d.closed(),
                    );
                    days_in_yaml.push_str(&yaml_struct);
                }

                let yaml_report = format!(
                    "{}: {}\n{}: {}\n",
                    c.first,
                    week.number(),
                    "Days".to_string(),
                    days_in_yaml
                );
                report_file.write_all(yaml_report.as_bytes())?;
            }
            None => {
                tracing::warn!("Using raw YAML serialization for week report");
                serde_yaml::to_writer(report_file, week)?;
            }
        }

        return Ok(());
    }

    /// Internal function to write an HTML week report file
    fn write_html_week_report(
        &self,
        week: &Week,
        columns: Option<WeekReportColumns>,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        tracing::debug!("Writing HTML report");

        let report_columns = match columns {
            Some(c) => {
                tracing::debug!("Using custom report columns provided");
                c
            }
            None => {
                tracing::warn!("Using default report columns");
                WeekReportColumns {
                    first: "Week".to_string(),
                    second: "Date".to_string(),
                    third: "StartingTime".to_string(),
                    fourth: "EndingTime".to_string(),
                    fifth: "Hours".to_string(),
                    sixth: "Description".to_string(),
                    seventh: "Closed".to_string(),
                }
            }
        };

        let headers = vec![
            report_columns.first,
            report_columns.second,
            report_columns.third,
            report_columns.fourth,
            report_columns.fifth,
            report_columns.sixth,
            report_columns.seventh,
        ];

        // Rows from the project entries
        let rows: Vec<Vec<String>> = week
            .entries()
            .iter()
            .map(|entry| {
                vec![
                    week.number().to_string(),
                    entry.date().to_string(),
                    self.format_datetime_to_report_string(entry.starting_time()),
                    self.format_datetime_to_report_string(entry.ending_time()),
                    entry.hours().to_string(),
                    entry.extra_info().to_string(),
                    entry.closed().to_string(),
                ]
            })
            .collect();

        // Build the HTML markup
        let markup: Markup = html! {
            table border="1" {
                thead {
                    tr {
                        @for header in &headers {
                            th { (header) }
                        }
                    }
                }
                tbody {
                    @for row in &rows {
                        tr {
                            @for cell in row {
                                td { (cell) }
                            }
                        }
                    }
                }
            }
        };

        let mut file = File::create(file_path)?;
        file.write_all(markup.into_string().as_bytes())?;

        Ok(())
    }

    /// Internal support function to format a datetime to a report string
    fn format_datetime_to_report_string(&self, t: Option<&DateTime<Local>>) -> String {
        t.map(|time| time.format("%H:%M:%S").to_string())
            .unwrap_or_else(|| "N/A".to_string())
    }

    pub fn generate_month_report(
        &self,
        month_number: u32,
        format: ReportFormat,
        days_in_month: &Vec<Day>,
    ) -> Result<(), ReportGenerationFailure> {
        tracing::debug!("Setting report suffix");
        let report_suffix = match self.get_report_suffix(format.clone()) {
            Ok(suffix) => suffix,
            Err(_) => return Err(ReportGenerationFailure),
        };

        let file_name = format!(
            "month{}_{}{}",
            month_number, self.default_report_file_name, report_suffix
        );

        let file_path = format!("{}/{}", self.default_report_dir, file_name);
        tracing::debug!("report file set to: {}", file_path);

        // Same as for week report
        let columns = MonthReportColumns {
            first: "Month".to_string(),
            second: "Week".to_string(),
            third: "Date".to_string(),
            fourth: "StartingTime".to_string(),
            fifth: "EndingTime".to_string(),
            sixth: "Hours".to_string(),
            seventh: "Description".to_string(),
            eighth: "Closed".to_string(),
        };

        match format {
            ReportFormat::Csv => {
                match self.write_csv_month_report(
                    month_number,
                    days_in_month,
                    Some(columns),
                    &file_path,
                ) {
                    Ok(_) => {
                        tracing::info!("Created report: {}", file_path);
                    }
                    Err(e) => {
                        tracing::error!("Error writing report: {}", e);
                        return Err(ReportGenerationFailure);
                    }
                }
            }
            ReportFormat::Json => {
                match self.write_json_month_report(
                    month_number,
                    days_in_month,
                    Some(columns),
                    &file_path,
                ) {
                    Ok(_) => {
                        tracing::info!("Created report: {}", file_path);
                    }
                    Err(e) => {
                        tracing::error!("Error writing report: {}", e);
                        return Err(ReportGenerationFailure);
                    }
                }
            }
            ReportFormat::Yaml => {
                match self.write_yaml_month_report(
                    month_number,
                    days_in_month,
                    Some(columns),
                    &file_path,
                ) {
                    Ok(_) => {
                        tracing::info!("Created report: {}", file_path);
                    }
                    Err(e) => {
                        tracing::error!("Error writing report: {}", e);
                        return Err(ReportGenerationFailure);
                    }
                }
            }
            ReportFormat::Html => {
                match self.write_html_month_report(
                    month_number,
                    days_in_month,
                    Some(columns),
                    &file_path,
                ) {
                    Ok(_) => {
                        tracing::info!("Created report: {}", file_path);
                    }
                    Err(e) => {
                        tracing::error!("Error writing report: {}", e);
                        return Err(ReportGenerationFailure);
                    }
                }
            }
            _ => {
                tracing::error!("Unsupported report format");
                return Err(ReportGenerationFailure);
            }
        }

        Ok(())
    }

    fn write_csv_month_report(
        &self,
        month_number: u32,
        month_days: &Vec<Day>,
        columns: Option<MonthReportColumns>,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        tracing::debug!("Writing CSV report");

        // Creating the file
        let mut writer = Writer::from_path(file_path)?;

        // Decide how to set up the headers based on the columns provided
        match columns {
            Some(c) => {
                writer.write_record(&[
                    c.first.as_str(),
                    c.second.as_str(),
                    c.third.as_str(),
                    c.fourth.as_str(),
                    c.fifth.as_str(),
                    c.sixth.as_str(),
                    c.seventh.as_str(),
                    c.eighth.as_str(),
                ])?;

                for d in month_days {
                    let formatted_start_time =
                        self.format_datetime_to_report_string(d.starting_time());

                    let formatted_end_time = self.format_datetime_to_report_string(d.ending_time());

                    writer.write_record(&[
                        month_number.to_string(),
                        d.week().to_string(),
                        d.date().to_string(),
                        formatted_start_time,
                        formatted_end_time,
                        d.hours().to_string(),
                        d.extra_info().to_string(),
                        d.closed().to_string(),
                    ])?;
                }
            }
            None => {
                for d in month_days {
                    writer.serialize(d)?;
                }
            }
        }

        writer.flush()?;

        Ok(())
    }

    fn write_json_month_report(
        &self,
        month_number: u32,
        month_days: &Vec<Day>,
        columns: Option<MonthReportColumns>,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        tracing::debug!("Writing JSON report");

        let mut report_file = File::create(file_path)?;

        match columns {
            Some(c) => {
                let mut weeks_map: HashMap<u32, Vec<Value>> = HashMap::new();

                // Group days by week number
                for d in month_days {
                    let week_number = d.week();

                    let formatted_start_time =
                        self.format_datetime_to_report_string(d.starting_time());
                    let formatted_end_time = self.format_datetime_to_report_string(d.ending_time());

                    let day_json = json!({
                        &c.third: d.date().to_string(),
                        &c.fourth: formatted_start_time,
                        &c.fifth: formatted_end_time,
                        &c.sixth: d.hours(),
                        &c.seventh: d.extra_info(),
                        &c.eighth: d.closed()
                    });

                    weeks_map
                        .entry(week_number)
                        .or_insert_with(Vec::new)
                        .push(day_json);
                }

                // Convert the grouped data into a structured JSON
                let mut weeks_json = Vec::new();
                for (week, days) in weeks_map {
                    weeks_json.push(json!({
                        &c.second: week,
                        "Days": days
                    }));
                }

                // Construct the final JSON report
                let json_report = json!({
                    c.first: month_number,
                    "Weeks": weeks_json
                });
                report_file.write_all(json_report.to_string().as_bytes())?;
            }
            None => {
                tracing::warn!("Columns not provided! Using JSON serialization for week report");
                unimplemented!(); //TODO: Implement this
            }
        }
        return Ok(());
    }

    fn write_yaml_month_report(
        &self,
        month_number: u32,
        month_days: &Vec<Day>,
        columns: Option<MonthReportColumns>,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        tracing::debug!("Writing YAML report");
        let mut report_file = File::create(file_path)?;

        match columns {
            Some(c) => {
                let mut weeks_map: HashMap<u32, Vec<serde_yaml::Value>> = HashMap::new();

                // Group days by week number
                for d in month_days {
                    let week_number = d.week();

                    let formatted_start_time =
                        self.format_datetime_to_report_string(d.starting_time());
                    let formatted_end_time = self.format_datetime_to_report_string(d.ending_time());

                    let mut day_yaml = serde_yaml::Mapping::new();
                    day_yaml.insert(
                        serde_yaml::Value::String(c.third.clone()),
                        serde_yaml::Value::String(d.date().to_string()),
                    );
                    day_yaml.insert(
                        serde_yaml::Value::String(c.fourth.clone()),
                        serde_yaml::Value::String(formatted_start_time),
                    );
                    day_yaml.insert(
                        serde_yaml::Value::String(c.fifth.clone()),
                        serde_yaml::Value::String(formatted_end_time),
                    );
                    day_yaml.insert(
                        serde_yaml::Value::String(c.sixth.clone()),
                        serde_yaml::Value::Number(d.hours().into()),
                    );
                    day_yaml.insert(
                        serde_yaml::Value::String(c.seventh.clone()),
                        serde_yaml::Value::String(d.extra_info().to_string()),
                    );
                    day_yaml.insert(
                        serde_yaml::Value::String(c.eighth.clone()),
                        serde_yaml::Value::Bool(d.closed()),
                    );

                    weeks_map
                        .entry(week_number)
                        .or_insert_with(Vec::new)
                        .push(serde_yaml::Value::Mapping(day_yaml));
                }

                // Convert the grouped data into a structured YAML format
                let mut weeks_yaml = Vec::new();
                for (week, days) in weeks_map {
                    let mut week_entry = serde_yaml::Mapping::new();
                    week_entry.insert(
                        serde_yaml::Value::String(c.second.clone()),
                        serde_yaml::Value::Number(week.into()),
                    );
                    week_entry.insert(
                        serde_yaml::Value::String("Days".to_string()),
                        serde_yaml::Value::Sequence(days),
                    );
                    weeks_yaml.push(serde_yaml::Value::Mapping(week_entry));
                }

                // Construct the final YAML report
                let mut yaml_report = serde_yaml::Mapping::new();
                yaml_report.insert(
                    serde_yaml::Value::String(c.first.clone()),
                    serde_yaml::Value::Number(month_number.into()),
                );
                yaml_report.insert(
                    serde_yaml::Value::String("Weeks".to_string()),
                    serde_yaml::Value::Sequence(weeks_yaml),
                );

                let yaml_string = serde_yaml::to_string(&yaml_report)?;
                report_file.write_all(yaml_string.as_bytes())?;
            }
            None => {
                tracing::warn!("Columns not provided! Using YAML serialization for week report");
                unimplemented!(); //TODO: Implement this
            }
        }
        return Ok(());
    }

    fn write_html_month_report(
        &self,
        month_number: u32,
        month_days: &Vec<Day>,
        columns: Option<MonthReportColumns>,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        tracing::debug!("Writing HTML report");

        let mut file = File::create(file_path)?;

        match columns {
            Some(c) => {
                let mut weeks_map: HashMap<u32, Vec<Vec<String>>> = HashMap::new();

                // Group days by week number
                for d in month_days {
                    let week_number = d.week();
                    let formatted_start_time = d
                        .starting_time()
                        .map_or_else(|| "N/A".to_string(), |t| t.to_string());
                    let formatted_end_time = d
                        .ending_time()
                        .map_or_else(|| "N/A".to_string(), |t| t.to_string());

                    let row = vec![
                        d.date().to_string(),
                        formatted_start_time,
                        formatted_end_time,
                        d.hours().to_string(),
                        d.extra_info().to_string(),
                        d.closed().to_string(),
                    ];

                    weeks_map
                        .entry(week_number)
                        .or_insert_with(Vec::new)
                        .push(row);
                }

                // Build the HTML markup
                let markup: Markup = html! {
                    html {
                        head {
                            title { "Weekly Report" }
                            style { (PreEscaped("
                                table { border-collapse: collapse; width: 100%; }
                                th, td { border: 1px solid black; padding: 8px; text-align: left; }
                                th { background-color: #f2f2f2; }
                            ")) }
                        }
                        body {
                            h1 { (format!("Monthly Report - Month {}", month_number)) }
                            @for (week, days) in &weeks_map {
                                h2 { (format!("Week {}", week)) }
                                table border="1" {
                                    thead {
                                        tr {
                                            th { (c.third) }
                                            th { (c.fourth) }
                                            th { (c.fifth) }
                                            th { (c.sixth) }
                                            th { (c.seventh) }
                                            th { (c.eighth) }
                                        }
                                    }
                                    tbody {
                                        @for row in days {
                                            tr {
                                                @for cell in row {
                                                    td { (cell) }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                };
                file.write_all(markup.into_string().as_bytes())?;
            }
            None => {
                tracing::warn!("Columns not provided! Using YAML serialization for week report");
                unimplemented!(); //TODO: Implement this
            }
        }
        Ok(())
    }
}
