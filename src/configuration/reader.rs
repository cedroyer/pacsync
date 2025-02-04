/*
 *  configuration/reader.rs
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

use crate::engine::compute_actions::{PackageOrGroup,PackageManager};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub enum ConfigReaderError {
    Io(io::Error),
    ParseError(String),
}

pub fn read(dir: &Path) -> Result<HashSet<PackageOrGroup>, ConfigReaderError> {
    let mut reference = HashSet::<PackageOrGroup>::new();
    visit_dirs(dir, &mut reference)?;
    Ok(reference)
}

fn visit_dirs(dir: &Path, reference: &mut HashSet<PackageOrGroup>) -> Result<(), ConfigReaderError> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, reference)?;
            } else {
                insert_packages(&path, reference)?;
            }
        }
    }
    Ok(())
}

fn insert_packages(filename: &Path, reference: &mut HashSet<PackageOrGroup>) -> Result<(), ConfigReaderError> {
    for package in read_packages(filename)? {
        let package = package?;

        // ignore comments
        if package.starts_with("#") {
            continue;
        }

        // ignore empty line
        if package.find(|c| !char::is_whitespace(c)).is_none() {
            continue;
        }
        let split: Vec<&str> = package.split("/").collect();

        if split.len() == 1 {
            reference.insert(PackageOrGroup::new(split[0].to_string(), PackageManager::PACMAN));
        } else if split.len() == 2 {
            reference.insert(PackageOrGroup::new(split[1].to_string(), parse_package_manager(&split[0])?));
        } else {
            return Err(ConfigReaderError::ParseError(format!("Too many / in line for {}", package)));
        }
    }
    Ok(())
}

fn read_packages(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_package_manager(raw: &str) -> Result<PackageManager, ConfigReaderError> {
    if raw == "local" {
        return Ok(PackageManager::LOCAL);
    } else if raw == "pacman" {
        return Ok(PackageManager::PACMAN);
    } else {
        return Err(ConfigReaderError::ParseError(format!("Unkown package manager: {}", raw)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nominal_case() {
        let reference = read(Path::new("tests_config_dir")).unwrap();

        let mut expected = HashSet::<PackageOrGroup>::new();
        expected.insert(PackageOrGroup::new("package_1".to_string(), PackageManager::PACMAN));
        expected.insert(PackageOrGroup::new("package_1_1".to_string(), PackageManager::PACMAN));
        expected.insert(PackageOrGroup::new("package_1_1_1".to_string(), PackageManager::PACMAN));
        expected.insert(PackageOrGroup::new("package_1_1_2".to_string(), PackageManager::PACMAN));
        expected.insert(PackageOrGroup::new("package_1_1_3".to_string(), PackageManager::PACMAN));
        expected.insert(PackageOrGroup::new("package_1_1_4".to_string(), PackageManager::PACMAN));
        expected.insert(PackageOrGroup::new("package_1_2".to_string(), PackageManager::LOCAL));
        expected.insert(PackageOrGroup::new("package_1_3".to_string(), PackageManager::PACMAN));
        expected.insert(PackageOrGroup::new("package_1_4".to_string(), PackageManager::LOCAL));
        expected.insert(PackageOrGroup::new("package_2".to_string(), PackageManager::PACMAN));
        expected.insert(PackageOrGroup::new("package_2_1".to_string(), PackageManager::PACMAN));
        expected.insert(PackageOrGroup::new("package_2_2".to_string(), PackageManager::PACMAN));
        expected.insert(PackageOrGroup::new("package_2_3".to_string(), PackageManager::PACMAN));
        expected.insert(PackageOrGroup::new("package_2_4".to_string(), PackageManager::PACMAN));
        expected.insert(PackageOrGroup::new("package_3".to_string(), PackageManager::PACMAN));
        expected.insert(PackageOrGroup::new("package_4".to_string(), PackageManager::PACMAN));
        assert_eq!(reference, expected);
    }
}

impl From<io::Error> for ConfigReaderError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
