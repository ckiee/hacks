import VM from "./day2";
import { readFileSync } from "fs";
const wanted = parseInt(process.argv[process.argv.length - 1]);
console.log("wanted", wanted)

for (let i = 0; i < 100; i++) {
    for (let j = 0; j < 100; j++) {
        const input = readFileSync("day2.in")
            .toString()
            .split(",")
            .map((s) => parseInt(s, 10));
        input[1] = i
        input[2] = j;
        const res = new VM(input).execute();
        // console.log({inputCpy,i,j,res,wanted});
        if (res[0] == wanted) {
            console.log("MATCH", i, j, 100*i +j)
            break;
        }
    }
}
