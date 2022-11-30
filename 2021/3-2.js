#!/usr/sbin/node
let fs = require('fs');

fs.readFile('inputs/3.txt', 'utf-8', (err, txt) => {
    let arr = txt.trim()
    .split('\n');

    amax = [...arr];
    amin = [...arr];
    let len = arr[0].length;


    for (i = 0; i < len; i++) {
        let fx = amax.map(x => x[i]).reduce((a, b) => a + +b, 0) / amax.length;
        let dig = fx < 0.5 ? 0 : 1;
        if (amax.length == 1) break;
        amax = amax.filter(x => x[i] == dig);
    };

    for (i = 0; i < len; i++) {
        let fx = amin.map(x => x[i]).reduce((a, b) => a + +b, 0) / amin.length;
        let dig = fx < 0.5 ? 1 : 0;
        if (amin.length == 1) break;
        amin = amin.filter(x => x[i] == dig);
    };

    minn = parseInt(amin, 2);
    maxx = parseInt(amax, 2);

    console.log(minn * maxx);
});
