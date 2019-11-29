type RegexpMap = { [key: string]: RegExp };

export const regexps = {
    comment: /^(\/\/.*)|(\/\*(.|[\n])*\*\/)/,
    keyword: /^const/,
    identifier: /^\w+/,
    seperator: /^(\(|\)|{|})/,
    whitespace: /^\s+/
} as RegexpMap;

export interface Token {
    type: string;
    match: string;
}

export default class Lexer {
    data: string;
    i: number = 0;
    debug: boolean = false;
    tokens: Token[] = [];
    ignoreWhitespace: boolean;
    constructor(data: string, ignoreWhitespace: boolean = false) {
        this.data = data;
        this.ignoreWhitespace = ignoreWhitespace;
    }
    debugLog(...args: any) {
        if (this.debug) {
            console.log(...args);
        }
    }
    lex() {
        while (this.data.length > this.i) {
            const char = this.data[this.i];
            const unprocessed = this.data.substring(this.i);

            this.debugLog(this.i, char);
            let lexed = false;
            for (const k of Object.keys(regexps)) {
                const v = regexps[k];
                const result = v.exec(unprocessed);
                if (!result) continue;
                const match = result[0];
                this.i += match.length - 1;
                if (k !== "whitespace" || !this.ignoreWhitespace) {
                    this.tokens.push({ type: k, match });
                }
                lexed = true;
                break;
            }
            if (!lexed) {
                console.error(`Lex Error, unknown token at position ${this.i}`);
                break;
            }
            this.debugLog("");
            this.i++;
        }
        return this.tokens;
    }
}
