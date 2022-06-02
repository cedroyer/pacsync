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

# Needed environment variables
tmp_dir=""
clean_target_path=""
clean_current_path=""
to_delete_path=""
to_delete_deps_path=""
to_add_path=""

function retrieve_target() {
  local target_dir="$1"
  find "$target_dir" -type f,l | while IFS= read -r target_file; do
    cat "$target_file"
  done
}

function retrive_all_packages() {
  pacman -Ssq
}

function packages_to_regex() {
  # match either package or group
  # in order to match complete word only
  sed 's/.*/(^|\\s)\0(\\s|$)/'
}

function retrive_current() {
  group_current="$tmp_dir/group_current"
  pacman -Qeqg | tee "$group_current"
  pacman -Qeq | grep -E -vf <(cut -d' ' -f 2 "$group_current" | packages_to_regex)
}

function clean_package_list() {
  # remove comment like '# a comment'
  # remove empty lines
  # remove duplicates and sort 
  sed -e 's/#.*$//'\
      -e '/^\s*$/d' |\
  sort -u
}

function compute_diff_files() {
  if ! grep -E -vf <(packages_to_regex < "$clean_target_path") "$clean_current_path" |\
    sed 's/^\([^ ]* \)\{0,1\}\([^ ]*\)/\2/'> "$to_delete_path"
  then
    rm -f "$to_delete_path"
  fi
  if ! grep -E -vf <(tr ' ' '\n' < "$clean_current_path" | packages_to_regex) "$clean_target_path" > "$to_add_path"
  then
    rm -f "$to_add_path"
  fi
}

function install_packages() {
  # In that case we want word splitting
  # shellcheck disable=SC2046
  sudo pacman -S $(cat "$1")
}

function remove_packages() {
  # In that case we want word splitting
  # shellcheck disable=SC2046
  sudo pacman -R $(cat "$1")
}
function remove_dependencies_packages() {
  while pacman -Qdtq > "$to_delete_deps_path"
  do
    echo "Remove unused dependencies (can be run many times)"
    # In that case we want word splitting
    # shellcheck disable=SC2046
    sudo pacman -R $(cat "$to_delete_deps_path")
  done
}
