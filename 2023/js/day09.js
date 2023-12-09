data = `0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45`;

interpolate_prev = (seq) => {
  let iters = [];
  let iter = seq.slice(1).map((x, i) => x - seq[i]);
  while (!iter.every((x) => x === 0)) {
    iters.push(iter);
    iter = iter.slice(1).map((x, i) => x - iter[i]);
  }
  let diff = iters.reverse().reduce((a, ii) => ii[0] - a, 0);
    console.log(seq, seq[0] - diff);
  return seq[0] - diff;
};

interpolate_next = (seq) => {
  let iters = [];
  let iter = seq.slice(1).map((x, i) => x - seq[i]);
  while (!iter.every((x) => x === 0)) {
    iters.push(iter);
    iter = iter.slice(1).map((x, i) => x - iter[i]);
  }
  let diff = iters.reverse().reduce((a, ii) => ii[ii.length - 1] + a, 0);
  return diff + seq[seq.length - 1];
};

ans = data.split`\n`
  .map((x) => x.split` `.map((y) => +y))
  .map(interpolate_prev)
  .reduce((a, b) => a + b, 0);
console.log(ans);
