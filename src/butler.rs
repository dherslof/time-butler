/*
 * File: butler.rs
 * Description: The main entry point for all functions.
 * Author: dherslof
 * Created: 13-12-2024
 * License: MIT
 */

use chrono::Datelike;
use comfy_table::{Cell, ContentArrangement, Table};
use std::collections::BTreeMap;
use std::io::{self, Write};
use uuid::Uuid;

use crate::cli_interactor::{CliCommand, CliInteractor};
use crate::config::AppConfiguration;
use crate::day::Day;
use crate::entry::Entry;
use crate::project::Project;
use crate::report::ReportFormat;
use crate::report_manager::ReportManager;
use crate::storage_handler::StorageHandler;
use crate::target::{MonthlyTargetStatus, WeeklyTargetStatus};
use crate::week::Week;

/// Butler struct - Main star of the show
pub struct Butler {
    /// Projects vector
    projects: Vec<Project>,
    // Week vector
    weeks: Vec<Week>,
    // Report functionality
    report_mngr: ReportManager,
    /// Storage functionality
    storage_handler: StorageHandler,
    /// User interaction functionality
    user_interactor: CliInteractor,
    /// Configuration
    configuration: AppConfiguration,
}

/// Implementation of the functionality for the Butler
impl Butler {
    /// Create a new Butler
    pub fn new(storage_handler: StorageHandler, configuration: AppConfiguration) -> Self {
        Self {
            projects: Vec::new(),
            weeks: Vec::new(),
            report_mngr: ReportManager::new(),
            storage_handler,
            user_interactor: CliInteractor::new(),
            configuration,
        }
    }

    /// Internal function for prompting user for confirmation. Used in interactive mode
    fn prompt_user_confirmation(question: &str) -> bool {
        let promt = format!("{} [y/N]: ", question);
        print!("{}", promt);
        io::stdout().flush().expect("Failed to flush stdout");

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Normalize and match the response
        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            _ => {
                tracing::warn!("Not confirmed by user");
                return false;
            }
        }
    }

    /// Internal function to get number of weeks currently stored
    fn number_of_weeks(&self) -> usize {
        self.weeks.len()
    }

    /// Internal function to get number of projects currently stored
    fn number_of_projects(&self) -> usize {
        self.projects.len()
    }

    /// Init the butler in order to get the saved data from the storage etc.
    pub fn init(&mut self) {
        tracing::debug!("Initializing the Butler!");

        // Update the file paths based on configuratoin
        self.storage_handler
            .set_paths_from_config(&self.configuration);

        // Load projects from storage
        if let Some(projects) = self.storage_handler.load_projects() {
            self.projects = projects;
        } else {
            tracing::error!("Failed to load projects from storage");
        }
        tracing::debug!("Loaded {} projects", self.number_of_projects());

        if let Some(weeks) = self.storage_handler.load_weeks() {
            self.weeks = weeks;
        } else {
            tracing::error!("Failed to load weeks from storage");
        }
        tracing::debug!("Loaded {} weeks", self.weeks.len());
    }

    /// Display information about the Butler
    pub fn self_info(&self, short: bool) {
        if short {
            // Display info in normal log print

            let output_str = format!(
                "\nNumber of stored projects: {}\nNumber of stored weeks: {}",
                self.number_of_projects(),
                self.number_of_weeks(),
            );

            tracing::info!("{}", output_str);
        } else {
            tracing::warn!("Only short info implemented so far..sorry!")
        }
    }

    /// Add a new project to the Butler
    pub fn add_project(&mut self, project: Project) -> bool {
        //search the project list for the project name
        for p in &self.projects {
            if p.name() == project.name() {
                tracing::error!(
                    "Project with name {} already exists in list, unable to add project",
                    project.name()
                );
                return false;
            }
        }

        tracing::info!(
            "Adding new project with name: {}, and description: {}",
            project.name(),
            project.description().unwrap_or("")
        );
        tracing::debug!("Project will be stored with ID: {}", project.id());
        self.projects.push(project);

        return true;
    }

    /// Create a new project report
    pub fn project_report(&self, project_name: &str, format: &str) -> bool {
        let report_format = match format {
            "json" => ReportFormat::Json,
            "csv" => ReportFormat::Csv,
            "yaml" => ReportFormat::Yaml,
            "html" => ReportFormat::Html,
            "pdf" => ReportFormat::Pdf,
            "text" => ReportFormat::Text,
            _ => {
                tracing::error!("Invalid format: {}", format);
                return false;
            }
        };
        // Search for the project
        for p in &self.projects {
            if p.name() == project_name {
                match self.storage_handler.create_report_dir() {
                    Ok(_) => (),
                    Err(e) => {
                        tracing::error!("Failed to create report directory: {}", e);
                        return false;
                    }
                }

                let generation_result =
                    match self.report_mngr.generate_project_report(report_format, &p) {
                        Ok(_) => true,
                        Err(e) => {
                            tracing::error!("failed to generate report: {}", e);
                            false
                        }
                    };

                return generation_result;
            }
        }

        tracing::error!("Project with name {} not found", project_name);
        return false;
    }

    /// Create a new week report
    pub fn week_report(&self, week_number: u32, format: &str, year: u32) -> bool {
        let report_format = match format {
            "json" => ReportFormat::Json,
            "csv" => ReportFormat::Csv,
            "yaml" => ReportFormat::Yaml,
            "html" => ReportFormat::Html,
            "pdf" => ReportFormat::Pdf,
            "text" => ReportFormat::Text,
            _ => {
                tracing::error!("Invalid format: {}", format);
                return false;
            }
        };

        // Search for the week with both week number and year
        for w in &self.weeks {
            if w.number() == week_number && w.year() as u32 == year {
                match self.storage_handler.create_report_dir() {
                    Ok(_) => (),
                    Err(e) => {
                        tracing::error!("Failed to create report directory: {}", e);
                        return false;
                    }
                }

                let generation_result =
                    match self.report_mngr.generate_week_report(report_format, &w) {
                        Ok(_) => true,
                        Err(e) => {
                            tracing::error!("failed to generate report: {}", e);
                            false
                        }
                    };

                return generation_result;
            }
        }

        tracing::error!(
            "Week with number {} and year {} not found",
            week_number,
            year
        );
        return false;
    }

    pub fn month_report(&self, month_number: u32, format: &str, year: u32) -> bool {
        let report_format = match format {
            "json" => ReportFormat::Json,
            "csv" => ReportFormat::Csv,
            "yaml" => ReportFormat::Yaml,
            "html" => ReportFormat::Html,
            //"pdf" => ReportFormat::Pdf,
            //"text" => ReportFormat::Text,
            _ => {
                tracing::error!("Invalid format: {}", format);
                return false;
            }
        };

        if month_number < 1 || month_number > 12 {
            tracing::error!("Invalid month number: {}", month_number);
            return false;
        }

        // Only include days from the specified year
        let days: Vec<Day> = self
            .get_days_in_month(month_number)
            .into_iter()
            .filter(|d| d.year() as u32 == year)
            .collect();

        if days.is_empty() {
            tracing::warn!(
                "No days found for month: {} and year: {}",
                month_number,
                year
            );
            return false;
        }

        match self.storage_handler.create_report_dir() {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("Failed to create report directory: {}", e);
                return false;
            }
        }

        let generation_result =
            match self
                .report_mngr
                .generate_month_report(month_number, report_format, &days)
            {
                Ok(_) => true,
                Err(e) => {
                    tracing::error!("failed to generate report: {}", e);
                    false
                }
            };

        return generation_result;
    }

    /// List all projects
    pub fn list_all_projects(&self) {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);

        table.set_header(vec![
            Cell::new("Name"),
            Cell::new("Description"),
            Cell::new("Number of Entries"),
            Cell::new("ID"),
        ]);

        for p in &self.projects {
            table.add_row(vec![
                Cell::new(p.name()),
                Cell::new(p.description().unwrap_or("")),
                Cell::new(&p.entries().len().to_string()),
                Cell::new(&p.id().to_string()),
            ]);
        }

        println!("{}", table);
    }

    /// List a specific project, will show all entries stored for that specific project
    pub fn list_specific_project(&self, project_name: &str) {
        for p in &self.projects {
            if p.name() == project_name {
                let mut table = Self::get_table_entry();

                for e in p.entries() {
                    table.add_row(vec![
                        Cell::new(p.name()),
                        Cell::new(e.description()),
                        Cell::new(&e.hours().to_string()),
                        Cell::new(&e.created().to_string()),
                        Cell::new(&e.id().to_string()),
                    ]);
                }

                println!("{}", table);
                return;
            }
        }

        tracing::error!("Project with name {} not found", project_name);
    }

    /// List all weeks stored, doesn't show the days stored in the weeks
    pub fn list_all_weeks(&self) {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);

        table.set_header(vec![
            Cell::new("Year"),
            Cell::new("Week"),
            Cell::new("Number of days registered"),
        ]);

        for w in &self.weeks {
            table.add_row(vec![
                Cell::new(&w.year().to_string()),
                Cell::new(&w.number().to_string()),
                Cell::new(&w.entries().len().to_string()),
            ]);
        }

        println!("{}", table);
    }

    /// List a specific week, will show all days stored for that specific week
    pub fn list_specific_week(&self, week_number: u32) {
        // Group weeks by year (for easier reading)
        let mut weeks_by_year: BTreeMap<i32, &Week> = BTreeMap::new();
        for w in &self.weeks {
            if w.number() == week_number {
                weeks_by_year.insert(w.year(), w);
            }
        }

        if weeks_by_year.is_empty() {
            tracing::error!("Week with number {} not found", week_number);
            return;
        }

        for (year, week) in weeks_by_year {
            println!("Year: {}", year);
            let mut table = Self::get_table_day();

            for d in week.entries() {
                let start_time = match d.starting_time() {
                    Some(st) => st.to_string(),
                    None => "N/A".to_string(),
                };

                let end_time = match d.ending_time() {
                    Some(et) => et.to_string(),
                    None => "N/A".to_string(),
                };

                table.add_row(vec![
                    Cell::new(&d.week().to_string()),
                    Cell::new(&d.date().to_string()),
                    Cell::new(start_time),
                    Cell::new(end_time),
                    Cell::new(&d.hours().to_string()),
                    Cell::new(&d.closed().to_string()),
                    Cell::new(&d.extra_info()),
                ]);
            }

            println!("{}", table);
        }
    }

    pub fn list_specific_month(&self, month_number: u32) {
        if month_number < 1 || month_number > 12 {
            tracing::error!("Invalid month number: {}", month_number);
            return;
        }

        let days = self.get_days_in_month(month_number);
        if days.is_empty() {
            tracing::warn!("No days found for month: {}", month_number);
            return;
        }

        use std::collections::BTreeMap;
        let mut days_by_year: BTreeMap<i32, Vec<Day>> = BTreeMap::new();
        for d in days {
            days_by_year.entry(d.year()).or_default().push(d);
        }

        for (year, days) in days_by_year {
            println!("Year: {}", year);
            let mut table = Self::get_table_day();

            for d in days {
                let start_time = match d.starting_time() {
                    Some(st) => st.to_string(),
                    None => "N/A".to_string(),
                };

                let end_time = match d.ending_time() {
                    Some(et) => et.to_string(),
                    None => "N/A".to_string(),
                };

                table.add_row(vec![
                    Cell::new(&d.week().to_string()),
                    Cell::new(&d.date().to_string()),
                    Cell::new(start_time),
                    Cell::new(end_time),
                    Cell::new(&d.hours().to_string()),
                    Cell::new(&d.closed().to_string()),
                    Cell::new(&d.extra_info()),
                ]);
            }

            println!("{}", table);
        }
    }

    /// Add new entry to project
    pub fn add_entry(&mut self, project_name: &str, entry: Entry) -> bool {
        // search for the project
        for p in &mut self.projects {
            if p.name() == project_name {
                // Get the entry ID before ownership transfer
                let entry_clone = entry.clone();
                p.add_entry(entry);

                // Print new entry as confirmation to user
                Self::print_entry_in_report_table(&entry_clone);
                return true;
            }
        }

        tracing::error!(
            "Project with name {} not found, unable to add entry",
            project_name
        );
        return false;
    }

    /// Add new day to a week
    pub fn add_day(&mut self, day: Day) -> bool {
        // search for the Week
        if day.week() > 52 && day.week() < 1 {
            tracing::error!("Week number is invalid: {}, unable to add day", day.week());
            return false;
        }

        // If no week exists, no idea to search and do the potential merge. Just create and add
        if self.weeks.is_empty() {
            tracing::info!("Weeks list is empty, creating new week");
            let mut new_week = Week::new(
                day.week(),
                day.year(),
                self.configuration.week_target_hours(),
            );
            // Print the new added day as confirmation to user, quite nice verification
            Self::print_day_in_report_table(&day);
            new_week.add_entry(day);
            self.weeks.push(new_week);
            return true;
        } else {
            let last_item_index = self.weeks.len() - 1;
            // If weeks exists, search for the week and add the day
            for (i, w) in self.weeks.iter_mut().enumerate() {
                // Find correct week
                if w.number() == day.week() && w.year() == day.year() {
                    // Correct week found
                    if w.exists(&day) {
                        tracing::info!(
                            "Day already exists in week {}, merging day entries",
                            day.week()
                        );

                        if w.merge_day(&day) {
                            tracing::info!("Day merged successfully");
                        } else {
                            tracing::info!("Failed to merge day");
                            return false;
                        }

                        tracing::debug!("Day: {}, updated in week:{}", day.date(), day.week());
                        let day_cpy = w.get_day_copy(&day.date()).unwrap(); // safe since day exists already

                        // Print the new added day as confirmation to user, quite nice verification
                        Self::print_day_in_report_table(&day_cpy);
                        return true;
                    } else {
                        // Day don't exists in week
                        tracing::debug!(
                            "Week:{} not found, adding new day: {}",
                            day.week(),
                            day.date()
                        );
                        tracing::debug!("Day added to week {}", day.week());
                        // Get the date of newly added day, since ownership is moved to week
                        let new_day_date = day.date();
                        w.add_entry(day);

                        let day_cpy = w.get_day_copy(&new_day_date).unwrap(); // safe since day exists already

                        // Print the new added day as confirmation to user, quite nice verification
                        Self::print_day_in_report_table(&day_cpy);
                        return true;
                    }
                } else {
                    // Week not found check if last element in week list
                    if i == last_item_index {
                        // Last element in list, create new week
                        tracing::debug!("Didn't find week {}, creating new week", day.week());
                        let mut new_week = Week::new(
                            day.week(),
                            day.year(),
                            self.configuration.week_target_hours(),
                        );

                        // Print the new added day as confirmation to user, before adding to week and loose ownership
                        Self::print_day_in_report_table(&day);

                        new_week.add_entry(day);
                        self.weeks.push(new_week);

                        return true;
                    }
                    // Not last element, continue searching
                }
            }
        }
        return false;
    }

    /// Save the butler data to storage, in bin format
    pub fn save(&self) -> bool {
        tracing::debug!("Saving data to storage");

        let project_storage_result =
            match self.storage_handler.store_projects(self.projects.clone()) {
                Ok(_) => true,
                Err(e) => {
                    tracing::error!("Failed to save projects to storage: {}", e);
                    false
                }
            };

        let week_storage_result = match self.storage_handler.store_weeks(self.weeks.clone()) {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("Failed to save weeks to storage: {}", e);
                false
            }
        };

        if !project_storage_result || !week_storage_result {
            tracing::debug!("Save failed");
            return false;
        }

        self.storage_handler.backup_storage_files(
            self.configuration.periodic_backup_enabled(),
            self.configuration.override_existing_backup(),
            self.configuration.periodic_backup_interval(),
        );

        return true;
    }

    pub fn force_backup(&self) -> bool {
        tracing::info!("Forcing backup of time-butler data");
        match self.storage_handler.do_backup_now() {
            Ok(_) => {
                tracing::info!("Backup completed successfully");
                return true;
            }
            Err(e) => {
                tracing::error!("Backup failed: {}", e);
                return false;
            }
        }
    }

    /// Remove a project from the Butler
    pub fn remove_project(&mut self, project_name: &str) -> bool {
        let mut index = 0;
        let mut found = false;
        for (i, p) in self.projects.iter().enumerate() {
            if p.name() == project_name {
                tracing::debug!("Project found: {}, at index: {}", p.name(), i);
                index = i;
                found = true;
                break;
            }
        }

        if found {
            if Self::prompt_user_confirmation(&format!(
                "Are you sure you want to remove {}",
                project_name
            )) {
                self.projects.remove(index);
                tracing::debug!("Project {}, removed", project_name);
                return true;
            } else {
                tracing::info!("Confirmation not given, aborting");
                return false;
            }
        } else {
            tracing::warn!("Project with name {} not found", project_name);
            return false;
        }
    }

    /// Remove an entry from a project
    pub fn remove_entry(&mut self, project: &str, id: String) -> bool {
        if self.projects.is_empty() {
            tracing::warn!("No projects stored, unable to remove entry");
            return false;
        }

        let parsed_id = match Uuid::parse_str(&id) {
            Ok(parsed_id) => {
                tracing::debug!("Parsed ID: {}", parsed_id);
                parsed_id
            }
            Err(e) => {
                tracing::error!("Failed to parse ID: {}", e);
                return false;
            }
        };

        // Search for project, if project exists -> search for the entry
        for p in &mut self.projects {
            if p.name() == project {
                if !p.entry_exists(&parsed_id) {
                    tracing::warn!(
                        "Entry with ID: {} not found in project: {}",
                        parsed_id.to_string(),
                        project
                    );
                    return false;
                }

                let entry_cpy = p.get_entry_copy(&parsed_id).unwrap(); // safe since we know it exists

                let mut table = Table::new();
                table.set_content_arrangement(ContentArrangement::Dynamic);

                table.set_header(vec![
                    Cell::new("ID"),
                    Cell::new("Description"),
                    Cell::new("Hours"),
                    Cell::new("Created"),
                ]);

                table.add_row(vec![
                    Cell::new(entry_cpy.id().to_string()),
                    Cell::new(entry_cpy.description()),
                    Cell::new(entry_cpy.hours().to_string()),
                    Cell::new(entry_cpy.created().to_string()),
                ]);
                println!("{}", table);

                if Self::prompt_user_confirmation(&format!(
                    "Are you sure you want to remove entry {}",
                    parsed_id.to_string()
                )) {
                    if p.remove_listed_entry(&parsed_id) {
                        tracing::info!(
                            "Entry {}, removed from project {}",
                            parsed_id.to_string(),
                            project
                        );
                        return true;
                    } else {
                        tracing::warn!("Failed to remove entry {}", parsed_id.to_string());
                        return false;
                    }
                } else {
                    tracing::warn!("Confirmation not given, aborting");
                    return false;
                }
            }
        }
        tracing::warn!("Project with name {} not found", project);
        return false;
    }

    pub fn remove_day(&mut self, week: u32, date: String) -> bool {
        if self.weeks.is_empty() {
            tracing::warn!("No weeks stored, unable to remove day");
            return false;
        }

        let date_format = "%Y-%m-%d";
        let parsed_date = match chrono::NaiveDate::parse_from_str(&date, date_format) {
            Ok(parsed_date) => {
                tracing::debug!("Parsed date: {}", parsed_date);
                parsed_date
            }
            Err(e) => {
                tracing::error!("Failed to parse date: {}, for format: y-m-d", e);
                return false;
            }
        };

        let year = parsed_date.year();

        // Search for week with both week number and year
        for w in &mut self.weeks {
            if w.number() == week && w.year() == year {
                if !w.exist(&parsed_date) {
                    tracing::warn!(
                        "Day with date {} not found in week {} year {}",
                        date,
                        week,
                        year
                    );
                    return false;
                }

                let day_cpy = w.get_day_copy(&parsed_date).unwrap(); // safe since we know it exists

                let mut table = Self::get_table_day();

                table.add_row(vec![
                    Cell::new(&day_cpy.week().to_string()),
                    Cell::new(&day_cpy.date().to_string()),
                    Cell::new(
                        day_cpy
                            .starting_time()
                            .map(|dt| dt.to_string())
                            .unwrap_or_else(|| "N/A".to_string()),
                    ),
                    Cell::new(
                        day_cpy
                            .ending_time()
                            .map(|dt| dt.to_string())
                            .unwrap_or_else(|| "N/A".to_string()),
                    ),
                    Cell::new(&day_cpy.hours().to_string()),
                    Cell::new(&day_cpy.closed().to_string()),
                    Cell::new(&day_cpy.extra_info()),
                ]);
                println!("{}", table);
                if Self::prompt_user_confirmation(&format!(
                    "Are you sure you want to remove day {}",
                    parsed_date
                )) {
                    // Remove day
                    w.remove_listed_day(&parsed_date);
                    tracing::info!(
                        "Day {}, removed from week {} year {}",
                        parsed_date,
                        week,
                        year
                    );
                    return true;
                } else {
                    tracing::warn!("Confirmation not given, aborting");
                    return false;
                }
            }
        }

        tracing::warn!(
            "Day with date {} not found in week {} year {}",
            date,
            week,
            year
        );
        return false;
    }

    /// Start the interaction with the user. This is the main loop for the interactive mode
    pub fn interact_with_user(&mut self) -> bool {
        loop {
            match self.user_interactor.start_user_interaction() {
                Ok(CliCommand::AddProject) => {
                    // Get project parameters
                    let project_name = match self.user_interactor.get_project_name() {
                        Ok(name) => name,
                        Err(e) => {
                            tracing::error!("Failed to get project name: {}", e);
                            return false;
                        }
                    };
                    tracing::debug!("New project name: {}", project_name);

                    let project_description = match self.user_interactor.get_project_description() {
                        Ok(desc) => desc,
                        Err(e) => {
                            tracing::error!("Failed to get project description: {}", e);
                            return false;
                        }
                    };
                    tracing::debug!("New project description received");

                    // Add project
                    if self.add_project(Project::new(project_name, Some(project_description))) {
                        tracing::info!("Project added successfully!");
                        return true;
                    } else {
                        tracing::info!("Failed to add project!");
                        return false;
                    }
                }

                Ok(CliCommand::AddEntry) => {
                    if self.number_of_projects() == 0 {
                        self.user_interactor
                            .print_msg_to_user("No projects stored, unable to add entry. Please add a new project first.");
                        return false;
                    }

                    self.display_stored_projects();

                    let project_name = match self.user_interactor.get_entry_project() {
                        Ok(name) => name,
                        Err(e) => {
                            tracing::error!("Failed to get project name: {}", e);
                            return false;
                        }
                    };

                    let entry_description = match self.user_interactor.get_entry_description() {
                        Ok(desc) => desc,
                        Err(e) => {
                            tracing::error!("Failed to get entry description: {}", e);
                            return false;
                        }
                    };

                    let entry_hours = match self.user_interactor.get_entry_hours() {
                        Ok(hours) => hours,
                        Err(e) => {
                            tracing::error!("Failed to get entry hours: {}", e);
                            return false;
                        }
                    };

                    if self.add_entry(
                        &project_name,
                        Entry::new(entry_hours, Some(entry_description)),
                    ) {
                        tracing::info!("Entry added successfully!");
                        return true;
                    } else {
                        tracing::info!("Failed to add entry!");
                        return false;
                    }

                    // Add interactive logic here
                }
                Ok(CliCommand::AddDay) => {
                    let description = self.user_interactor.get_day_description();
                    let mut new_day = Day::new(Some(description.unwrap())); // safe since it will contain empty string in worst case

                    if let Ok(true) = self.user_interactor.get_day_starting_time() {
                        new_day.set_starting_time(Some(&chrono::Local::now()));
                    }

                    if let Ok(true) = self.user_interactor.get_day_ending_time() {
                        new_day.set_ending_time(Some(&chrono::Local::now()));
                    }

                    if self.add_day(new_day) {
                        tracing::info!("Day added successfully!");
                        return true;
                    } else {
                        tracing::info!("Failed to add day!");
                        return false;
                    }

                    // Add interactive logic here
                }
                Ok(CliCommand::ListProjects) => {
                    match self.user_interactor.get_list_projects(&self.projects) {
                        Ok((true, None)) => {
                            self.list_all_projects();
                        }
                        Ok((false, Some(project_name))) => {
                            self.list_specific_project(&project_name);
                        }
                        Err(e) => {
                            tracing::error!("An error occurred while listing projects: {}", e);
                        }
                        _ => {
                            tracing::warn!("Unexpected option selected");
                        }
                    }
                    return false;
                }
                Ok(CliCommand::ListWeeks) => {
                    match self.user_interactor.get_list_weeks(&self.weeks) {
                        Ok((true, None)) => {
                            self.list_all_weeks();
                        }
                        Ok((false, Some(week_number))) => {
                            self.list_specific_week(week_number);
                        }
                        Err(e) => {
                            tracing::error!("An error occurred while listing weeks: {}", e);
                        }
                        _ => {
                            tracing::warn!("Unexpected option selected");
                        }
                    }
                    return false;
                }
                Ok(CliCommand::Exit) => {
                    tracing::debug!("Exiting...");
                    return false;
                }
                Err(e) => {
                    tracing::error!("Error: {}", e);
                    return false;
                }
            }
        }
    }

    pub fn display_week_target_status(&self, week: u32, year: u32) -> bool {
        if self.weeks.is_empty() {
            tracing::warn!("No weeks stored, unable to display weekly target status");
            return false;
        }

        for w in &self.weeks {
            if w.number() == week && w.year() as u32 == year {
                let status = WeeklyTargetStatus::new(&w, &w.target_hours());
                let mut table = Self::get_table_target_week();
                table.add_row(vec![
                    Cell::new(week),
                    Cell::new(status.target_hours().to_string()),
                    Cell::new(status.status_hours().to_string()),
                    Cell::new(status.percentage().to_string()),
                    Cell::new(status.target_status_string()),
                    if status.remaining_hours() > &0.0 {
                        Cell::new(status.remaining_hours().to_string())
                    } else {
                        Cell::new("0.0")
                    },
                    if status.remaining_hours() < &0.0 {
                        Cell::new(status.hours_difference().to_string())
                    } else {
                        Cell::new("0.0")
                    },
                    Cell::new(status.target_set_method_string()),
                ]);

                println!("{}", table);
                return true;
            }
        }

        tracing::warn!("Week with number {} not found", week);
        return false;
    }

    pub fn display_month_target_status(&self, month_number: u32, year: u32) -> bool {
        if month_number < 1 || month_number > 12 {
            tracing::error!("Invalid month number: {}", month_number);
            return false;
        }

        let days_vec = self.get_days_in_month_for_year(month_number, year);
        if days_vec.is_empty() {
            tracing::warn!("No days found for month: {}", month_number);
            return false;
        }

        let weeks_in_month = self.get_weeks_in_month(month_number);
        if weeks_in_month.is_empty() {
            tracing::warn!("No weeks found for month: {}", month_number);
            return false;
        }

        let mut month_target_hours: f32 = 0.0;
        if self.configuration.weekly_target_for_month() {
            tracing::debug!("Calculating month target hours based on weekly target hours");
            // Calculate month target hours based on number of weeks in month and weekly target hours
            for w in &weeks_in_month {
                month_target_hours += w.target_hours();
            }
        } else {
            tracing::debug!("Calculating month target hours based on configuration value");
            month_target_hours = self.configuration.month_target_hours();
        }

        let status = MonthlyTargetStatus::new(&days_vec, &month_target_hours);
        let mut table = Self::get_table_target_month();

        table.add_row(vec![
            Cell::new(month_number),
            Cell::new(status.target_hours().to_string()),
            Cell::new(status.status_hours().to_string()),
            Cell::new(status.percentage().to_string()),
            Cell::new(status.target_status_string()),
            if status.remaining_hours() > &0.0 {
                Cell::new(status.remaining_hours().to_string())
            } else {
                Cell::new("0.0")
            },
            if status.remaining_hours() < &0.0 {
                Cell::new(status.hours_difference().to_string())
            } else {
                Cell::new("0.0")
            },
            Cell::new(status.target_set_method_string()),
        ]);

        println!("{}", table);
        return true;
    }

    pub fn dump_configuration_to_terminal(&self, configuration_file_path: String) {
        let config_str = self.configuration.get_as_string();
        println!(
            "Current configuration (from {}):\n{}",
            configuration_file_path, config_str
        );
    }

    pub fn dump_configuration_to_file(&self, configuration_file_path: String, output_file: String) {
        let config_str = self.configuration.get_as_string();
        let output = format!(
            "Current configuration (from {}):\n{}",
            configuration_file_path, config_str
        );

        match std::fs::write(&output_file, output) {
            Ok(_) => {
                tracing::debug!("Configuration successfully dumped to file: {}", output_file);
            }
            Err(e) => {
                tracing::error!(
                    "Failed to write configuration to file {}: {}",
                    output_file,
                    e
                );
            }
        }
    }

    fn get_days_in_month(&self, month: u32) -> Vec<Day> {
        let mut days = Vec::new();
        for w in &self.weeks {
            for d in w.entries() {
                if d.month() == month {
                    days.push(d.clone());
                }
            }
        }
        days
    }

    fn get_days_in_month_for_year(&self, month: u32, year: u32) -> Vec<Day> {
        let mut days = Vec::new();
        for w in &self.weeks {
            for d in w.entries() {
                if d.month() == month && d.year() as u32 == year {
                    days.push(d.clone());
                }
            }
        }
        days
    }

    // Prep for future functionality, when the year report is implemented
    #[allow(dead_code)]
    fn get_days_in_year(&self, year: i32) -> Vec<Day> {
        let mut days = Vec::new();
        for w in &self.weeks {
            for d in w.entries() {
                if d.year() == year {
                    days.push(d.clone());
                }
            }
        }
        days
    }

    /// Get the weeks in a month
    fn get_weeks_in_month(&self, month: u32) -> Vec<Week> {
        let mut weeks = Vec::new();
        for w in &self.weeks {
            for d in w.entries() {
                if d.month() == month {
                    weeks.push(w.clone());
                    break; // No need to check other days in the week
                }
            }
        }
        weeks
    }

    /// Display current stored projects
    fn display_stored_projects(&self) {
        // Display stored projects - mainly for interactive mode
        for p in &self.projects {
            println!("Current stored projects: ");
            println!("- {}", p.name());
        }
    }

    /// Internal function to get the table for printing a day
    fn get_table_day() -> Table {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);

        table.set_header(vec![
            Cell::new("Week"),
            Cell::new("Date"),
            Cell::new("Start time"),
            Cell::new("End time"),
            Cell::new("Hours"),
            Cell::new("Closed"),
            Cell::new("Extra info"),
        ]);

        table
    }

    /// Internal function to get a table for printing a entry in a project
    fn get_table_entry() -> Table {
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
    fn get_table_target_week() -> Table {
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
    fn get_table_target_month() -> Table {
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
    fn print_day_in_report_table(day: &Day) {
        let mut table = Self::get_table_day();

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
            Cell::new(&day.hours().to_string()),
            Cell::new(&day.closed().to_string()),
            Cell::new(&day.extra_info()),
        ]);

        println!("{}", table);
    }

    // Internal function to print a single entry, in report table format
    fn print_entry_in_report_table(entry: &Entry) {
        let mut table = Self::get_table_entry();

        table.add_row(vec![
            Cell::new("Project"),
            Cell::new(entry.description()),
            Cell::new(&entry.hours().to_string()),
            Cell::new(&entry.created().to_string()),
            Cell::new(&entry.id().to_string()),
        ]);

        println!("{}", table);
    }
}
