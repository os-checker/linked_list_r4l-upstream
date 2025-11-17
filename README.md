# Linked list in Rust for Linux

## About this project

I'd like to see how it is possible to run Miri on linked list code, so I copied all relevent
code out to this repo, in cargo project style.

`cargo build` works, but `cargo test` emits linking error.

```txt
error: linking with `cc` failed: exit status: 1
  |
  = note:  "cc" "/tmp/rustcVCcbkX/symbols.o" "<65 object files omitted>" "-Wl,--as-needed" "-Wl,-Bstatic" "/home/gh-zjp-CN/bugs-found/fix/link
ed_list_r4l-upstream/target/debug/deps/libbindings-7f4d313fd23c439b.rlib" "<sysroot>/lib/rustlib/aarch64-unknown-linux-gnu/lib/{libtest-*,libg
etopts-*,librustc_std_workspace_std-*}.rlib" "/home/gh-zjp-CN/bugs-found/fix/linked_list_r4l-upstream/target/debug/deps/{libffi-f874a28251d3b0
f5,libpin_init-13fe690f7aeb02bf}.rlib" "<sysroot>/lib/rustlib/aarch64-unknown-linux-gnu/lib/{libstd-*,libpanic_unwind-*,libobject-*,libmemchr-
*,libaddr2line-*,libgimli-*,libcfg_if-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libminiz_oxide-*,lib
adler2-*,libunwind-*,liblibc-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lu
til" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-L" "/tmp/rustcVCcbkX/raw-dylibs" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "<sysroot>/lib/ru
stlib/aarch64-unknown-linux-gnu/lib" "-o" "/home/gh-zjp-CN/bugs-found/fix/linked_list_r4l-upstream/target/debug/deps/linked_list_r4l_upstream-
c771e58127b2dd99" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: /usr/bin/ld: /home/gh-zjp-CN/bugs-found/fix/linked_list_r4l-upstream/target/debug/deps/linked_list_r4l_upstream-c771e58127b2dd99.12h
wma543ylqxalu13pb5xbvw.1sk2xak.rcgu.o:(.data.rel.ro..Lanon.91ffed9f72cf8861b64a56abddfe1591.0+0x0): undefined reference to `rust_helper_kreall
oc_node_align'
          /usr/bin/ld: /home/gh-zjp-CN/bugs-found/fix/linked_list_r4l-upstream/target/debug/deps/linked_list_r4l_upstream-c771e58127b2dd99.12h
wma543ylqxalu13pb5xbvw.1sk2xak.rcgu.o:(.data.rel.ro..Lanon.91ffed9f72cf8861b64a56abddfe1591.1+0x0): undefined reference to `rust_helper_vreall
oc_node_align'
          ...
o82kjqyejvks3w4ila1nsv.1sk2xak.rcgu.o: in function `<i32 as linked_list_r4l_upstream::sync::atomic::internal::AtomicBasicOps>::atomic_read':
          /home/gh-zjp-CN/bugs-found/fix/linked_list_r4l-upstream/src/sync/atomic/internal.rs:123:(.text._ZN24linked_list_r4l_upstream4sync6at
omic9predefine5tests23atomic_arithmetic_tests28_$u7b$$u7b$closure$u7d$$u7d$17hfba01a70bb3ba787E+0x2e4): undefined reference to `rust_helper_at
omic_read'
          collect2: error: ld returned 1 exit status
          
  = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/ref
erence/build-scripts.html#rustc-link-lib)
```

## Linux Preparation

```bash
git clone https://github.com/Rust-for-Linux/linux

cd linux
# See https://kernel.org/pub/tools/llvm/rust/ for matched toolchain
wget https://mirrors.edge.kernel.org/pub/tools/llvm/rust/files/llvm-21.1.3-rust-1.91.0-aarch64.tar.gz

llvm_prefix=$PWD/llvm-21.1.3-rust-1.91.0-aarch64
PATH=$llvm_prefix/bin:$PATH
export LIBCLANG_PATH=$llvm_prefix/lib/libclang.so
```

```
# Add kernel/configs/rfl-for-rust-ci.config with following configs:
CONFIG_RUST=y

CONFIG_SAMPLES=y
CONFIG_SAMPLES_RUST=y

CONFIG_SAMPLE_RUST_MINIMAL=y
CONFIG_SAMPLE_RUST_PRINT=y

CONFIG_RUST_PHYLIB_ABSTRACTIONS=y

CONFIG_AX88796B_PHY=y
CONFIG_AX88796B_RUST_PHY=y

CONFIG_KUNIT=y
CONFIG_RUST_KERNEL_DOCTESTS=n
```

```shell
# Preparation.
make LLVM=1 rustavailable defconfig rfl-for-rust-ci.config

# Generate rust-project.json for RA to work.
make LLVM=1 rust-analyzer

# Compile a sample.
make LLVM=1 samples/rust/rust_minimal.o
```
