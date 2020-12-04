const fs = require("fs");
const inp = fs.readFileSync("inputs/day4").toString()
/*
    byr (Birth Year)
    iyr (Issue Year)
    eyr (Expiration Year)
    hgt (Height)
    hcl (Hair Color)
    ecl (Eye Color)
    pid (Passport ID)
    cid (Country ID)
*/
const req = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];

i=0
inp.split("\n\n").map(x=>req.every(h=>x.includes(h))).forEach(x=>x?i++:0)
console.log(i);