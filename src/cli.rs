/*
 * File: cli.rs
 * Description: The definition of the Command Line Interface for the time-butler.
 * Author: dherslof
 * Created: 18-12-2024
 * License: MIT
 */

use clap::{Parser, Subcommand};

/// Struct to define the CLI structure
#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),         // Package name from Cargo.toml
    version = env!("CARGO_PKG_VERSION"),   // Version from Cargo.toml
    about = env!("CARGO_PKG_DESCRIPTION"), // Description from Cargo.toml
    long_about = "A tool to report time on different projects. This can be done by two ways from the CLI, interactive and direct."
)]
#[command(about = "A tool to report time on different projects", long_about = None)]
pub struct Cli {
    /// Command to be selected
    #[command(subcommand)]
    pub command: Commands,
    /// Verbose logging flag
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,
    /// Generate JSON output from logging
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub json: bool,
    /// Configuration file path. Absolute path.
    #[arg(short, long, default_value = "tb-config.json")]
    pub config: String,
}

/// Enum to define available subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Report time interactively
    Interactive,
    /// Report time with input arguments
    Add {
        #[command(subcommand)]
        entity: AddSubcommands,
    },
    /// Generate a time overview report
    Report {
        #[command(subcommand)]
        entity: ReportSubcommands,
    },
    /// List already reported items
    List {
        /// Project name
        #[arg(short, long)]
        project: Option<String>,
        /// Week number - if multiple weeks exists with same number, all will be listed
        #[arg(short, long)]
        week: Option<String>,
        /// Month number - if multiple months exists with same number, all will be listed
        #[arg(short, long)]
        month: Option<String>,
        /// Display all weeks
        #[arg(long, action = clap::ArgAction::SetTrue)]
        all_weeks: bool,
        /// Display all projects
        #[arg(long, action = clap::ArgAction::SetTrue)]
        all_projects: bool,
    },
    /// Remove a already store item
    Remove {
        /// Project name
        #[command(subcommand)]
        entity: RemoveSubcommands, // This should maybe be a separate enum, in order to not need to provide the same arguments
    },
    /// Modify a project or entry
    Modify {
        /// Project name
        #[arg(short, long)]
        project: Option<String>,
        /// Entry ID
        #[arg(short, long)]
        entry: Option<String>,
        /// Day date
        #[arg(short, long)]
        day: Option<String>,
    },

    /// Short Info regarding internal storage
    Info {
        /// Short summary
        #[arg(long, action = clap::ArgAction::SetTrue)]
        short: bool,
    },

    /// Time targets for the week/month/year
    Targets {
        /// Target times info
        #[command(subcommand)]
        entity: TargetTimesSubcommands,
    },
}

/// Enum for "add" subcommands
#[derive(Subcommand)]
pub enum AddSubcommands {
    /// Add a new project
    Project {
        /// Project name
        #[arg(short, long)]
        name: String,
        /// Description of the project
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Add a new time entry to an existing project
    Entry {
        /// Project name
        #[arg(long)]
        project: String,
        /// Hours worked
        #[arg(long)]
        hours: u32,
        /// Description of the work done
        #[arg(long)]
        description: String,
    },
    /// Add new day
    Day {
        /// Extra info for the day
        #[arg(short, long)]
        extra_info: Option<String>,
        /// Starting time flag, if set to true, the starting time will be set to the current time.
        #[arg(long, action = clap::ArgAction::SetTrue)]
        starting_time: bool,
        /// Ending time flag, if set to true, the ending time will be set to the current time.
        #[arg(long, action = clap::ArgAction::SetTrue)]
        ending_time: bool,
    },
}

/// Enum for "remove" subcommands
#[derive(Subcommand)]
pub enum RemoveSubcommands {
    /// Remove a project and all its entries
    Project {
        /// Project name
        #[arg(short, long)]
        name: String,
    },
    /// Remove a specific entry from a project
    Entry {
        /// Project name
        #[arg(long)]
        project: String,
        /// Entry ID
        #[arg(long)]
        id: Option<String>,
    },
    /// Remove a specific day from a week
    Day {
        /// Date
        #[arg(long)]
        date: String,
        /// Week number
        #[arg(long)]
        week: u32,
    },
}

/// Enum for "remove" subcommands
#[derive(Subcommand)]
pub enum ModifySubcommands {
    Project {
        /// Project name
        #[arg(short, long)]
        name: Option<String>,
        /// description of the project
        #[arg(short, long)]
        description: Option<String>,
    },
    //TODO: add more sub-options, day, entry, etc.
}

#[derive(Subcommand)]
pub enum ReportSubcommands {
    /// Project report
    Project {
        /// Project name
        #[arg(short, long)]
        name: String,
        /// Report format, valid options are: "json, csv, yaml, html, pdf, text"`
        #[arg(short, long)]
        format: String,
    },
    /// Week report
    Week {
        /// Week number
        #[arg(short, long)]
        number: u32,
        /// Year number
        #[arg(short, long)]
        year: u32,
        /// Report format, valid options are: "json, csv, yaml, html, pdf, text"`
        #[arg(short, long)]
        format: String,
    },
    /// Month report
    Month {
        /// Month number
        #[arg(short, long)]
        number: u32,
        /// Year number
        #[arg(short, long)]
        year: u32,
        /// Report format, valid options are: "json, csv, yaml, html, pdf, text"`
        #[arg(short, long)]
        format: String,
    },
    /// Year report
    Year {
        /// Year number
        #[arg(short, long)]
        number: u32,
        /// Report format, valid options are: "json, csv, yaml, html, pdf, text"`
        #[arg(short, long)]
        format: String,
    },
}

#[derive(Subcommand)]
pub enum TargetTimesSubcommands {
    /// Set target for the week
    Week {
        /// Week number
        #[arg(short, long)]
        number: u32,
        /// Year number
        #[arg(short, long)]
        year: u32,
    },
    /// Set target for the month
    Month {
        /// Month number
        #[arg(short, long)]
        number: u32,
        /// Year number
        #[arg(short, long)]
        year: u32,
    },
}

//TODO:
// * Extend the CLI with entries for month and year as well
// * Add modify subcommand
// * Add subcommand for summary of the report, both when generating and taking file as input
