#!/usr/bin/env node
const child_process = require("child_process");
const fs = require("fs");
const path = require("path");

const cleanPath = p => p.split("/")[3].replace(/[^a-zA-Z_]/g, "");

function generateDot(minRefs, path) {
  const derivs = JSON.parse(child_process.execSync(`@nix@/bin/nix path-info -sr ${path} --json`, { maxBuffer: Math.pow(2, 32) }).toString("utf8"));
  const refCounts = {};
  const preproc = derivs
    .map(d => {
      // ns = noStore
      d.nsPath = cleanPath(d.path);
      d.nsReferences = d.references.map(cleanPath);
      d.nsReferences.forEach(ref => {
        if (refCounts[ref] > 0) refCounts[ref]++;
        else refCounts[ref] = 1;
      });

      return d;
    })
    .filter(d => refCounts[d.nsPath] > minRefs);

  return `
digraph CurrentSystem {
node [shape=box];
${preproc.map(d => `${d.nsPath}[label=${JSON.stringify(d.path)}];`).join("\n")}

${preproc.map(d => `${d.nsPath} -> { ${d.nsReferences.join(" ")} };`).join("\n")}
}
`;
}

function graphvizRender(dot, output) {
  const tmpPath = fs.mkdtempSync("graphviz-paths");
  const dotPath = path.join(tmpPath, "ir.dot");
  fs.writeFileSync(dotPath, dot, "utf8");
  child_process.execSync(`@graphviz@/bin/dot -Ksfdp -v -Tpng -x -Goverlap=scale "${dotPath}" > ${output}`)
  fs.unlinkSync(dotPath);
  fs.rmdirSync(tmpPath);
}

function abort(...args) {
  console.error(...args);
  process.exit(1);
}

function main() {
  const args = process.argv;
  args.shift();
  args.shift();
  let help = false, minRefs = 7, path, output;
  while (args.length > 0) {
    const arg = args.shift();
    if (arg == "--help") {
      help = true;
    } else if (arg == "--min-refs") {
      if (args.length < 0) {
        help = true;
        break;
      }
      minRefs = parseInt(args.shift());
    } else if (!path && !output) {
      path = arg;
    } else if (path && !output) {
      output = arg;
    } else {
      help = true;
    }
  }
  if (help || !path) {
    abort(`USAGE: graphviz-paths [--min-refs 7] <INPUT> <OUTPUT>
    `);
  } else {
    graphvizRender(generateDot(minRefs, path), output);
  }
}

main()
