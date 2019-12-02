const input = parseInt(process.argv[process.argv.length - 1], 10); 
const arr = new Array(1000).fill(0).map(a => new Array(100000).fill(0).map(a => []));
console.log(arr);