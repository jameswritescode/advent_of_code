import { type Result, solve } from "../helpers.ts";

function isRepeated(full: string, toRepeat: string, repeat: number) {
  return toRepeat.repeat(repeat) === full;
}

export default function day2(input: string[]): Result {
  const lines = input[0].split(",").map((line) => {
    return line.split("-").map((n) => parseInt(n, 10));
  });

  let part1 = 0;
  let part2 = 0;

  for (const line of lines) {
    const [start, end] = line;

    for (let i = start; i <= end; i++) {
      const str = i.toString();
      const mid = Math.floor(str.length / 2);
      const num = parseInt(str, 10);

      if (num <= 9) continue;

      // TODO: How can we do this in the code below?
      if (isRepeated(str, str.slice(0, mid), 2)) {
        part1 += num;
      }

      if (num <= 999 && str[0].repeat(str.length) === str) {
        part2 += num;

        continue;
      }

      if (!(str.length % 2 === 0) && isRepeated(str, str[0], str.length)) {
        part2 += num;
      }

      outer: for (let j = 1; j <= Math.floor(mid); j++) {
        const toRepeat = str.slice(0, j);

        for (let k = 1; k <= mid; k++) {
          if (isRepeated(str, toRepeat, k)) {
            part2 += num;
            break outer;
          }
        }
      }
    }
  }

  return { part1, part2 };
}

solve(day2);
