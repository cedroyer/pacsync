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

use std::collections::HashSet;

pub type PackageOrGroup = String;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Package {
    name: String,
    group: String,
}

#[derive(PartialEq, Debug)]
pub struct Actions {
    to_add: HashSet<PackageOrGroup>,
    to_delete: HashSet<PackageOrGroup>,
}

pub fn compute_actions(reference: HashSet<PackageOrGroup>, current: HashSet<Package>) -> Actions {
    let current_groups: std::collections::HashSet<&String> =
        HashSet::from_iter(current.iter().map(|p| &p.group));
    let current_packages: std::collections::HashSet<&String> =
        HashSet::from_iter(current.iter().map(|p| &p.name));
    let to_add = HashSet::from_iter(
        reference
            .iter()
            .filter(|&p_or_g| {
                !(current_packages.contains(p_or_g) || current_groups.contains(p_or_g))
            })
            .map(|p_or_g| p_or_g.clone()),
    );
    let to_delete = HashSet::from_iter(
        current
            .iter()
            .filter(|&p| !(reference.contains(&p.name) || reference.contains(&p.group)))
            .map(|p| p.name.clone()),
    );
    Actions {
        to_add: to_add,
        to_delete: to_delete,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nominal_case() {
        let mut reference = HashSet::new();
        reference.insert("to_keep1".to_string());
        reference.insert("to_keep2".to_string());
        reference.insert("to_keep3".to_string());
        reference.insert("to_add1".to_string());

        let mut current = HashSet::new();
        current.insert(Package {
            name: "to_keep1".to_string(),
            group: "a group".to_string(),
        });
        current.insert(Package {
            name: "a package".to_string(),
            group: "to_keep2".to_string(),
        });
        current.insert(Package {
            name: "another package".to_string(),
            group: "to_keep2".to_string(),
        });
        current.insert(Package {
            name: "to_keep3".to_string(),
            group: "a group".to_string(),
        });
        current.insert(Package {
            name: "to_rm1".to_string(),
            group: "a group".to_string(),
        });

        let actions = compute_actions(reference, current);

        let mut expected_to_add = HashSet::new();
        expected_to_add.insert("to_add1".to_string());

        let mut expected_to_rm = HashSet::new();
        expected_to_rm.insert("to_rm1".to_string());

        assert_eq!(
            actions,
            Actions {
                to_add: expected_to_add,
                to_delete: expected_to_rm
            }
        );
    }
}
