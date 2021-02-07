@hello = constant [14 x i8] c"Hello, world!\00"

define i32 @main() {
    ; getelementptr <指している値の型>, <指している値の型>* <ポインタ値>, <インデックス1>, <インデックス2>, …
    %helloptr = getelementptr [14 x i8], [14 x i8]* @hello, i32 0, i32 0
    call i32 @puts(i8* %helloptr)
    ret i32 0
}

declare i32 @puts(i8*)
