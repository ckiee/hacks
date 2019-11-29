import readline from "readline";
import Lexer from "./lexer";
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});
function question(question: string): Promise<string> {
    return new Promise(resolve => {
        rl.question(question, data => {
            resolve(data);
        })
    });
}
async function repl() {
    while (true) {
        const answer = await question("> ");
        console.log(new Lexer(answer, process.argv.includes("--ignore-whitespace") || process.argv.includes("-i")).lex())
    }
}
repl();