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

use crate::engine::compute_actions::PackageOrGroup;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read(dir: &Path) -> io::Result<HashSet<PackageOrGroup>> {
    let mut reference = HashSet::<PackageOrGroup>::new();
    visit_dirs(dir, &mut reference)?;
    Ok(reference)
}

fn visit_dirs(dir: &Path, reference: &mut HashSet<PackageOrGroup>) -> io::Result<()> {
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

fn insert_packages(filename: &Path, reference: &mut HashSet<PackageOrGroup>) -> io::Result<()> {
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

        reference.insert(package);
    }
    Ok(())
}

fn read_packages(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nominal_case() {
        let reference = read(Path::new("tests_config_dir")).unwrap();

        let mut expected = HashSet::<PackageOrGroup>::new();
        expected.insert("package_1".to_string());
        expected.insert("package_1_1".to_string());
        expected.insert("package_1_1_1".to_string());
        expected.insert("package_1_1_2".to_string());
        expected.insert("package_1_1_3".to_string());
        expected.insert("package_1_1_4".to_string());
        expected.insert("package_1_2".to_string());
        expected.insert("package_1_3".to_string());
        expected.insert("package_1_4".to_string());
        expected.insert("package_2".to_string());
        expected.insert("package_2_1".to_string());
        expected.insert("package_2_2".to_string());
        expected.insert("package_2_3".to_string());
        expected.insert("package_2_4".to_string());
        expected.insert("package_3".to_string());
        expected.insert("package_4".to_string());
        assert_eq!(reference, expected);
    }
}
