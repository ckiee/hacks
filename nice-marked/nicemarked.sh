#!/bin/bash
fn=$(echo $1 | sed -e "s/\.\w*$//g")
replace=$(marked $1)
# Escape it for use as a Sed replacement string. https://stackoverflow.com/a/29613573 I do not claim copyright on this guy's work.
IFS= read -d '' -r < <(sed -e ':a' -e '$!{N;ba' -e '}' -e 's/[&/\]/\\&/g; s/\n/\\&/g' <<<"$replace")
replaceEscaped=${REPLY%$'\n'}
title="$(cat $1 | grep -m 1 "# " | cut -d " " -f 2-)"
cat template.html | sed -e "s/%title/$title/g" | sed -e "s/%content/$replaceEscaped/g" > $fn.html