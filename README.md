# aleo-ffi-bindings
C external interface using lib_std as a depedendency.

currently supports standard targets, branch `with_no_std` meant to support target: `thumbv7m-none-eabi`,
but currently blocked due to heavy dependance by sdk on `lib_std`.

The filesize of the optimized static library archive is 4.4M.

Generate C Headers with the following command: `cargo test --features c-headers generate_headers`

```
arosboro@Andrews-iMac aleo-ffi-bindings % ls -alsh target/release
total 8952
   0 drwxr-xr-x   10 arosboro  staff   320B Dec 27 13:05 .
   0 drwxr-xr-x@   7 arosboro  staff   224B Dec 27 13:04 ..
   0 -rw-r--r--    1 arosboro  staff     0B Dec 27 13:04 .cargo-lock
   0 drwxr-xr-x  278 arosboro  staff   8.7K Dec 27 13:04 .fingerprint
   0 drwxr-xr-x   89 arosboro  staff   2.8K Dec 27 13:04 build
   0 drwxr-xr-x  554 arosboro  staff    17K Dec 27 13:05 deps
   0 drwxr-xr-x    2 arosboro  staff    64B Dec 27 13:04 examples
   0 drwxr-xr-x    2 arosboro  staff    64B Dec 27 13:04 incremental
8944 -rw-r--r--    1 arosboro  staff   4.4M Dec 27 13:05 libaleo.a
   8 -rw-r--r--    1 arosboro  staff   533B Dec 27 13:05 libaleo.d
```

The following tests were carried over from aleo-wasm:

```
running 10 tests
test account::private_key::tests::test_sanity_check ... ok
test account::private_key::tests::test_from_seed_unchecked ... ok
test account::private_key::tests::test_new ... ok
test account::address::tests::test_from_private_key ... ok
test account::view_key::tests::test_decrypt_fails ... ok
test account::view_key::tests::test_decrypt_success ... ok
test account::view_key::tests::test_from_private_key ... ok
test account::private_key::tests::test_to_address ... ok
test account::private_key::tests::test_signature ... ok
test account::signature::tests::test_sign_and_verify ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.97s
```

C Makefiles and C Tests need work.
