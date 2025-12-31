import { expect, test } from "vitest";

import day from "./index.ts";

test("sample", () => {
  const result = day(
    [
      /* input */
    ],
    { part1: 0, part2: 0 },
  );

  expect(result).toMatchObject({
    part1: 0,
    part2: 0,
  });
});
