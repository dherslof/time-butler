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
        Cell::new(&day.week().to_string()),
        Cell::new(&day.date().to_string()),
        Cell::new(start_time),
        Cell::new(end_time),
        Cell::new(&day.hours_paused().to_string()),
        Cell::new(&day.hours().to_string()),
        Cell::new(&day.closed().to_string()),
        Cell::new(&day.extra_info()),
    ]);

    println!("{}", table);
}

// Internal function to print a single entry, in report table format
pub fn print_entry_in_report_table(entry: &Entry) {
    let mut table = get_table_entry();

    table.add_row(vec![
        Cell::new("Project"),
        Cell::new(entry.description()),
        Cell::new(&entry.hours().to_string()),
        Cell::new(&entry.created().to_string()),
        Cell::new(&entry.id().to_string()),
    ]);

    println!("{}", table);
}
