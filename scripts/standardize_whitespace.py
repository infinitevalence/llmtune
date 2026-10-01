#!/usr/bin/env python3
"""Replace space-based indentation with tabs in Rust source files.

Auto-detects the base indent size from the file and converts all
indentation to tab characters.
"""

import re
import sys
from collections import Counter
from functools import reduce
from math import gcd


def count_indent(line: str) -> tuple[int, int]:
  """Return (tab_count, space_count) for a line's leading whitespace."""
  m = re.match(r"^\s*", line)
  if not m:
    return 0, 0
  raw = m.group()
  tabs = raw.count("\t")
  spaces = raw.count(" ")
  return tabs, spaces


def detect_base_indent(content: str) -> int:
  """Scan space-based indentation to find the common divisor.

  Only considers space-only indent lines (no tabs mixed) that are
  multiples of 4. Returns the GCD of those lengths.
  Falls back to multiples-of-2 if none are multiples-of-4.
  """
  # Collect space-only indent lengths
  space_lengths: list[int] = []
  for line in content.split("\n"):
    _, spaces = count_indent(line)
    if spaces > 0 and spaces % 4 == 0:
      space_lengths.append(spaces)

  if not space_lengths:
    # Fallback: use multiples of 2
    for line in content.split("\n"):
      _, spaces = count_indent(line)
      if spaces > 0 and spaces % 2 == 0:
        space_lengths.append(spaces)

  if not space_lengths:
    return 4  # default to 4-space blocks

  space_lengths = sorted(set(space_lengths))
  return reduce(gcd, space_lengths)


def standardize_tabs(content: str, base: int | None = None) -> str:
  """Convert all space indentation to tab characters.

  If *base* is None, auto-detect from the file.
  Existing tabs are kept as-is.
  """
  if base is None:
    base = detect_base_indent(content)

  lines = content.split("\n")
  converted = []
  for line in lines:
    tabs, spaces = count_indent(line)
    if spaces == 0:
      converted.append(line)
      continue
    new_tabs = tabs + spaces // base
    remainder = spaces % base
    new_indent = "\t" * new_tabs + " " * remainder
    raw = re.match(r"^\s*", line).group()
    line = new_indent + line[len(raw):]
    converted.append(line)
  return "\n".join(converted)


if __name__ == "__main__":
  path = sys.argv[1]
  with open(path, "r") as f:
    content = f.read()
  detected = detect_base_indent(content)
  result = standardize_tabs(content)
  with open(path, "w") as f:
    f.write(result)
  print(f"Standardized: {path} (detected {detected}-space base -> tabs)")
