import readline from "readline";
import Interperter from ".";
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});
function repl() {
    rl.question("> ", (answer) => {
		if (answer.includes("cleanup")) {
			console.log(answer.split("").filter(v => "><+-.,[]#".includes(v)).join(""));
			repl();
		}
		if (answer.includes("raw")) answer = answer.split(".").join("#");
		const int = new Interperter();
		int.debug = answer.includes("dbg");
		int.run(answer);
		repl();
    });
}
repl();