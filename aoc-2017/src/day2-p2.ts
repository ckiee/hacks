import { readFileSync } from "fs";

const input = readFileSync("day2.in")
    .toString()
    .split("\n")
    .map((l) => l.split(/\s+/).map((s) => parseInt(s)));
console.log(
    input
        .map((l) => {
            let sum = 0;
            l.forEach((x, i) => {
                l.forEach((y, j) => {
                    if (i == j) return;
                    // console.log(x, y, x / y, ~~x / y);
                    if (x / y !== parseInt(`` + x / y)) return;
                    sum += x / y;
                });
            });
            return sum;
        })
        .reduce((a, b) => a + b)
);
