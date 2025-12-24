import fs from "fs";

const START = 50;

const MIN = 0;
const MAX = 99;

const SIZE = MAX - MIN + 1;

function solve(input: string[]) {
  let part1 = 0;
  let part2 = 0;
  let current = START;

  for (const line of input) {
    const dir = line[0];
    const times = parseInt(line.slice(1), 10);

    for (let i = 0; i < times; i++) {
      if (dir === "R") {
        current = (current + 1) % SIZE;
      } else {
        current = (current - 1) % SIZE;
      }

      if (current === 0) part2 += 1;
    }

    if (current === 0) part1 += 1;
  }

  console.log({ part1, part2 });
}

solve(fs.readFileSync("./input.txt", "utf-8").trim().split("\n"));
