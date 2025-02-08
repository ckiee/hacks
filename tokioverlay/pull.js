#!/usr/bin/env bun

const words = await fetch("https://api.linku.la/v1/words").then(r => r.json());

for (const key in words) {
  words[key] = words[key]?.translations?.en?.definition
}

console.log(JSON.stringify(words))
