#    pacsync, for pacman synchronize with a given conf
#    Copyright (C) 2022  Cédric ROYER
#
#    This program is free software; you can redistribute it and/or modify
#    it under the terms of the GNU General Public License as published by
#    the Free Software Foundation; either version 2 of the License, or
#    (at your option) any later version.
#
#    This program is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.
#
#    You should have received a copy of the GNU General Public License along
#    with this program; if not, write to the Free Software Foundation, Inc.,
#    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

# working_dir
tmp_dir=$(mktemp -d /tmp/pacsync.XXXXXXXXXX)
trap 'rm -r "'"$tmp_dir"'"' EXIT

# input files
target_dir='/etc/pacsync.d'

# clean files
clean_dir="$tmp_dir/clean"
mkdir "$clean_dir"
clean_target_path="$clean_dir/target"
clean_current_path="$clean_dir/current"

# actions files
actions_dir="$tmp_dir/actions_to_performs"
mkdir "$actions_dir"
to_delete_path="$actions_dir/to_delete"
to_delete_deps_path="$actions_dir/to_delete_deps"
to_add_path="$actions_dir/to_add"

# pacsync workflow
# 1. Retrieve package lists
# 2. Clean and sort package lists
# 3. Compute what to remove and to install
# 4. Proceed

retrieve_target "$target_dir" | clean_package_list > "$clean_target_path"
retrive_current | sort > "$clean_current_path"
compute_diff_files

if [ -r "$to_add_path" ]
then
  echo "Intall new targeted packages"
  install_packages "$to_add_path"
else
  echo "Nothing to add"
fi
if [ -r "$to_delete_path" ]
then
  echo "Removing untargeted packages"
  remove_packages "$to_delete_path"
else
  echo "Nothing to delete"
fi
remove_dependencies_packages
