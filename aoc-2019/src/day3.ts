import { readFileSync } from "fs";

export function manhattan(x0: number, x1: number, y0: number, y1: number) {
    return Math.abs(x1 - x0) + Math.abs(y1 - y0);
}
const world: [number, number][] = [];
enum Direction {
    Right = "R",
    Up = "U",
    Left = "L",
    Down = "D"
}
export class Wire {
    x: number = 0;
    y: number = 0;
    hits: [number, number][] = [];
    hitCheck() {
        if (world.find((arr) => arr[0] == this.x && arr[1] == this.y)) {
            this.hits.push([this.x, this.y]);
            console.log("hit", this.x, this.y, manhattan(0, this.x, 0, this.y));
        }
    }
    step(d: Direction | string, amount: number, returnHit: boolean) {
        while (amount !== 0) {
            switch (d) {
                case Direction.Down:
                    this.y -= 1;
                    break;
                case Direction.Up:
                    this.y += 1;
                    break;
                case Direction.Right:
                    this.x += 1;
                    break;
                case Direction.Left:
                    this.x -= 1;
                    break;
            }
            if (returnHit) this.hitCheck();
            world.push([this.x, this.y]);
            amount--;
        }
    }
    parseAndStep(str: string, returnHit: boolean) {
        str.split(",").forEach((step) => {
            this.step(
                step.substr(0, 1),
                parseInt(step.substring(1), 10),
                returnHit
            );
        });
    }
}
const wire1 = new Wire();
const lines = readFileSync("day3.in")
    .toString()
    .split("\n");
wire1.parseAndStep(lines[0], false);
const wire2 = new Wire();
wire2.parseAndStep(lines[1], true);
const sortedHits = wire2.hits.sort((a, b) => {
    return manhattan(0, a[0], 0, a[1]) - manhattan(0, b[0], 0, b[1]);
});
console.log({ wire1, wire2, hits: wire2.hits, sortedHits });
console.log("result", manhattan(0, sortedHits[0][0], 0, sortedHits[0][1]));
console.log("this ones a bit weird - needed to use the 2nd best hit")