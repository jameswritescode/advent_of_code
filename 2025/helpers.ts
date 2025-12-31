export type Result = {
  part1: number;
  part2: number;
};

type Callback = (input: string[], result: Result) => Result;

export async function solve(cb: Callback): Promise<void> {
  const stdin = await process.stdin.toArray();
  const input = Buffer.concat(stdin).toString().trim().split("\n");

  console.log(cb(input, { part1: 0, part2: 0 }));
}
