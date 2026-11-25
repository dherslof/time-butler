/*
 * File: tables.rs
 * Description: The definition of the display tables for time items used by the butler.
 * Author: dherslof
 * Created: 27-04-2026
 * License: MIT
 */

use comfy_table::{Cell, ContentArrangement, Table};

use crate::day::Day;
use crate::entry::Entry;

/// Internal function to get the table for printing a day
pub fn get_table_day() -> Table {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Week"),
        Cell::new("Date"),
        Cell::new("Start time"),
        Cell::new("End time"),
        Cell::new("Paused hours"),
        Cell::new("Hours"),
        Cell::new("Closed"),
        Cell::new("Extra info"),
        Cell::new("ID"),
    ]);

    table
}

/// Internal function to get a table for printing a entry in a project
pub fn get_table_entry() -> Table {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Project"),
        Cell::new("Description"),
        Cell::new("Hours"),
        Cell::new("Created"),
        Cell::new("ID"),
    ]);

    table
}

/// Internal function to get a table for printing week target status
pub fn get_table_target_week() -> Table {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Week"),
        Cell::new("Target hours"),
        Cell::new("Current reported hours"),
        Cell::new("Percentage done"),
        Cell::new("Target status"),
        Cell::new("Hours remaining"),
        Cell::new("Hours overtime"),
        Cell::new("Target hours set method"),
    ]);

    table
}

/// Internal function to get a table for printing week target status
pub fn get_table_target_month() -> Table {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Month"),
        Cell::new("Target hours"),
        Cell::new("Current reported hours"),
        Cell::new("Percentage done"),
        Cell::new("Target status"),
        Cell::new("Hours remaining"),
        Cell::new("Hours overtime"),
        Cell::new("Target hours set method"),
    ]);

    table
}

// Internal function to print a single day, in report table format
pub fn print_day_in_report_table(day: &Day) {
    let mut table = get_table_day();

    let start_time = match day.starting_time() {
        Some(st) => st.to_string(),
        None => "N/A".to_string(),
    };

    let end_time = match day.ending_time() {
        Some(et) => et.to_string(),
        None => "N/A".to_string(),
    };

    table.add_row(vec![
        Cell::new(day.week().to_string()),
        Cell::new(day.date().to_string()),
        Cell::new(start_time),
        Cell::new(end_time),
        Cell::new(day.hours_paused().to_string()),
        Cell::new(day.hours().to_string()),
        Cell::new(day.closed().to_string()),
        Cell::new(day.extra_info()),
        Cell::new(day.id().to_string()),
    ]);

    println!("{}", table);
}

// Internal function to print a single entry, in report table format
pub fn print_entry_in_report_table(entry: &Entry, project_name: &str) {
    let mut table = get_table_entry();

    table.add_row(vec![
        Cell::new(project_name),
        Cell::new(entry.description()),
        Cell::new(entry.hours().to_string()),
        Cell::new(entry.created().to_string()),
        Cell::new(entry.id().to_string()),
    ]);

    println!("{}", table);
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local, TimeZone};

    #[test]
    fn test_get_table_day_contains_expected_headers() {
        let rendered = format!("{}", get_table_day());

        assert!(rendered.contains("Week"));
        assert!(rendered.contains("Date"));
        assert!(rendered.contains("Start"));
        assert!(rendered.contains("End"));
        assert!(rendered.contains("Paused"));
        assert!(rendered.contains("Hours"));
        assert!(rendered.contains("Closed"));
        assert!(rendered.contains("Extra"));
        assert!(rendered.contains("ID"));
    }

    #[test]
    fn test_get_table_entry_contains_expected_headers() {
        let rendered = format!("{}", get_table_entry());

        assert!(rendered.contains("Project"));
        assert!(rendered.contains("Description"));
        assert!(rendered.contains("Hours"));
        assert!(rendered.contains("Created"));
        assert!(rendered.contains("ID"));
    }

    #[test]
    fn test_get_table_target_week_contains_expected_headers() {
        let mut table = get_table_target_week();
        table.add_row(vec![
            Cell::new("W1"),
            Cell::new("40"),
            Cell::new("32"),
            Cell::new("80"),
            Cell::new("ONTRACK"),
            Cell::new("8"),
            Cell::new("0"),
            Cell::new("MANUAL"),
        ]);
        let rendered = format!("{}", table);

        assert!(rendered.contains("Week"));
        assert!(rendered.contains("W1"));
        assert!(rendered.contains("ONTRACK"));
        assert!(rendered.contains("MANUAL"));
    }

    #[test]
    fn test_get_table_target_month_contains_expected_headers() {
        let mut table = get_table_target_month();
        table.add_row(vec![
            Cell::new("Jan"),
            Cell::new("160"),
            Cell::new("120"),
            Cell::new("75"),
            Cell::new("IP"),
            Cell::new("40"),
            Cell::new("0"),
            Cell::new("CFG"),
        ]);
        let rendered = format!("{}", table);

        assert!(rendered.contains("Month"));
        assert!(rendered.contains("Jan"));
        assert!(rendered.contains("IP"));
        assert!(rendered.contains("CFG"));
    }

    #[test]
    fn test_print_day_in_report_table_smoke() {
        let mut day = Day::new(Some("Daily report".to_string()));
        let start = Local
            .with_ymd_and_hms(2026, 1, 20, 9, 0, 0)
            .single()
            .unwrap();
        let end = start + Duration::hours(8);
        day.set_starting_time(Some(&start));
        day.set_ending_time(Some(&end));
        day.set_paused_time(0.5);

        print_day_in_report_table(&day);
    }

    #[test]
    fn test_print_entry_in_report_table_smoke() {
        let entry = Entry::new(2.5, Some("Table print".to_string()));

        print_entry_in_report_table(&entry, "Test Project");
    }
}
