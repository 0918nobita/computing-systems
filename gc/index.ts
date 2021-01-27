// GC ランタイムをアセンブリで実装するうえでアルゴリズムを確認するためのサンプル
// BASIC では __GC_COLLECT 手続き呼び出しで GC を強制実行できるようにする予定
// Copying GC または Lisp2 (Mark&Compact) GC

const TYPE_BYTE = 1;
const SIZE_BYTE = 2;

const TYPE_CHAR = 2;
const SIZE_CHAR = 2;

const TYPE_PTR = 3;
const SIZE_PTR = 2;

type Type =
  | typeof TYPE_BYTE
  | typeof TYPE_CHAR
  | typeof TYPE_PTR;

const heap = new Uint8Array(200);

const stack: number[] = [];

let toSpace = 100;
let fromSpace = 0;

heap[0] = 0;  // next: null
heap[1] = 0;  // forwarding-pointer
heap[2] = 2;  // size
heap[3] = TYPE_BYTE;
heap[4] = 0;  // value

function allocate(size: number, type: Type): number {
  let ptr = fromSpace;

  while (true) {
    const next = heap[ptr];

    if (next === 0) {
      const cellSize = heap[ptr+2];
      const newPtr = ptr + cellSize + 3;
      if (newPtr + size + 3 > fromSpace + 100) collect();
      heap[ptr]      = newPtr;
      heap[newPtr]   = 0;       // next: null
      heap[newPtr+1] = newPtr;  // forwarding-pointer
      heap[newPtr+2] = size;
      heap[newPtr+3] = type;
      for (let i = 0; i < size; i++) heap[newPtr+4+i] = 0;
      return newPtr + 4;
    }

    ptr = next;
  }
}

function collect(): void {
  // wip
}

const ptrA = allocate(SIZE_CHAR, TYPE_CHAR);
heap[ptrA] = 65;  // will not be marked

const ptrB = allocate(SIZE_CHAR, TYPE_CHAR);
heap[ptrB] = 66;  // will be marked
stack.push(ptrB);
stack.push(TYPE_PTR);

console.log('stack:', stack);
console.log('heap (first half):', heap.slice(0, 100));
console.log('heap (second half):', heap.slice(100));

collect();
