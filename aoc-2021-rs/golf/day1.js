d=`${require("fs").readFileSync("../inputs/1")}`.split`\n`.map(x=>+x).filter(x=>x==x)
console.log([1,3].map(w=>Array(2000-w).fill``.filter((_,i)=>(s=i=>(d.slice(i,i+w)).reduce((a,b)=>a+b,0))(i)>(i?s(i-1):0)).length))
