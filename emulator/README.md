# x86エミュレータ

```bash
make
cargo run -- ./input.bin
```

## インクリメント専用の inc 命令

```text
ff 45 fc
```

```asm
inc dword [ebp-4]
```

- `ff` はオペコード
- `45 fc` はオペランド
    - `45` : `ebp + [8ビットの変位]` で番地を決めるということを示す
    - `fc` : 変位 (2の補数で、`-4` を表す)
- i386の32ビットモードにおいてはdword指定がデフォルトのため、機械語では　`dword`　に対応する記述がない

## メモリ

```c
void func(void) {
    int val;
    int *ptr = &val;
    *ptr = 41;
}
```

```asm
push ebp
mov ebp, esp
sub esp, 16

lea eax, [ebp-8]
mov [ebp-4], eax

mov eax, [ebp-4]
mov dword [eax], 41

leave  ; mov esp, ebp
       ; pop ebp
ret
```

## 初めてのエミュレータ

- レジスタ幅やメモリアドレス空間が32ビットである、一部の機能(セグメンテーションやページング等)を省略したリアルモードのエミュレータを作っていく
- エミュレータの構成部品はレジスタとメモリ
- BIOSもエミュレータの一部として実装する
