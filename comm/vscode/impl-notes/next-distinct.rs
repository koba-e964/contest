// 隣同士が異なるように並べ替えるには、残っている頻度最大のもので、前の文字と違うものを取っていく。
// state: (freq: int[], last: int)
let mut freq = vec![0; 26];
let mut last = !0usize;
