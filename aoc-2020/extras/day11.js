let cycle = `L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL`
	.split("\n")
	.map(x => x.split(""));

function adj(data, x, y) {
	// console.log(data[i][j]);
	let score = 0;
	for (let i = -1; i <= 1; i++) {
		for (let j = -1; j <= 1; j++) {
			try {
				if (data[i + x][j + y] == "#") score++;
			} catch (e) {}
		}
	}
	return score;
}
let lastCycle = "";

while (lastCycle !== cycle) {
	cycle = cycle.map((x, i, a) =>
		x.map((y, j, b) => {
			if (y == "L" && adj(a, i, j) == 0) return "#";
			else if (y == "#" && adj(a, i, j) >= 4) return "L";
			else return y;
		})
	);
	console.log(cycle.map(x => x.join("")).join("\n"), "\n------");
	lastCycle = cycle;
}
console.log(
	cycle.map(x => x.filter(x => x == "#").length).reduce((a, b) => a + b)
);
