import { type Result, solve } from "../helpers.ts";

const DIRECTIONS: Array<[number, number]> = [
  [-1, -1],
  [-1, 0],
  [-1, 1],
  [0, -1],
  [0, 1],
  [1, -1],
  [1, 0],
  [1, 1],
];

function directions(position: number, size: number): number[] {
  const row = Math.floor(position / size);
  const col = position % size;
  const result = [];

  for (const [x, y] of DIRECTIONS) {
    const nrow = row + x;
    const ncol = col + y;

    const valid = nrow >= 0 && nrow < size && ncol >= 0 && ncol < size;

    if (valid) {
      result.push(nrow * size + ncol);
    }
  }

  return result;
}

function removeRolls(line: string[], size: number): number {
  let removed: number[] = [];

  line.forEach((char, index) => {
    if (char !== "@") return;

    let surrounding = 0;

    const dirs = directions(index, size);

    for (const direction of dirs) {
      if (line[direction] === "@") {
        surrounding += 1;
      }
    }

    if (surrounding < 4) {
      removed.push(index);
    }
  });

  for (const position of removed) {
    line[position] = ".";
  }

  return removed.length;
}

export default function day(input: string[], result: Result): Result {
  const size = input[0]!.length;
  let line = input.join("").split("");

  let loops = 1;

  while (true) {
    const removed = removeRolls(line, size);

    if (removed === 0) break;

    if (loops === 1) result.part1 += removed;
    result.part2 += removed;

    loops += 1;
  }

  return result;
}

solve(day);
