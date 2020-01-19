import { createInterface } from "readline";
import { readFileSync } from "fs";

const rl = createInterface(process.stdin, process.stdout);
function question(query: string): Promise<string> {
    return new Promise((resolve) => {
        rl.question(query, (s) => resolve(s));
    });
}
export default class VM {
    data: number[];
    i: number = 0;
    get n() {
        return this.data[this.i];
    }
    constructor(data: number[]) {
        this.data = data;
    }
    async execute() {
        while (this.i < this.data.length) {
            const opcode = parseInt(this.n.toString().slice(-2), 10);
            let pmlen = this.n.toString().split("").length - 2;
            if (pmlen < 0) pmlen = 0;
            const paramsModes = new Array(pmlen).fill(0).map(
                (v, i) =>
                    parseInt(
                        this.n
                            .toString()
                            .split("")
                            .reverse()[i],
                        10
                    ) || 0
            );
            console.log({ opcode, paramsModes });
            if (opcode == 99) break;
            if (opcode == 1 || this.n == 2) {
                const addr = this.data[this.i + 3] || 0;
                const x = this.data[this.data[this.i + 1] || 0] || 0;
                const y = this.data[this.data[this.i + 2] || 0] || 0;
                this.data[addr] = this.n == 1 ? x + y : x * y;
                this.i += 4;
                continue;
            } else if (opcode == 3) {
                const answer = await question("> ");
                this.data[this.data[this.i + 1]] = parseInt(answer, 10);
                this.i += 2;
                continue;
                // console.error(`invalid value ${this.n} at pos ${this.i}`);
            } else if (opcode == 4) {
                console.log(this.data[this.data[this.i + 1]]);
                this.i += 2;
                continue;
            }

            this.i++;
        }
        return this.data;
    }
}

const input = readFileSync("day5.in")
    .toString()
    .split(",")
    .map((s) => parseInt(s, 10));
new VM(input).execute().catch(console.error);
