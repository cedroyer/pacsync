/*
 *  engine/compute_actions.rs
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

use std::{collections::HashSet, fmt::{Debug, Display}, hash::Hash};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum PackageManager {
    PACMAN,
    LOCAL,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct PackageOrGroup {
    pub name: String,
    pub manager: PackageManager,
}

impl PackageOrGroup {
    pub fn new(name: String, manager: PackageManager) -> Self {
        PackageOrGroup{name, manager}
    }
}

#[derive(PartialEq, Debug)]
pub struct Actions {
    pub to_add: HashSet<PackageOrGroup>,
    pub to_delete: HashSet<PackageOrGroup>,
}

#[derive(Eq, Debug)]
pub struct Package {
    name: String,
    group: Option<String>,
}

impl Hash for Package {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}


impl Package {
    pub fn new(name: String, group: Option<String>) -> Package {
        Package{name, group}
    }
}

pub fn compute_actions(reference: HashSet<PackageOrGroup>, current: HashSet<Package>) -> Actions {
    let current_groups: std::collections::HashSet<&String> =
        HashSet::from_iter(current.iter().filter_map(|p| p.group.as_ref()));
    let current_packages: std::collections::HashSet<&String> =
        HashSet::from_iter(current.iter().map(|p| &p.name));
    let reference_packages: std::collections::HashSet<&String> =
        HashSet::from_iter(reference.iter().map(|p| &p.name));
    let to_add = HashSet::from_iter(
        reference
        .iter()
        .filter(|&p_or_g| {
            !(current_packages.contains(&p_or_g.name) || current_groups.contains(&p_or_g.name))
        })
        .map(|p_or_g| p_or_g.clone()),
    );
    let to_delete = HashSet::from_iter(
        current
        .iter()
        .filter(|&p| !(reference_packages.contains(&p.name) || p.group.as_ref().is_some_and(|g| reference_packages.contains(g))))
        .map(|p| PackageOrGroup::new(p.name.clone(), PackageManager::PACMAN)),
    );
    Actions {to_add, to_delete}
}

impl Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "Nothing to do")?;
            return Ok(());
        }
        if !self.to_add.is_empty() {
            write!(f, "To add:\n")?;
            for package_or_group in self.to_add.iter() {
                write!(f, "\t- {} ({})\n", package_or_group.name, package_or_group.manager)?;
            }
        }
        if !self.to_delete.is_empty() {
            write!(f, "To delete:\n")?;
            for package_or_group in self.to_delete.iter() {
                write!(f, "\t- {} ({})\n", package_or_group.name, package_or_group.manager)?;
            }
        }
        Ok(())
    }
}

impl Actions {
    pub fn is_empty(&self) -> bool {
        self.to_add.is_empty() && self.to_delete.is_empty()
    }
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::PACMAN => write!(f, "pacman"),
            PackageManager::LOCAL => write!(f, "local")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hasher};

    use super::*;

    #[test]
    fn nominal_case() {
        // Given 
        let mut reference = HashSet::new();
        reference.insert(PackageOrGroup::new("to_keep1".to_string(), PackageManager::PACMAN));
        reference.insert(PackageOrGroup::new("to_keep2".to_string(), PackageManager::PACMAN));
        reference.insert(PackageOrGroup::new("to_keep3".to_string(), PackageManager::LOCAL));
        reference.insert(PackageOrGroup::new("to_add1".to_string(), PackageManager::PACMAN));
        reference.insert(PackageOrGroup::new("to_add2".to_string(), PackageManager::LOCAL));

        let mut current = HashSet::new();
        current.insert(Package::new("to_keep1".to_string(), Some("a group".to_string())));
        current.insert(Package::new("a package".to_string(), Some("to_keep2".to_string())));
        current.insert(Package::new("another package".to_string(),Some("to_keep2".to_string())));
        current.insert(Package::new("to_keep3".to_string(), None));
        current.insert(Package::new("to_rm1".to_string(), Some("a group".to_string())));
        current.insert(Package::new("to_rm2".to_string(), None));

        let mut expected_to_add = HashSet::new();
        expected_to_add.insert(PackageOrGroup::new("to_add1".to_string(), PackageManager::PACMAN));
        expected_to_add.insert(PackageOrGroup::new("to_add2".to_string(), PackageManager::LOCAL));

        let mut expected_to_rm = HashSet::new();
        expected_to_rm.insert(PackageOrGroup::new("to_rm1".to_string(), PackageManager::PACMAN));
        expected_to_rm.insert(PackageOrGroup::new("to_rm2".to_string(), PackageManager::PACMAN));

        // When
        let actions = compute_actions(reference, current);

        // Then
        assert_eq!(
            actions,
            Actions {
                to_add: expected_to_add,
                to_delete: expected_to_rm
            }
        );
    }

    #[test]
    fn package_hash() {
        // Given
        let package_with_group = Package::new("baobab".to_string(), Some("gnome".to_string()));
        let package_without_group =Package::new("baobab".to_string(), None);
        let mut hasher_with_group = DefaultHasher::new();
        let mut hasher_without_group = DefaultHasher::new();

        // When
        package_with_group.hash(&mut hasher_with_group);
        package_without_group.hash(&mut hasher_without_group);


        // Then
        assert_eq!(hasher_with_group.finish(), hasher_without_group.finish())
    }
}
