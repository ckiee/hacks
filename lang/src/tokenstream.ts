import { Token } from "./lexer";

export default class TokenStream {
    data: Token[];
    i: number = 0;
    constructor(data: Token[]) {
        this.data = data;
    }
    next() {
        this.i++;
        return this.data[this.i]
    }
    peek(l = 1) {
        return this.data[this.i + l]
    }
    skip(l = 1) {
        this.i += l
    }
    skipOver(type: string) {
        if (this.peek().type == type) this.skip()
        else throw new Error(`expected token to be of type "${type}" but got "${this.peek().type}"`)
    }
    get() {
        return this.data[this.i]
    }
    hasNext() {
        return this.data[this.i + 1] !== undefined;
    }
}
