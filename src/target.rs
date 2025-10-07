/*
 * File: target.rs
 * Description: The definition of the Target struct. Contains the target and status for the stored time containers
 * Author: dherslof
 * Created: 02-04-2025
 * License: MIT
 */

use crate::day::Day;
use crate::week::Week;

const K_100_PERCENT: u32 = 100;
const K_TARGET_HOURS_NOT_SET: f32 = 0.0;
const K_DEFAULT_WEEK_TARGET_HOURS: f32 = 40.0;
const K_DEFAULT_MONTH_TARGET_HOURS: f32 = 160.0;

/// TargetStatus enum to represent the progress of the target
#[derive(Debug, Clone)]
enum TargetStatus {
    /// Target is set, not sure if this is needed
    Reached,
    /// Target is not reached
    NotReached,
    /// Target is overreached
    OverReached,
}

/// TargetStatus enum to represent how the target was set
#[derive(Debug, Clone)]
enum TargetSetMethod {
    /// Target is set from config
    SetFromConfig,
    /// Target is set from default
    SetFromDefault,
}

/// Weekly-Target struct to store the target and status of the week
#[derive(Debug, Clone)]
pub struct WeeklyTargetStatus {
    /// Target for the week
    target_hours: f32,
    /// Status of the week
    status_hours: f32,
    /// hours_difference
    hours_difference: f32,
    /// Percentage of the target
    percentage: u32,
    /// Remaining hours
    remaining_hours: f32,
    /// Target status
    target_status: TargetStatus,
    /// Target set method
    target_set_method: TargetSetMethod,
}

///Monthly-Target struct to store the target and status of the month
#[derive(Debug, Clone)]
pub struct MonthlyTargetStatus {
    /// Target for the month
    target_hours: f32,
    /// Status of the month
    status_hours: f32,
    /// hours_difference
    hours_difference: f32,
    /// Percentage of the target
    percentage: u32,
    /// Remaining hours
    remaining_hours: f32,
    /// Target status
    target_status: TargetStatus,
    /// Target set method
    target_set_method: TargetSetMethod,
}

//Todo: project target

impl WeeklyTargetStatus {
    /// Create a new WeeklyTarget
    pub fn new(week: &Week, target_hours_conf: &f32) -> Self {
        let mut status_hours = 0.0;
        // Calculate the total hours for the week
        for day in week.entries() {
            status_hours += day.hours();
        }

        let target_set_method = if target_hours_conf != &K_TARGET_HOURS_NOT_SET {
            TargetSetMethod::SetFromConfig
        } else {
            TargetSetMethod::SetFromDefault
        };

        let mut target_hours = K_DEFAULT_WEEK_TARGET_HOURS;
        if target_hours_conf != &K_TARGET_HOURS_NOT_SET {
            target_hours = *target_hours_conf;
        }

        let hours_difference = (target_hours - status_hours).abs();
        let percentage = ((status_hours / target_hours) * K_100_PERCENT as f32) as u32;
        let remaining_hours = target_hours - status_hours;

        let target_status = if percentage == K_100_PERCENT {
            TargetStatus::Reached
        } else if percentage > K_100_PERCENT {
            TargetStatus::OverReached
        } else if percentage < K_100_PERCENT && percentage > 0 {
            TargetStatus::NotReached
        } else {
            TargetStatus::NotReached
        };

        Self {
            target_hours: target_hours,
            status_hours,
            hours_difference,
            percentage,
            remaining_hours,
            target_status,
            target_set_method,
        }
    }

    /// Getter for `target_hours`
    pub fn target_hours(&self) -> &f32 {
        &self.target_hours
    }
    /// Getter for `status_hours`
    pub fn status_hours(&self) -> &f32 {
        &self.status_hours
    }
    /// Getter for `hours_difference`
    pub fn hours_difference(&self) -> &f32 {
        &self.hours_difference
    }
    /// Getter for `percentage`
    pub fn percentage(&self) -> &u32 {
        &self.percentage
    }
    /// Getter for `remaining_hours`
    pub fn remaining_hours(&self) -> &f32 {
        &self.remaining_hours
    }

    /// Getter for `target_status` as string
    pub fn target_status_string(&self) -> String {
        match self.target_status {
            TargetStatus::Reached => "Reached".to_string(),
            TargetStatus::NotReached => "NotReached".to_string(),
            TargetStatus::OverReached => "OverReached".to_string(),
        }
    }

    /// Getter for `target_set_method` as string
    pub fn target_set_method_string(&self) -> String {
        match self.target_set_method {
            TargetSetMethod::SetFromConfig => "Set from config".to_string(),
            TargetSetMethod::SetFromDefault => "Set from default".to_string(),
        }
    }
}

//TODO: Refactor this to use the same method as WeeklyTargetStatus (avoid copy-paste code)

impl MonthlyTargetStatus {
    /// Create a new MonthlyTarget
    pub fn new(days_in_month: &Vec<Day>, target_hours_conf: &f32) -> Self {
        let mut status_hours = 0.0;
        // Calculate the total hours for the week
        for day in days_in_month {
            status_hours += day.hours();
        }

        let target_set_method = if target_hours_conf != &K_TARGET_HOURS_NOT_SET {
            TargetSetMethod::SetFromConfig
        } else {
            TargetSetMethod::SetFromDefault
        };

        let mut target_hours = K_DEFAULT_MONTH_TARGET_HOURS;
        if target_hours_conf != &K_TARGET_HOURS_NOT_SET {
            target_hours = *target_hours_conf;
        }

        let hours_difference = (target_hours - status_hours).abs();
        let percentage = ((status_hours / target_hours) * K_100_PERCENT as f32) as u32;
        let remaining_hours = target_hours - status_hours;

        let target_status = if percentage == K_100_PERCENT {
            TargetStatus::Reached
        } else if percentage > K_100_PERCENT {
            TargetStatus::OverReached
        } else if percentage < K_100_PERCENT && percentage > 0 {
            TargetStatus::NotReached
        } else {
            TargetStatus::NotReached
        };

        Self {
            target_hours: target_hours,
            status_hours,
            hours_difference,
            percentage,
            remaining_hours,
            target_status,
            target_set_method,
        }
    }

    /// Getter for `target_hours`
    pub fn target_hours(&self) -> &f32 {
        &self.target_hours
    }
    /// Getter for `status_hours`
    pub fn status_hours(&self) -> &f32 {
        &self.status_hours
    }
    /// Getter for `hours_difference`
    pub fn hours_difference(&self) -> &f32 {
        &self.hours_difference
    }
    /// Getter for `percentage`
    pub fn percentage(&self) -> &u32 {
        &self.percentage
    }
    /// Getter for `remaining_hours`
    pub fn remaining_hours(&self) -> &f32 {
        &self.remaining_hours
    }

    /// Getter for `target_status` as string
    pub fn target_status_string(&self) -> String {
        match self.target_status {
            TargetStatus::Reached => "Reached".to_string(),
            TargetStatus::NotReached => "NotReached".to_string(),
            TargetStatus::OverReached => "OverReached".to_string(),
        }
    }

    /// Getter for `target_set_method` as string
    pub fn target_set_method_string(&self) -> String {
        match self.target_set_method {
            TargetSetMethod::SetFromConfig => "Set from config".to_string(),
            TargetSetMethod::SetFromDefault => "Set from default".to_string(),
        }
    }
}
