#!/usr/sbin/node
let fs = require('fs');
const fnWin = arr => {
    row = arr;
    col = [0,1,2,3,4].map(i => arr.map(y => y[i]));
    wrow = row.find(x => x.join('') == 'xxxxx');
    wcol = col.find(x => x.join('') == 'xxxxx');
    if (wrow) return true;
    if (wcol) return true;
    return false;
}

fs.readFile('4.txt', 'utf-8', (err, txt) => {
    arr = txt.split('\n\n');
    nums = arr[0].split(',').map(x => x);
    aa = arr.slice(1);

    boards = aa.map(x => x.split('\n').map(y => y.trim().split(/\s+/g).map(z => z.trim())));

    d = 0;
    win = undefined;

    while (!win) {
        draw = nums[d];
        boards = boards.map(board => {
            return board.map(row => row.map(col => col == draw ? 'x' : col));
        });
        win = boards.find(fnWin)
        d++;
    }

    tot = win.reduce((a, b) => a + b.reduce((x, y) => x + (+y || 0) , 0), 0);
    console.log(tot * draw);
});
