import * as fs from "fs";
import path from "path";

const readInput = (filename = "../input.txt"): string =>
  fs
    .readFileSync(path.resolve(__dirname, filename))
    .toString()
    .trim()

export const solve = (unique: number): number => {
  const input = readInput();
  for(let i = unique-1; i < input.length; i++){
    let letters = []
    for(let j = 0; j < unique; j++){
      letters.push(input[i-j])
    }
    var uniquelst = letters.filter((v, i, a) => a.indexOf(v) === i);
    if(uniquelst.length == unique){
      return i+1;
    }
  }
  return -1;
};

const part: string = process.env.part || "part1";

if (part === "part1") {
  console.log(solve(4));
} else {
  console.log(solve(14));
}
