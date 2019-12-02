import { readFileSync } from "fs";

const input = readFileSync("day1.in")
    .toString()
    .split("\n")
    .map((m) => parseInt(m));
console.log(input);
const fuelRq = input.map((mass) => ~~(mass / 3) - 2).map(mass => calcFuelForFuel(mass) + mass).reduce((a, b) => a + b);
console.log(fuelRq);
function calcFuelForFuel(fuel: number) {
    let nFrq = fuel;
    let sum = 0;
    while (true) {
        let c = ~~(nFrq / 3) - 2;
        if (c <= 0) break;
        sum += c;
        nFrq = c;
    }
    return sum
}
