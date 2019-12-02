import { readFileSync } from "fs";

const input = readFileSync("day1.in")
    .toString()
    .split("")
    .map((s) => parseInt(s, 10));

let i = 0;
let total = 0;
while (i < input.length) {
    const v = input[i];
    const stepsRequired = input.length / 2
    const next = input[i + stepsRequired] || input[stepsRequired - (input.length - i)]
    if (v == next) {
        total += v
    }
    i++;
}
console.log(total);
