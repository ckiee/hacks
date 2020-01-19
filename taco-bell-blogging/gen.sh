#!/bin/bash
#    taco-bell-blogging
#    Copyright (C) 2020 Ron B
#
#    This program is free software: you can redistribute it and/or modify
#    it under the terms of the GNU General Public License as published by
#    the Free Software Foundation, either version 3 of the License, or
#    (at your option) any later version.
#
#    This program is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.
#
#    You should have received a copy of the GNU General Public License
#    along with this program.  If not, see <http://www.gnu.org/licenses/>.
mkdir -p {out,posts}
cp style.css out
ul=" "
for post in posts/*.md; do
    fn=$(echo $post | sed -e "s/\.\w*$//g" | sed -e "s/^posts\///g")
    replace=$(marked posts/*.md)
    # Escape it for use as a Sed replacement string. https://stackoverflow.com/a/29613573 I do not claim copyright on this guy's work.
    IFS= read -d '' -r < <(sed -e ':a' -e '$!{N;ba' -e '}' -e 's/[&/\]/\\&/g; s/\n/\\&/g' <<<"$replace")
    replaceEscaped=${REPLY%$'\n'}
    title="$(cat $post | grep "# " | cut -d " " -f 2-)"
    ul="<li><a href=\"/$fn.html\">$title</a></li>$ul"
    cat template.html | sed -e "s/%title/$title/g" | sed -e "s/%content/$replaceEscaped/g" > out/$fn.html
done
IFS= read -d '' -r < <(sed -e ':a' -e '$!{N;ba' -e '}' -e 's/[&/\]/\\&/g; s/\n/\\&/g' <<<"$ul")
ulEscaped=${REPLY%$'\n'}
cat index_template.html | sed -e "s/%content/$ulEscaped/g" > out/index.html