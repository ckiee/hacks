import { readFileSync } from "fs";

const input = readFileSync("day1.in")
    .toString()
    .split("")
    .map((s) => parseInt(s, 10));

let i = 0;
let total = 0;
while (i < input.length) {
    const v = input[i];
    const next = input[i + 1] || input[0]
    if (v == next) {
        total += v
    }
    i++;
}
console.log(total);
