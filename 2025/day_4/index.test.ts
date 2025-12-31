import { expect, test } from "vitest";

import day from "./index.ts";

test("sample", () => {
  const result = day(
    [
      "..@@.@@@@.",
      "@@@.@.@.@@",
      "@@@@@.@.@@",
      "@.@@@@..@.",
      "@@.@@@@.@@",
      ".@@@@@@@.@",
      ".@.@.@.@@@",
      "@.@@@.@@@@",
      ".@@@@@@@@.",
      "@.@.@@@.@.",
    ],
    { part1: 0, part2: 0 },
  );

  expect(result).toMatchObject({
    part1: 13,
    part2: 43,
  });
});
