/*
data = `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`;
/**/

lines = data.split`\n`.map((l) => l.split` `.map((x) => +x));

isSafe = (x) => {
    o = x[1] - x[0];
    if (Math.abs(o) > 3 || o === 0) return false;
    return x.reduce((a, b, i) => {
        return (
            a &&
            (i === 0
                ? true
                : Math.abs(b - x[i - 1]) > 3
                  ? false
                  : (b - x[i - 1]) * o > 0)
        );
    }, true);
};

silver = lines.map(isSafe).filter((x) => x).length;

gold = lines
    .map((x) => {
        if (isSafe(x)) return true;
        console.log('arr: ', x);
        for (i in x) {
            aa = [...x.slice(0, +i), ...x.slice(+i + 1)];
            console.log(aa);
            if (isSafe(aa)) return true;
        }
        return false;
    })
    .filter((x) => x).length;
