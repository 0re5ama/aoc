paste <(awk '{print $1}' inputs/1 | sort -n) <(awk '{print $2}' inputs/1 | sort -n) | awk ' { sum += ($1 > $2) ? $1 - $2 : $2 - $1 } END { print sum } '

paste <(awk '{ print $1 }' inputs/1) <(awk '{ print $1 }' inputs/1 | while read value; do rg "^$value\$" <(awk '{ print $2}' inputs/1) | wc -l; done) | awk '{ sum += $1 * $2 } END { print sum }'
