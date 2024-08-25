const _ = require("lodash");
const fs = require("fs");
const pr = console.log;
const file = fs.readFileSync("input.txt", {encoding: "utf-8"}).toString();
const lines = file.split("\n").map(l=> l.split(" "));

let score = 0;
for (let line of lines) {
    if (line.length!=2)continue;
    const [them,needr] = line.map((x,i)=>[...[`ABC`,`XYZ`][i]].indexOf(x));
    let choice = [them==0?2:them-1,them,them==2?0:them+1][needr]
    pr({them,choice,needr})
    let inc = (choice+1) + (them==choice?3:[`02`,`21`,`10`].includes([them,choice].join``)?0:6)
    score += inc;
    pr({inc, score});
    pr()
}
pr(score)
