curl "https://adventofcode.com/$1/day/$2/input"\
    --compressed\
    -H 'Cookie: session=53616c7465645f5ff7db6cf7dcfaeaa9a3606909544b8b1b9e8b648a598479fa6742d5a34462905a2ec1d3e0d92fd15b053968b9acd8f23765664d67f8a7c9f0' >> "$HOME/dev/aoc/$1/inputs/$2"
