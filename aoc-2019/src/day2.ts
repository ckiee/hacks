import { readFileSync } from "fs";

const input = readFileSync("day2.in")
    .toString()
    .split(",")
    .map((s) => parseInt(s, 10));

class VM {
    data: number[];
    i: number = 0;
    get n() {
        return this.data[this.i];
    }
    constructor(data: number[]) {
        this.data = data;
    }
    execute() {
        while (this.i < this.data.length) {
            if (this.n == 99) break;
            if (this.n == 1 || this.n == 2) {
                const addr = this.data[this.i + 3];
                const x = this.data[this.data[this.i + 1]];
                const y = this.data[this.data[this.i + 2]];
                this.data[addr] = this.n == 1 ? x + y : x * y;
                this.i += 4;
                continue;
            } else {
                console.error(`invalid value ${this.n} at pos ${this.i}`);
            }

            this.i++;
        }
        return this.data;
    }
}
console.log(new VM(input).execute());
