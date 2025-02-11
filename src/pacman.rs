/*
 *  pacman.rs
 *
 *  Copyright (c) 2024-2024 Cédric ROYER <cedric dot royer at zaclys dot net>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::collections::HashSet;
use std::fmt::Display;
use std::str::Utf8Error;
use std::{io, result, str};
use std::process::{Command, ExitStatus};
use crate::engine::compute_actions::{Package, PackageManager};
use crate::compute_actions::Actions;

#[derive(Debug)]
pub enum PacmanError {
    ParseGroupError(String),
    Utf8(str::Utf8Error),
    Io(io::Error),
    PacmanErrorStatus(String, ExitStatus),
}

pub type Result<T> = result::Result<T, PacmanError>;

pub fn get_explicit_installed_packages() -> Result<HashSet<Package>> {
    let groups= parse_pacman_groups(Command::new("pacman").arg("-Qeqg").output()?.stdout)?;
    let packages= parse_pacman_packages(Command::new("pacman").arg("-Qeq").output()?.stdout)?;
    Ok(merge_packages(groups, packages))
}

pub fn print_actions(actions: &Actions) {
    if !actions.to_add.is_empty() {
        println!("{:?}", build_install_command(actions));
    }
    if !actions.to_delete.is_empty() {
        println!("{:?}", build_remove_command(actions));
    }
}

pub fn apply_actions(actions: &Actions) -> Result<()> {
    if !actions.to_add.is_empty() {
        let mut add = build_install_command(actions);
        let status = add.status()?;
        if !status.success() {
            return Err(PacmanError::PacmanErrorStatus("Pacman install command failed".to_string(), status));
        }
    }
    if !actions.to_delete.is_empty() {
        let mut delete = build_remove_command(actions);
        let status = delete.status()?;
        if !status.success() {
            return Err(PacmanError::PacmanErrorStatus("Pacman remove command failed".to_string(), status));
        }
    }
    Ok(())
}

fn build_install_command(actions: &Actions) -> Command {
    let mut cmd = Command::new("sudo");
    cmd.arg("pacman");
    cmd.arg("-S");
    cmd.args(actions.to_add.iter().filter(|&p_or_g| p_or_g.manager == PackageManager::PACMAN).map(|p_or_g| p_or_g.name.clone()));
    cmd
}

fn build_remove_command(actions: &Actions) -> Command {
    let mut cmd = Command::new("sudo");
    cmd.arg("pacman");
    cmd.arg("-R");
    cmd.args(actions.to_delete.iter().filter(|&p_or_g| p_or_g.manager == PackageManager::PACMAN).map(|p_or_g| p_or_g.name.clone()));
    cmd
}

fn parse_pacman_groups(output: Vec<u8>) -> Result<HashSet<Package>> {
   let utf8_output = String::from(str::from_utf8(&output)?);
   let mut groups  = HashSet::new();
   let mut errors = Vec::new();
   for row in utf8_output.split("\n") {
       if !row.is_empty() {
           match parse_group(row) {
               Ok(group) => {groups.insert(group);()},
               Err(err) => errors.push(err),
           }
       }
   }
   if errors.is_empty() {
       return Ok(groups);
   } else {
       let mut message = "cannot read pacman output because:\n - ".to_owned();
       for error in errors {
           message.push_str(" - ");
           message.push_str(error.to_string().as_str());
           message.push_str("\n");
       }
       return Err(PacmanError::ParseGroupError(message))
   }
}

fn parse_pacman_packages(output: Vec<u8>) -> Result<HashSet<Package>> {
   let utf8_output = String::from(str::from_utf8(&output)?);
   Ok(HashSet::from_iter(utf8_output.split("\n").filter(|row| row.len() > 0).map(|row| Package::new(row.to_string(), Option::None))))
}

fn parse_group(row: &str) -> Result<Package> {
    let values: Vec<&str> = row.split(" ").collect();
    if values.len() != 2 {
        return Err(PacmanError::ParseGroupError(row.to_string()));
    }
    Ok(Package::new(values[1].to_string(), Option::Some(values[0].to_string())))
}

fn merge_packages(groups: HashSet<Package>, packages: HashSet<Package>) -> HashSet<Package> {
    let mut result = HashSet::new();
    for group in groups {
        result.insert(group);
    }
    for package in packages {
        result.insert(package);
    }
    result
}

impl From<Utf8Error> for PacmanError {
    fn from(err: Utf8Error) -> PacmanError {
        PacmanError::Utf8(err)
    }
}

impl From<io::Error> for PacmanError {
    fn from(err: io::Error) -> PacmanError {
        PacmanError::Io(err)
    }
}

impl Display for PacmanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacmanError::ParseGroupError(err) => write!(f, "parsing group error: {err}"),
            PacmanError::Utf8(err) => write!(f, "cannot parse Utf8: {err}"),
            PacmanError::Io(err) => write!(f, "cannot read pacman output: {err}"),
            PacmanError::PacmanErrorStatus(message, status) => write!(f, "{message} with status {status}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use super::*;

    #[test]
    fn parse_groups_nominal() {
        // Given
        let output = fs::read(Path::new("tests/data/pacman_groups_output"));
        let mut expected = HashSet::<Package>::new();
        expected.insert(Package::new("baobab".to_string(), Some("gnome".to_string())));
        expected.insert(Package::new("epiphany".to_string(), Some("gnome".to_string())));
        expected.insert(Package::new("evince".to_string(), Some("gnome".to_string())));
        expected.insert(Package::new("gdm".to_string(), Some("gnome".to_string())));
        
        // When
        let groups = parse_pacman_groups(output.unwrap()).unwrap();

        // Then
        assert_eq!(groups, expected)
    }

    #[test]
    fn parse_packages_nominal() {
        // Given
        let output = fs::read(Path::new("tests/data/pacman_packages_output"));
        let mut expected = HashSet::<Package>::new();
        expected.insert(Package::new("amd-ucode".to_string(), None));
        expected.insert(Package::new("baobab".to_string(), None));
        expected.insert(Package::new("base".to_string(), None));
        expected.insert(Package::new("blender".to_string(), None));

        // When
        let packages = parse_pacman_packages(output.unwrap()).unwrap();

        // Then
        assert_eq!(packages, expected)
    }

    #[test]
    fn merge_packages_nominal() {
        // Given
        let output = fs::read(Path::new("tests/data/pacman_groups_output"));
        let groups = parse_pacman_groups(output.unwrap()).unwrap();
        let output = fs::read(Path::new("tests/data/pacman_packages_output"));
        let packages = parse_pacman_packages(output.unwrap()).unwrap();
        let mut expected = HashSet::<Package>::new();
        expected.insert(Package::new("baobab".to_string(), Some("gnome".to_string())));
        expected.insert(Package::new("epiphany".to_string(), Some("gnome".to_string())));
        expected.insert(Package::new("evince".to_string(), Some("gnome".to_string())));
        expected.insert(Package::new("gdm".to_string(), Some("gnome".to_string())));
        expected.insert(Package::new("amd-ucode".to_string(), None));
        expected.insert(Package::new("base".to_string(), None));
        expected.insert(Package::new("blender".to_string(), None));

        // When
        let merged_packages = merge_packages(groups, packages);

        // Then
        assert_eq!(merged_packages, expected)

    }
}
