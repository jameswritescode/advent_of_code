import { type Result, solve } from "../helpers.ts";

type Range = [number, number];

function merge(ranges: Range[]): Range[] {
  const sorted = ranges.sort((a, b) => a[0] - b[0]);
  const merged = [sorted[0]!];
  let last = merged[0]!;

  for (let i = 1; i < sorted.length; i++) {
    const current = sorted[i]!;

    if (current[0] <= last[1]) {
      last[1] = Math.max(last[1], current[1]);
    } else {
      merged.push(current);
      last = current;
    }
  }

  return merged;
}

export default function day(input: string[], result: Result): Result {
  const separator = input.indexOf("");

  const ranges = merge(
    input
      .slice(0, separator)
      .map((range) =>
        range.split("-").map((int) => parseInt(int, 10)),
      ) as Range[],
  );

  const ids = input.slice(separator + 1).map((id) => parseInt(id, 10));

  for (const [begin, end] of ranges) {
    for (const id of ids) {
      if (id >= begin && id <= end) result.part1 += 1;
    }

    result.part2 += end - begin + 1;
  }

  return result;
}

solve(day);
