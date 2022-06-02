#!/usr/bin/env bash

set -euo pipefail

source pacsync/libpacsync.bash

# working_dir
tmp_dir=$(mktemp -d)
trap 'rm -r "'"$tmp_dir"'"' EXIT

#########################################
echo "- test clean_package_list"

cat <<EOF | clean_package_list > "$tmp_dir/clean_target_result"

# This is a test

pac2
pac1
pac3

# a duplicate
pac1

EOF

cat <<EOF | diff - "$tmp_dir/clean_target_result"
pac1
pac2
pac3
EOF

##########################################
echo "- test packages_to_regex"
cat <<EOF | packages_to_regex > "$tmp_dir/regex_list"
pac1
pac2
pac3
python2-virtualenv
EOF

cat <<EOF | diff "$tmp_dir/regex_list" -
(^|\s)pac1(\s|$)
(^|\s)pac2(\s|$)
(^|\s)pac3(\s|$)
(^|\s)python2-virtualenv(\s|$)
EOF

packages_to_regex <<< '' > /dev/null

##########################################
echo "- test compute_diff_files"

cat <<EOF > "$tmp_dir/clean_target_path"
pac1
pac4
pac5
group1
group_new
pac_new
pac
python2-virtualenv
EOF

cat <<EOF > "$tmp_dir/clean_current_path"
group1 pac1
group1 pac2
group3 pac3
group4 pac4
pa
pac5
pac6
python2-virtualenv
EOF

( 
  export clean_target_path=clean_target_path
  export clean_current_path=clean_current_path
  export to_delete_path=to_delete_path
  export to_add_path=to_add_path
  cd "$tmp_dir" && compute_diff_files 
)

cat <<EOF | diff - "$tmp_dir/to_delete_path"
pac3
pa
pac6
EOF

cat <<EOF | diff - "$tmp_dir/to_add_path"
group_new
pac_new
pac
EOF
