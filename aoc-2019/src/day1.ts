import { readFileSync } from "fs";

const input = readFileSync("day1.in").toString();

console.log(
    input
        .split("\n")
        .map(m => parseInt(m))
        .map((mass) => ~~(mass / 3) - 2)
        .reduce((a, b) => a + b)
);
