const _ = require("lodash");
const fs = require("fs");
const pr = console.log;
const file = fs.readFileSync("input.txt", {encoding: "utf-8"}).toString();
const lines = file.split("\n").map(l=> l.split(" "));

let score = 0;
for (let line of lines) {
    if (line.length!=2)continue;
    const idxs = line.map((x,i)=>[...[`ABC`,`XYZ`][i]].indexOf(x));
    console.log(idxs);
    pr("a", idxs.join``)
    let inc = (idxs[1]+1) + (idxs[0]==idxs[1]?3:[`02`,`21`,`10`].includes(idxs.join``)?0:6)
    score += inc;
    pr({inc, score});
}
// pr(score)
