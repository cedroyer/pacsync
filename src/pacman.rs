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
use std::{io, str};
use std::process::Command;
use crate::engine::compute_actions::Package;
use crate::compute_actions::Actions;

#[derive(Debug)]
pub enum PacmanError {
    ParseGroupError(String),
    Utf8(str::Utf8Error),
    Io(io::Error),
}

pub fn get_explicit_installed_packages() -> Result<HashSet<Package>, PacmanError> {
    let groups= parse_pacman_groups(Command::new("pacman").arg("-Qeqg").output()?.stdout)?;
    let packages= parse_pacman_packages(Command::new("pacman").arg("-Qeq").output()?.stdout)?;
    Ok(merge_packages(groups, packages))
}

pub fn apply_actions(actions: Actions) -> io::Result<()> {
    Ok(())
}

fn parse_pacman_groups(output: Vec<u8>) -> Result<HashSet<Package>, PacmanError> {
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

fn parse_pacman_packages(output: Vec<u8>) -> Result<HashSet<Package>, str::Utf8Error> {
   let utf8_output = String::from(str::from_utf8(&output)?);
   Ok(HashSet::from_iter(utf8_output.split("\n").filter(|row| row.len() > 0).map(|row| Package::new(row.to_string(), Option::None))))
}

fn parse_group(row: &str) -> Result<Package, PacmanError> {
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
