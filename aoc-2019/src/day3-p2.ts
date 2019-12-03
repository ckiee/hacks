import { readFileSync } from "fs";

export function manhattan(x0: number, x1: number, y0: number, y1: number) {
    return Math.abs(x1 - x0) + Math.abs(y1 - y0);
}
const world: [number, number, number][] = [];
enum Direction {
    Right = "R",
    Up = "U",
    Left = "L",
    Down = "D"
}
export class Wire {
    steps = 0;
    x = 0;
    y = 0;
    hits: [number, number, number, number, number][] = [];
    hitCheck(): boolean {
        const find = world.find((arr) => arr[0] == this.x && arr[1] == this.y)
        if (find) {
            this.hits.push([this.x, this.y, this.steps, find[2], this.steps+find[2]]);
            console.log("hit", this.x, this.y, manhattan(0, this.x, 0, this.y));
        }
        return !!find;
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
            this.steps++
            if (returnHit &&this.hitCheck()) {}
            else world.push([this.x, this.y, this.steps]);
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
    return a[4]-b[4]
});
console.log({ wire1, wire2, hits: wire2.hits, sortedHits });
