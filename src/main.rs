/*
 *  main.rs
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

use crate::configuration::reader;
use crate::engine::compute_actions;
use std::path::Path;
use std::io;

pub mod configuration;
pub mod engine;
pub mod pacman;

fn main() {
    let reference = reader::read(Path::new("/etc/pacsync.d/target/")).expect("Cannot read configuration.");
    let current = pacman::get_explicit_installed_packages().expect("Cannot query pacman.");
    let actions = compute_actions::compute_actions(reference, current);
    println!("# actions to be done\n{}", actions);
    println!("apply/print/no abort [y/p/n] ?");
    let answer = get_answer().unwrap();
    if answer == "y\n" {
        pacman::apply_actions(&actions).expect("Cannot apply actions.");
    } else if answer == "p\n" {
        pacman::print_actions(&actions);
    } else {
        println!("Abort")
    }
}

fn get_answer() -> io::Result<String> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}
