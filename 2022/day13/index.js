const fs = require("fs");

function inputPairs(filename = "input.txt") {
  return fs
    .readFileSync(filename)
    .toString()
    .trim()
    .split("\n\n")
    .reduce((pairs, part) => {
      let pairPair = part.split("\n");
      let fst = JSON.parse(pairPair[0]);
      let snd = JSON.parse(pairPair[1]);
      pairs.push({ fst: fst, snd: snd });
      return pairs;
    }, []);
}

function rightOrder(leftArray, rightArray, index) {
  if (rightArray.length <= index || leftArray.length <= index) {
    if (leftArray.length == rightArray.length) return undefined;
    return leftArray.length < rightArray.length;
  }
  let left = leftArray[index];
  let right = rightArray[index];
  if (typeof left == "number" && typeof right == "number") {
    if (left < right) return true;
    if (left > right) return false;
    return rightOrder(leftArray, rightArray, index + 1);
  }
  let res = undefined;
  if (typeof left == "number") res = rightOrder([left], right, 0);
  else if (typeof right == "number") res = rightOrder(left, [right], 0);
  else res = rightOrder(left, right, 0);
  if (res == undefined) return rightOrder(leftArray, rightArray, index + 1);
  return res;
}

function getSolutionPart1() {
  return inputPairs().reduce((sum, pair, index) => {
    return sum + (rightOrder(pair.fst, pair.snd, 0) ? index + 1 : 0);
  }, 0);
}

function sortPackets(left, right) {
  if (rightOrder(left, right, 0)) return -1;
  return 1;
}

function getSolutionPart2() {
  let values = inputPairs().reduce((lst, pair) => {
    lst.push(pair.fst);
    lst.push(pair.snd);
    return lst;
  }, []);
  values.push([[2]]);
  values.push([[6]]);
  values.sort(sortPackets);
  let fst = -1;
  let snd = -1;
  for (let i = 0; i < values.length; i++) {
    if (JSON.stringify(values[i]) === JSON.stringify([[2]])) fst = i + 1;
    if (JSON.stringify(values[i]) === JSON.stringify([[6]])) {
      snd = i + 1;
      break;
    }
  }
  return fst * snd;
}

const part = process.env.part || "part1";

if (part === "part1") console.log(getSolutionPart1());
else console.log(getSolutionPart2());

module.exports = {
  getSolutionPart1,
  getSolutionPart2,
  sortPackets,
  inputPairs,
};
