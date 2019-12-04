const [min, max] = process.argv[process.argv.length - 1]
    .split("-")
    .map((s) => parseInt(s, 10));
let code = min;

function adjacentEqual(j: number): boolean {
    const str = j.toString();
    let last;
    let res = false;
    for (let c of str.split("")) {
        if (c == last) res = true;
        last = c;
    }
    return res;
}
function biggerEqual(j: number): boolean {
    const str = j.toString();
    let last = 0;
    let res = true;
    for (let c of str.split("")) {
        const current = parseInt(c, 10);
        if (last > current) res = false;
        last = current;
    }
    return res;
}
const arr = [];
while (code < max) {
    arr.push(code);
    code++;
}

const result = arr
    .filter((v) => adjacentEqual(v))
    .filter((v) => biggerEqual(v));
console.log(result.length);
