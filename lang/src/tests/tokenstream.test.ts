import TokenStream from "../tokenstream";
let ts: TokenStream;

beforeEach(() => {
    ts = new TokenStream([
        { type: "identifier", match: "hello" },
        { type: "identifier", match: "world" }
    ]);
});

test("it skips over tokens", () => {
    ts.skipOver("identifier");
    expect(ts.get()).toEqual({ type: "identifier", match: "world" });
});

test("it fails skipping over the wrong type of token", () => {
    expect(() => {
        ts.skipOver("whitespace");
    }).toThrow(/expected token to be of type .+/);
});

test("it peeks", () => {
    expect(ts.peek()).toEqual({ type: "identifier", match: "world" });
});

test("it goes to the next token", () => {
    expect(ts.next()).toEqual({ type: "identifier", match: "world" });
});

test("it gets the current token", () => {
    expect(ts.get()).toEqual({ type: "identifier", match: "hello" });
});

test("it goes to the end of the tokenstream", () => {
    ts.next();
    expect(ts.next()).toEqual(undefined);
});

test("it knows about the end", () => {
    ts.next();
    expect(ts.hasNext()).toEqual(false);
});
