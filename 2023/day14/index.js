const { log } = require("console");
const fs = require("fs");

function readInput(filename = "input.txt") {
  return fs
    .readFileSync(filename)
    .toString()
    .trim()
    .split("\n")
    .map((x) => x.split(""));
}

function getSolutionPart1() {
  let rocks = readInput();
  let load = 0;
  for (let y = 0; y < rocks.length; y++) {
    for (let x = 0; x < rocks[y].length; x++) {
      if (rocks[y][x] != "O") continue;
      let newY = y;
      while (newY > 0 && rocks[newY - 1][x] == ".") newY--;
      load += rocks.length - newY;
      if (newY == y) continue;
      rocks[newY][x] = "O";
      rocks[y][x] = ".";
    }
  }
  return load;
}

function moveNorth(rocks) {
  for (let y = 0; y < rocks.length; y++) {
    for (let x = 0; x < rocks[y].length; x++) {
      if (rocks[y][x] != "O") continue;
      let newY = y;
      while (newY > 0 && rocks[newY - 1][x] == ".") newY--;
      if (newY == y) continue;
      rocks[newY][x] = "O";
      rocks[y][x] = ".";
    }
  }
}

function moveWest(rocks) {
  for (let x = 0; x < rocks[0].length; x++) {
    for (let y = 0; y < rocks.length; y++) {
      if (rocks[y][x] != "O") continue;
      let newX = x;
      while (newX > 0 && rocks[y][newX - 1] == ".") newX--;
      if (newX == x) continue;
      rocks[y][newX] = "O";
      rocks[y][x] = ".";
    }
  }
}

function moveSouth(rocks) {
  for (let y = rocks.length - 1; y >= 0; y--) {
    for (let x = 0; x < rocks[y].length; x++) {
      if (rocks[y][x] != "O") continue;
      let newY = y;
      while (newY < rocks.length - 1 && rocks[newY + 1][x] == ".") newY++;
      if (newY == y) continue;
      rocks[newY][x] = "O";
      rocks[y][x] = ".";
    }
  }
}

function moveEast(rocks) {
  for (let x = rocks[0].length - 1; x >= 0; x--) {
    for (let y = 0; y < rocks.length; y++) {
      if (rocks[y][x] != "O") continue;
      let newX = x;
      while (newX < rocks[y].length - 1 && rocks[y][newX + 1] == ".") newX++;
      if (newX == x) continue;
      rocks[y][newX] = "O";
      rocks[y][x] = ".";
    }
  }
}

function getLoad(rocks) {
  let load = 0;
  for (let y = 0; y < rocks.length; y++) {
    for (let x = 0; x < rocks[y].length; x++) {
      if (rocks[y][x] == "O") load += rocks.length - y;
    }
  }
  return load;
}
function getSolutionPart2() {
  let rocks = readInput();
  last = 0;
  same = 0;
  for (let i = 0; i < 1000; i++) {
    moveNorth(rocks);
    moveWest(rocks);
    moveSouth(rocks);
    moveEast(rocks);
    // Find cycle lenght
    // if (getLoad(rocks) == 100680) {
    //   console.log(i - last);
    //   last = i;
    // }
    if ((i + 1) % 39 == 25) {
      let load = getLoad(rocks);
      if (load == last) same++;
      else same = 0;
      if (same > 10) return load;
      last = load;
    }
  }
}

const part = process.env.part || "part1";

if (part === "part1") console.log(getSolutionPart1());
else console.log(getSolutionPart2());

module.exports = {
  getSolutionPart1,
  getSolutionPart2,
};
