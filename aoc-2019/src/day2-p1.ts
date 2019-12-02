import { readFileSync } from "fs";
import VM from "./day2";

const input = readFileSync("day2.in")
    .toString()
    .split(",")
    .map((s) => parseInt(s, 10));

console.log(new VM(input).execute());
