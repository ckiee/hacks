import { readFileSync } from "fs";

const input = readFileSync("day1.in")
	.toString()
	.split("\n")
	.map(x => parseInt(x, 10));

for (const x of input) {
	for (const y of input) {
		for (const z of input) {
			if (x + y + z == 2020) {
				console.log("Solution", x * y * z);
			}
		}
	}
}
