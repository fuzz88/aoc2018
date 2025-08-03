#!/bin/bash
# Usage: ./init.sh dayXX "Day Title"

set -e

if [ "$#" -lt 2 ]; then
  echo "Usage: $0 <day_id> <day_title>"
  exit 1
fi

DAY_ID="$1"
DAY_TITLE="$2"
DAY_NUM=$((10#${DAY_ID:3}))

SRC_FILE="src/days/${DAY_ID}.rs"

if [ -f "$SRC_FILE" ]; then
  echo "$SRC_FILE already exists!"
  exit 1
fi

cat > "$SRC_FILE" <<EOF
//! ## --- Day ${DAY_NUM}: ${DAY_TITLE} ---
//!

pub fn parse(input: &str) -> Vec<i32> {
    vec![]
}

pub fn part1(input: &[i32]) -> i32 {
    0
}

pub fn part2(input: &[i32]) -> i32 {
    0
}

#[cfg(test)]
mod test {
    #[test]
    fn sample1() {
        assert!(true);
    }
}
EOF

echo "Added $SRC_FILE."

# Add new day to days! macro in src/lib.rs if not present
grep -q "${DAY_ID}" src/lib.rs || \
  sed -i "/days!(/s/)/, ${DAY_ID})/" src/lib.rs

echo "Updated src/lib.rs."

# Insert ${DAY_ID} into run! macro in src/main.rs if not present
grep -q "${DAY_ID}" src/main.rs || \
  sed -i "s/run!(\(.*\));/run!(\1, ${DAY_ID});/" src/main.rs

echo "Updated src/main.rs."
