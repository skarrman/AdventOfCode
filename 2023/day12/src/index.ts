import { log } from "console";
import * as fs from "fs";
import path from "path";

const readInput = (
  filename = "../input.txt"
): { row: string; springs: number[] }[] =>
  fs
    .readFileSync(path.resolve(__dirname, filename))
    .toString()
    .trim()
    .split("\n")
    .map((x) => ({
      row: x.split(" ")[0],
      springs: x
        .split(" ")[1]
        .split(",")
        .map((y) => parseInt(y)),
    }));

let arrCache: Record<string, number> = {};
const key = (row: string, springs: number[]): string => {
  return row + springs.join(",");
};

const arrangements = (row: string, springs: number[]): number => {
  let k = key(row, springs);
  if (k in arrCache) {
    return arrCache[key(row, springs)];
  }
  if (row.length == 0 && springs.length == 0) return 1;
  if (row.length == 0) return 0;
  if (springs.length == 0) return row.indexOf("#") === -1 ? 1 : 0;
  if (springs.reduce((s, ss) => s + ss, 0) > row.length) return 0;
  if (row[0] == ".") return arrangements(row.slice(1), springs);
  if (row[0] == "#") {
    for (let i = 0; i < springs[0]; i++) if (row[i] == ".") return 0;
    if (row[springs[0]] == "#") return 0;
    if (row[springs[0]] == "?")
      return arrangements("." + row.slice(springs[0] + 1), springs.slice(1));
    return arrangements(row.slice(springs[0]), springs.slice(1));
  }
  if (row[0] == "?") {
    let p = arrangements("." + row.slice(1), springs);
    let q = arrangements("#" + row.slice(1), springs);

    arrCache[k] = p + q;

    return p + q;
  }

  throw 1;
};

const solutionOne = (): number =>
  readInput().reduce((sum, { row, springs }) => {
    let r = arrangements(row, springs);
    return sum + r;
  }, 0);

export const solutionTwo = (): number =>
  readInput().reduce((sum, { row, springs }) => {
    return (
      sum +
      arrangements(`${row}?${row}?${row}?${row}?${row}`, [
        ...springs,
        ...springs,
        ...springs,
        ...springs,
        ...springs,
      ])
    );
  }, 0);

const part: string = process.env.part || "part1";

if (part === "part1") {
  console.log(solutionOne());
} else {
  console.log(solutionTwo());
}
