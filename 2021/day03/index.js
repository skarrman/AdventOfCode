const fs = require('fs')

function inputDataLinesIntegers(filename = "input.txt") {
    return fs.readFileSync(filename).toString().trim().split("\n")
}

function calculateBits(input, index = null) {
    let vars = Array.apply(null, Array(input[0].length)).map(_ => 0);
    for (let i = 0; i < input.length; i++)
        if (index == null)
            for (let j = 0; j < input[i].length; j++)
                vars[j] += parseInt(input[i].charAt(j))
        else
            vars[index] += parseInt(input[i].charAt(index))
    return { v: index == null ? vars : vars[index], l: input.length };
}

function getSolutionPart1() {
    let data = calculateBits(inputDataLinesIntegers())
    let vars = { gamma: "", elipson: "" }
    for (let j = 0; j < data.v.length; j++) {
        vars.gamma += data.v[j] > data.l / 2 ? "1" : "0"
        vars.elipson += data.v[j] < data.l / 2 ? "1" : "0"
    }
    return parseInt(vars.gamma, 2) * parseInt(vars.elipson, 2)
}

function getSolutionPart2() {
    let inputO2 = inputDataLinesIntegers();
    let inputCO2 = inputDataLinesIntegers();
    let length = inputCO2[0].length
    let vars = { o2: "", co2: "" }
    for (let j = 0; j < length; j++) {
        let common = calculateBits(inputO2, j);
        let one = common.v >= common.l / 2;
        vars.o2 += one ? "1" : "0"
        inputO2 = inputO2.filter(x => x.charAt(j) == (one ? '1' : '0'));
        if (inputCO2.length == 1) {
            vars.co2 += inputCO2[0].charAt(j);
            continue;
        }
        common = calculateBits(inputCO2, j);
        one = common.v >= common.l / 2;
        vars.co2 += one ? "0" : "1"
        inputCO2 = inputCO2.filter(x => x.charAt(j) == (one ? '0' : '1'));
    }
    return parseInt(vars.co2, 2) * parseInt(vars.o2, 2)
}
const part = process.env.part || "part1"

if (part === "part1")
    console.log(getSolutionPart1())
else
    console.log(getSolutionPart2())

module.exports = {
    getSolutionPart1, getSolutionPart2, inputDataLinesIntegers
}