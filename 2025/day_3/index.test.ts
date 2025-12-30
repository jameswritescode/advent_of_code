import { expect, test } from "vitest";

import day3 from "./index.ts";

test("sample", () => {
  const result = day3([
    "987654321111111",
    "811111111111119",
    "234234234234278",
    "818181911112111",
  ]);

  expect(result).toMatchObject({
    part1: 357,
    part2: 3121910778619,
  });
});
