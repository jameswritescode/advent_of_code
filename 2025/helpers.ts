export type Result = {
  part1: number;
  part2: number;
};

export async function solve(cb: (input: string[]) => Result): Promise<void> {
  const stdin = await process.stdin.toArray();
  const input = Buffer.concat(stdin).toString().trim().split("\n");

  console.log(cb(input));
}
