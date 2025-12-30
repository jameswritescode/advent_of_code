import { type Result, solve } from "../helpers.ts";

const PART1_BATTERIES = 2;
const PART2_BATTERIES = 12;

function findLargestBattery(
  batteries: number[],
  size: number,
): { battery: number; position: number } {
  let lastLarge = 0;
  let lastIndex = 0;
  let maxIndex = batteries.length;

  batteries.forEach((battery, index) => {
    if (battery > lastLarge && maxIndex - index >= size) {
      lastLarge = battery;
      lastIndex = index;
    }
  });

  return { battery: lastLarge, position: lastIndex + 1 };
}

function totalOutput(batteries: number[], maxBatteries: number): number {
  let nextIndex = 0;
  let output = 0;

  for (let size = maxBatteries; size > 0; size--) {
    const largest = findLargestBattery(batteries.slice(nextIndex), size);

    nextIndex += largest.position;
    output += largest.battery * 10 ** (size - 1);
  }

  return output;
}

export default function day3(input: string[]): Result {
  let part1 = 0;
  let part2 = 0;

  for (const bank of input) {
    const batteries = bank.split("").map((c) => parseInt(c, 10));

    part1 += totalOutput(batteries, PART1_BATTERIES);
    part2 += totalOutput(batteries, PART2_BATTERIES);
  }

  return { part1, part2 };
}

solve(day3);
