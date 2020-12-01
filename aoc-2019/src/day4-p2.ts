const [min, max] = process.argv[process.argv.length - 1]
	.split("-")
	.map(s => parseInt(s, 10));
let code = min;

function rollaroundIndex<T>(arr: T[], index: number, offset: number): T {
	index += offset;
	return arr[Math.abs(index % arr.length)];
}

function adjacentEqual(j: number): boolean {
	const str = j.toString();
	const result = /(.)\1/g.exec(str);

	if (!result) return false;
	return !/(.)\1\1/g.test(str.substr(result.index, result.index + 2));
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

[112233, 123444, 111122].forEach(x => console.log(x, adjacentEqual(x)));

const arr = [];
while (code < max) {
	arr.push(code);
	code++;
}

const result = arr.filter(v => adjacentEqual(v)).filter(v => biggerEqual(v));
console.log(result.length);
