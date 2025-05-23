/*
 * File: main.rs
 * Description: The main file, user entry and interaction point. The butler is controlled from here.
 * Author: dherslof
 * Created: 13-12-2024
 * License: MIT
 */

mod butler;
mod cli;
mod cli_interactor;
mod day;
mod entry;
mod project;
mod report;
mod report_manager;
mod storage_handler;
mod week;
mod target;

use cli::{AddSubcommands, Cli, Commands, RemoveSubcommands, ReportSubcommands, TargetTimesSubcommands};
use std::process;
use tracing::Level;
use tracing_subscriber::EnvFilter;

use butler::Butler;
use clap::Parser;

// Error codes
const K_BUTLER_SAVE_FAILED: i32 = 1;

fn main() {
    // Parse the CLI arguments
    let args = Cli::parse();

    // Initialize the logger based on cli arguments
    if args.json {
        if args.verbose {
            tracing_subscriber::fmt()
                .json() // Output logs in JSON format
                .with_max_level(Level::DEBUG) // Log only INFO level and higher
                .with_env_filter(EnvFilter::from_default_env()) // Optional: filter logs based on an environment variable
                .init();
        } else {
            tracing_subscriber::fmt()
                .json() // Output logs in JSON format
                .with_max_level(Level::INFO) // Log only INFO level and higher
                .with_env_filter(EnvFilter::from_default_env()) // Optional: filter logs based on an environment variable
                .init();
        }
    } else {
        if args.verbose {
            tracing_subscriber::fmt()
                .with_max_level(Level::DEBUG) // Log messages of `DEBUG` level and higher
                .init();
        } else {
            tracing_subscriber::fmt()
                .with_max_level(Level::INFO) // Log messages of `INFO` level and higher
                .init();
        }
    }

    tracing::debug!("Creating the Butler!");

    let mut store_data = false;
    let mut butler = Butler::new();

    butler.init();

    match args.command {
        Commands::Interactive => {
            tracing::debug!("Starting in interactive mode");

            // True or false depending on if data needs to be stored
            if butler.interact_with_user() {
                store_data = true;
            }
        }
        Commands::Add { entity } => match entity {
            AddSubcommands::Project { name, description } => {
                tracing::debug!("Adding new project");

                let new_project = project::Project::new(name, description);
                if butler.add_project(new_project) {
                    tracing::info!("Project added successfully!");
                    store_data = true;
                } else {
                    tracing::info!("Failed to add project!");
                }
            }
            AddSubcommands::Entry {
                project,
                hours,
                description,
            } => {
                tracing::debug!("Adding new entry");
                let e = entry::Entry::new(hours, Some(description));
                if butler.add_entry(&project, e) {
                    tracing::info!("Entry added successfully!");
                    store_data = true;
                } else {
                    tracing::info!("Failed to add entry!");
                }
            }
            AddSubcommands::Day {
                extra_info,
                starting_time,
                ending_time,
            } => {
                tracing::debug!("Adding new day");
                let mut d = day::Day::new(extra_info);

                if starting_time == true {
                    d.set_starting_time(Some(&chrono::Local::now()));
                }

                if ending_time == true {
                    d.set_ending_time(Some(&chrono::Local::now()));
                }

                if butler.add_day(d) {
                    tracing::info!("Day added successfully!");
                    store_data = true;
                } else {
                    tracing::info!("Failed to add day!");
                }
            }
        },
        Commands::Report { entity } => match entity {
            ReportSubcommands::Project { name, format } => {
                tracing::debug!("Generating Project report");
                if butler.project_report(&name, &format) {
                    tracing::info!("Project report generated successfully!");
                }
            }
            ReportSubcommands::Week { number, format } => {
                tracing::debug!("Generating Week report");
                if butler.week_report(number, &format) {
                    tracing::info!("Report for week {} generated successfully!", number);
                }
            }
            ReportSubcommands::Month { number, format } => {
                tracing::debug!("Generating Month report");
                if butler.month_report(number, &format) {
                    tracing::info!("Report for month {} generated successfully!", number);
                }
            }
            ReportSubcommands::Year { number, format } => {
                tracing::debug!("Generating Year report");
                unimplemented!();
            }
        },
        Commands::List {
            project,
            week,
            month,
            all_weeks,
            all_projects,
        } => {
            tracing::debug!("List selected entities");

            if all_weeks {
                tracing::info!("Listing all weeks");
                butler.list_all_weeks();
            }

            if all_projects {
                tracing::info!("Listing all projects");
                butler.list_all_projects();
            }

            // Select what user want to list
            match project {
                Some(proj_name) => {
                    tracing::debug!("Project specified: {}", proj_name);
                    butler.list_specific_project(&proj_name);
                }
                None => {
                    tracing::debug!("No specific project specified, no projects will be listed");
                }
            }

            match week {
                Some(week) => {
                    tracing::debug!("Week specified: {}", week);
                    // Convert string to int
                    let w: u32 = week.parse().unwrap();
                    butler.list_specific_week(w);
                }
                None => {
                    tracing::debug!("No specific week specified, no weeks will be listed");
                }
            }

            match month {
                Some(month) => {
                    tracing::debug!("Month specified: {}", month);
                    // Convert string to int
                    let m: u32 = month.parse().unwrap();
                    butler.list_specific_month(m);
                }
                None => {
                    tracing::debug!("No specific month specified, no months will be listed");
                }
            }
        }
        Commands::Remove { entity } => match entity {
            RemoveSubcommands::Project { name } => {
                tracing::debug!("Removing project");
                if !butler.remove_project(&name) {
                    tracing::info!("Failed to remove project!");
                } else {
                    tracing::info!("Project removed successfully!");
                    store_data = true;
                }
            }
            RemoveSubcommands::Entry { project, id } => {
                tracing::debug!("Removing entry");
                if !butler.remove_entry(&project, id.unwrap_or("".to_string())) {
                    tracing::info!("Failed to remove entry!");
                } else {
                    tracing::info!("Entry removed successfully!");
                    store_data = true;
                }
            }
            RemoveSubcommands::Day { date, week } => {
                tracing::debug!("Removing day");
                if !butler.remove_day(week, date) {
                    tracing::info!("Failed to remove day!");
                } else {
                    tracing::info!("Day removed successfully!");
                    store_data = true;
                }
            }
        },
        Commands::Modify { .. } => todo!(), // all other commands
        Commands::Info {short } => {
            tracing::debug!("Displaying storage info!");
            butler.self_info(short);

        }
        Commands::Targets {entity } => match entity {
            TargetTimesSubcommands::Week { number } => {
                tracing::debug!("Displaying target times for week {}", number);
                if !butler.display_week_target_status(number) {
                    tracing::error!("Failed to display target times for week {}", number);
                } else {
                    tracing::info!("Target times for week {} displayed successfully!", number);
                }
            }
            TargetTimesSubcommands::Month { number } => {
                tracing::debug!("Displaying target times for month{}", number);
                if !butler.display_month_target_status(number) {
                    tracing::error!("Failed to display target times for week {}", number);
                } else {
                    tracing::info!("Target times for week {} displayed successfully!", number);
                }
            }
        }
    }

    if store_data {
        if !butler.save() {
            tracing::error!("Failed to save butler data, added entry will not be stored properly");
            process::exit(K_BUTLER_SAVE_FAILED);
        }
    }
}
