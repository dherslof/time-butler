/*
 * File: cli.rs
 * Description: The cli user interactor functionality for the butler.
 * Author: dherslof
 * Created: 13-12-2024
 * License: MIT
 */

use std::error::Error;
use std::fmt;
use std::io;

use crate::project::Project;
use crate::week::Week;

pub enum CliCommand {
    AddProject,
    AddEntry,
    AddDay,
    ListProjects,
    ListWeeks,
    Exit,
}

/// Parse error for ReportFormat. Allow dead code now, since it might be used later. If not used in release 1.0.0 - Remove it
#[allow(dead_code)]
#[derive(Debug)]
pub struct UserInteractionFailure;

// Implement Display trait for UserInteractionFailure
impl fmt::Display for UserInteractionFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The user interaction failed")
    }
}

// Implement Error trait for UserInteractionFailure
impl Error for UserInteractionFailure {}

/// The CliInteractor struct
pub struct CliInteractor {}

/// Implementation for CliInteractor functionality
impl CliInteractor {
    pub fn new() -> Self {
        Self {}
    }

    /// Generic function to print a message to the user from the butler
    pub fn print_msg_to_user(&self, message: &str) {
        println!("{}", message);
    }

    /// The entry point and first message to the user in interactive mode
    pub fn start_user_interaction(&self) -> Result<CliCommand, Box<dyn std::error::Error>> {
        tracing::debug!("Starting user interaction");
        // Starting point of user interaction, program "main menu"

        loop {
            println!("Welcome to the TimeButler's interactive mode!");
            println!("Here you will be guided through the functionalities of the program.");
            println!(" ");
            println!("More functionalities will be added in the future. Use CLI for full set of features.");
            println!(" ");
            println!("Please select an option:");
            println!("1. Add a new project");
            println!("2. Add a new entry to a project");
            println!("3. Add a new work day");
            println!("4. List project or projects");
            println!("5. List entry or entries for a week");
            println!("6. Exit");

            // Read user input
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            match input.trim() {
                "1" => return Ok(CliCommand::AddProject),
                "2" => return Ok(CliCommand::AddEntry),
                "3" => return Ok(CliCommand::AddDay),
                "4" => return Ok(CliCommand::ListProjects),
                "5" => return Ok(CliCommand::ListWeeks),
                "6" => {
                    println!("Exiting...");
                    return Ok(CliCommand::Exit);
                }
                _ => println!("Invalid option, please try again."),
            }
        }
    }

    /// Function to get the name of a project from the user
    pub fn get_project_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.print_and_read_input("Please enter the name of the project:")
    }

    /// Function to get the description of a project from the user
    pub fn get_project_description(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.print_and_read_input("Please enter the description of the project:")
    }

    /// Function to get the name of a project from the user
    pub fn get_entry_project(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.print_and_read_input(
            "Please enter the name of the project which the entry should be added to:",
        )
    }

    /// Function to get the description of an entry from the user
    pub fn get_entry_description(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.print_and_read_input("Please enter the description of the entry:")
    }

    /// Function to get the hours worked from the user
    pub fn get_entry_hours(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let input = self.print_and_read_input("Please enter the hours worked:");
        match input?.parse::<u32>() {
            Ok(hours) => Ok(hours),
            Err(_) => {
                println!("Invalid input, please enter a number.");
                self.get_entry_hours()
            }
        }
    }

    /// Function to get determine of starting time should be set
    pub fn get_day_starting_time(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let input =
            self.print_and_read_input("Would you like to set the starting time to now? (y/n)");
        match input?.as_str() {
            "y" => Ok(true),
            "n" => Ok(false),
            _ => {
                println!("Invalid input, please enter 'y' or 'n'.");
                self.get_day_starting_time()
            }
        }
    }

    /// Function to get determine of ending time should be set
    pub fn get_day_ending_time(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let input =
            self.print_and_read_input("Would you like to set the ending time to now? (y/n)");
        match input?.as_str() {
            "y" => Ok(true),
            "n" => Ok(false),
            _ => {
                println!("Invalid input, please enter 'y' or 'n'.");
                self.get_day_ending_time()
            }
        }
    }

    /// Function to get the description of a day from the user
    pub fn get_day_description(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.print_and_read_input("Please enter the description of the day:")
    }

    /// Support function for the user. Display the list of current stored projects. Can be hard to remember when adding.
    pub fn get_list_projects(
        &self,
        stored_projects: &Vec<Project>,
    ) -> Result<(bool, Option<String>), Box<dyn std::error::Error>> {
        let input = self.print_and_read_input("Would you like to list all projects? (y/n)");
        match input?.as_str() {
            "y" => Ok((true, None)),
            "n" => {
                println!("Current stored projects:");
                for p in stored_projects {
                    println!("- {}", p.name());
                }
                let name =
                    self.print_and_read_input("Please enter the name of the project: to display");
                Ok((false, Some(name?)))
            }
            _ => {
                println!("Invalid input, please enter 'y' or 'n'.");
                self.get_list_projects(stored_projects)
            }
        }
    }

    /// Support function for the user. Display the list of current stored weeks. Can be hard to remember when adding.
    pub fn get_list_weeks(
        &self,
        stored_weeks: &Vec<Week>,
    ) -> Result<(bool, Option<u32>), Box<dyn std::error::Error>> {
        let input = self.print_and_read_input("Would you like to list all weeks? (y/n)");
        match input?.as_str() {
            "y" => Ok((true, None)),
            "n" => {
                println!("Current stored weeks:");
                for w in stored_weeks {
                    println!("- Week {}", w.number());
                }
                let number = self.print_and_read_input("Please enter week number to list:");
                match number?.parse::<u32>() {
                    Ok(number) => Ok((false, Some(number))),
                    Err(_) => {
                        println!("Invalid input, please enter a number.");
                        self.get_list_weeks(stored_weeks)
                    }
                }
            }
            _ => {
                println!("Invalid input, please enter 'y' or 'n'.");
                self.get_list_weeks(stored_weeks)
            }
        }
    }

    // Helper functions

    /// Helper function to print a message and read input from the user. Internal use only.
    fn print_and_read_input(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("{}", message);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}
