export default class Interperter {
    debug: boolean = false;
    mem: number[] = [];
    memIndex: number = 0;
    jumpToNextClosing: boolean = false;
    jumpToNextOpening: boolean = false;
    get memC() {
        return this.mem[this.memIndex] || 0;
    }
    run(code: string) {
        let pos = 0;
        while (pos !== code.length) {
            const char = code[pos];
            if (this.debug) console.log("\ndbg now at char", pos + 1);
            if (this.debug)
                console.log(
                    "dbg (start)",
                    this.mem.map((n) => n.toString(16)).join("")
                );

            if (this.jumpToNextClosing && code[pos] !== "]") {
                pos++;
                continue;
            } else if (this.jumpToNextOpening && code[pos] !== "[") {
                pos--;
                continue;
            } else {
                if (this.debug) console.log("dbg (start)", code);
                if (this.debug)
                    console.log("dbg (start)", " ".repeat(pos) + "^");
            }
            switch (char) {
                case ">":
                    this.memIndex++;
                    break;
                case "<":
                    this.memIndex--;
                    break;
                case "+":
                    this.mem[this.memIndex] = this.memC + 1;
                    break;
                case "-":
                    this.mem[this.memIndex] = this.memC - 1;
                    if (this.memC < 0) this.mem[this.memIndex] = 0;
                    break;
                case ".":
                    console.log(
                        "outascii",
						`"` + String.fromCharCode(this.memC) + `"`,
						this.debug ? `aka ${this.memC}` : ""
                    );
                    break;
				// TODO: impl ","
				case ",":
					console.error(", is unimplemented")
					pos = code.length-1;
					break;
                case "[":
                    if (this.jumpToNextOpening) {
                        this.jumpToNextOpening = false;
                        break;
                    }
                    if (this.memC !== 0) break;

                    this.jumpToNextClosing = true;
                    break;
                case "]":
                    if (this.jumpToNextClosing) {
                        this.jumpToNextClosing = false;
                        break;
                    }
                    if (this.memC !== 0) this.jumpToNextOpening = true;
                    break;

                case "#":
                    console.log("outnum", this.memC);
                    break;
            }
            if (this.debug)
                console.log(
                    "dbg (end)",
                    this.mem.map((n) => n.toString(16)).join(" ")
                );

            if (this.jumpToNextOpening) pos--;
            else pos++;
        }
    }
}

// new Interperter().run("++-[+-]+++#");
