import { expect, test } from "vitest";

import day from "./index.ts";

test("sample", () => {
  const result = day(
    ["3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32"],
    { part1: 0, part2: 0 },
  );

  expect(result).toMatchObject({
    part1: 3,
    part2: 14,
  });
});
