import { expect, test } from "vitest";

import day2 from "./index.ts";

test("sample", () => {
  const result = day2([
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
    1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
    824824821-824824827,2121212118-2121212124",
  ]);

  expect(result).toMatchObject({
    part1: 1227775554,
    part2: 4174379265,
  });
});

test("odd ranges", () => {
  const result = day2(["110-111,50000-55555"]);

  expect(result).toMatchObject({
    part2: 55666,
  });
});
