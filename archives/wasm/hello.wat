(module
  (import
    "wasi_unstable"
    "fd_write"
    (func $__wasi_fd_write (param i32 i32 i32 i32) (result i32)))
  (memory 1)
  (export "memory" (memory 0))
  (data (i32.const 8) "Hello, world!\n")
  (func $_start (export "_start")
    ;; Creating a new io vector within linear memory
    ;; iov.iov_base - This is a pointer to the start of the string
    (i32.store (i32.const 0) (i32.const 8))
    ;; iov.iov_len  - The length of the string
    (i32.store (i32.const 4) (i32.const 14))

    (call $__wasi_fd_write
      (i32.const 1)  ;; file_descriptor - 1 for stdout
      (i32.const 0)  ;; *iovs - The pointer to the iov array, which is stored at memory location 0
      (i32.const 1)  ;; num_iovs - We're printing 1 string stored in an iov - so one.
      (i32.const 8)) ;; nwritten - A place in memory to store the number of bytes written
    drop))
