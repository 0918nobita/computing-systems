// GC ランタイムをアセンブリで実装するうえでアルゴリズムを確認するためのサンプル
// BASIC では __GC_COLLECT 手続き呼び出しで GC を強制実行できるようにする予定
// Copying GC または Lisp2 (Mark&Compact) GC

const heap = new Uint8Array(200);

const stack = [];

let toSpace = 100;
let fromSpace = 0;

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

heap[10] =  0;  // next (null)
heap[11] =  1;  // used
heap[12] =  2;  // size
heap[13] =  1;  // type: char
heap[14] = 66;  // value: 'B'

function malloc(size: number): number {
  let ptr = fromSpace;
  while (true) {
    const next = heap[ptr];
    const used = heap[ptr+1];
    const cellSize = heap[ptr+2];
    if (!used && size <= cellSize) {
      heap[ptr+1] = 1; // used
      heap[ptr+2] = size;
      for (let i = 0; i < size; i++) heap[ptr+3+i] = 0;
      break;
    }
    if (next === 0) {
      heap[ptr] = ptr + cellSize + 3;
      ptr += cellSize + 3;
      const next = ptr + size + 3;
      if (next > toSpace) collect();
      if (next > toSpace) throw new Error('insufficient memory');
      heap[ptr] = next
      heap[ptr+1] = 1; // used
      heap[ptr+2] = size;
      for (let i = 0; i < size; i++) heap[ptr+3+i] = 0;
      heap[next] = 0; // null
      break;
    }
    ptr = next;
  }
  return ptr;
}

function free() {
  // wip
}

function collect() {
  // wip
}

console.log('malloc(10) =>', malloc(10));
console.log('malloc(2) =>', malloc(2));
console.log('malloc(20) =>', malloc(20));
console.log('malloc(2) =>', malloc(2));
console.log('heap:', heap);
