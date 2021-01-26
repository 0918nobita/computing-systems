// GC ランタイムをアセンブリで実装するうえでアルゴリズムを確認するためのサンプル
// BASIC では __GC_COLLECT 手続き呼び出しで GC を強制実行できるようにする予定
// Copying GC または Lisp2 (Mark&Compact) GC

const heap = new Uint8Array(100);

const stack = [];

// const toSpace = 50;
// const fromSpace = 0;
// let allocPtr = fromSpace;

heap[0] =  5;  // next
heap[1] =  1;  // used
heap[2] =  2;  // size
heap[3] =  1;  // type: char
heap[4] = 65;  // value: 'A'

heap[5] = 10;  // next
heap[6] =  0;  // used
heap[7] =  2;  // size
heap[8] =  0;
heap[9] =  0;

heap[10] =  0;  // next
heap[11] =  1;  // used
heap[12] =  2;  // size
heap[13] =  1;  // type: char
heap[14] = 66;  // value: 'B'

function collect() {
  // wip
}

console.log({ heap });
