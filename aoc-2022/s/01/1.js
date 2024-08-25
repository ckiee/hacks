const _ = require("lodash");
const fs = require("fs");
const pr = console.log;
const file = fs.readFileSync("input.txt", {encoding: "utf-8"}).toString();
const lines = file.split("\n");

pr(
    lines
)

