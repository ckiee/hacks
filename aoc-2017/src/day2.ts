import { readFileSync } from "fs";

const input = readFileSync("day2.in")
    .toString()
    .split("\n")
    .map((l) => l.split("\t").map((s) => parseInt(s)));
console.log(input.map(l => Math.max(...l) - Math.min(...l)).reduce((a,b)=>a+b))