import Lexer, { regexps } from "../lexer";
const testedTypes = ["comment", "keyword", "whitespace", "identifier"];

test("it lexes comment", () => {
    const tokens = new Lexer("//hello").lex();
    expect(tokens).toEqual([{ match: "//hello", type: "comment" }]);

    const tokens2 = new Lexer("/*hello\nworld*/").lex();
    expect(tokens2).toEqual([{ match: "/*hello\nworld*/", type: "comment" }]);
});

test("it lexes keywords", () => {
    const tokens = new Lexer("const").lex();
    expect(tokens).toEqual([{ match: "const", type: "keyword" }]);
});

test("it lexes whitespace", () => {
    const tokens = new Lexer("            \n\n    ").lex();
    expect(tokens).toEqual([
        { match: "            \n\n    ", type: "whitespace" }
    ]);
});

test("it lexes identifier", () => {
    const tokens = new Lexer("world").lex();
    expect(tokens).toEqual([{ match: "world", type: "identifier" }]);
});

test("all token types are covered by tests", () => {
    expect(testedTypes).toEqual(Object.keys(regexps));
});
