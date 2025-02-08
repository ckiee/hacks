#!/usr/bin/env bun
import { readFile, writeFile } from "fs/promises";

const f = await readFile("userscript.js", "utf-8")
// console.log(`javascript:${encodeURIComponent(f)}`)

writeFile("bookmarklet.html", `
drag this into your bookmarks:
<a href="javascript:${encodeURIComponent(f)}">tokioverlay</a>
`, "utf-8")
process.stderr.write("wrote bookmarklet.html\n")
