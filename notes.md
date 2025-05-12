# Notes

These are notes relating to my plans regarding this project. It's mostly
for me, but there's no harm in others seeing it. Good look working it
out.

## Weird CI Bug


    Running tests/integration_test.rs (target/debug/deps/integration_test-da4ed1895c58c777)
test test_cargo_run ... ok
test test_cargo_run_with_only_4 ... ok
test test_cargo_run_with_only_local ... ok
test test_cargo_run_with_only_6 ... FAILED
test test_cargo_run_with_only_wan ... ok
test test_cargo_run_with_only_wan_and_only_4 ... ok
test test_cargo_run_with_only_wan_and_only_6 ... FAILED
test test_cargo_run_with_reverse ... ok
test test_condition_not_only_6_and_not_only_local ... ok
failures:
---- test_cargo_run_with_only_6 stdout ----
Program output with --only-6: fe80::dc2c:5bff:fee9:cc93
fc00:f853:ccd:e793::1
fe80::a8b9:f6ff:fe90:40e2
::1
fe80::58f7:2eff:fea5:37b2
Program output with --only-6 --only-local: fc00:f853:ccd:e793::1
fe80::58f7:2eff:fea5:37b2
::1
fe80::a8b9:f6ff:fe90:40e2
fe80::dc2c:5bff:fee9:cc93
thread 'test_cargo_run_with_only_6' panicked at tests/integration_test.rs:14:5:
Program execution failed with args: ["--only-6", "--only-wan"]
stack backtrace:
0: rust_begin_unwind
at /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/panicking.rs:695:5
1: core::panicking::panic_fmt
at /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/panicking.rs:75:14
2: integration_test::run_with_args
at ./tests/integration_test.rs:14:5
3: integration_test::test_cargo_run_with_only_6
at ./tests/integration_test.rs:121:22
4: integration_test::test_cargo_run_with_only_6::{{closure}}
at ./tests/integration_test.rs:94:32
5: core::ops::function::FnOnce::call_once
at /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:5
6: core::ops::function::FnOnce::call_once
at /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
---- test_cargo_run_with_only_wan_and_only_6 stdout ----
thread 'test_cargo_run_with_only_wan_and_only_6' panicked at tests/integration_test.rs:14:5:
Program execution failed with args: ["--only-wan", "--only-6"]
stack backtrace:
0: rust_begin_unwind
at /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/std/src/panicking.rs:695:5
1: core::panicking::panic_fmt
at /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/panicking.rs:75:14
2: integration_test::run_with_args
at ./tests/integration_test.rs:14:5
3: integration_test::test_cargo_run_with_only_wan_and_only_6
at ./tests/integration_test.rs:204:18
4: integration_test::test_cargo_run_with_only_wan_and_only_6::{{closure}}
at ./tests/integration_test.rs:202:45
5: core::ops::function::FnOnce::call_once
at /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:5
6: core::ops::function::FnOnce::call_once
at /rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
failures:
test_cargo_run_with_only_6
test_cargo_run_with_only_wan_and_only_6
test result: FAILED. 7 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.49s
error: test failed, to rerun pass `--test integration_test`
error: Recipe `test` failed on line 7 with exit code 101


There is also a docker bake failure too that maybe is related?

#19 [docker builder  4/14] RUN xx-apk add --no-cache musl-dev zlib-dev zlib-static openssl-dev openssl-libs-static pkgconfig alpine-sdk
#19 0.214 + apk  --root / add --no-cache musl-dev zlib-dev zlib-static openssl-dev openssl-libs-static pkgconfig alpine-sdk
#19 0.251 fetch https://dl-cdn.alpinelinux.org/alpine/v3.21/main/x86_64/APKINDEX.tar.gz
#19 0.531 fetch https://dl-cdn.alpinelinux.org/alpine/v3.21/community/x86_64/APKINDEX.tar.gz
#19 1.455 (1/23) Installing libcap2 (2.71-r0)
#19 1.478 (2/23) Installing libcap-getcap (2.71-r0)
#19 1.498 (3/23) Installing fakeroot (1.36-r0)
#19 1.524 (4/23) Installing lzip (1.24.1-r1)
#19 1.547 (5/23) Installing openssl (3.3.3-r0)
#19 1.595 (6/23) Installing patch (2.7.6-r10)
#19 1.624 (7/23) Installing acl-libs (2.3.2-r1)
#19 1.644 (8/23) Installing tar (1.35-r2)
#19 1.684 (9/23) Installing abuild (3.14.1-r4)
#19 1.709 Executing abuild-3.14.1-r4.pre-install
#19 1.742 (10/23) Installing abuild-sudo (3.14.1-r4)
#19 1.765 (11/23) Installing libmagic (5.46-r2)
#19 1.855 (12/23) Installing file (5.46-r2)
#19 1.875 (13/23) Installing g++ (14.2.0-r4)
#19 2.674 (14/23) Installing make (4.4.1-r2)
#19 2.699 (15/23) Installing build-base (0.5-r3)
#19 2.699 (16/23) Installing libexpat (2.7.0-r0)
#19 2.721 (17/23) Installing pcre2 (10.43-r0)
#19 2.753 (18/23) Installing git (2.47.2-r0)
#19 2.915 (19/23) Installing git-init-template (2.47.2-r0)
#19 2.937 (20/23) Installing alpine-sdk (1.1-r0)
#19 2.937 (21/23) Installing openssl-libs-static (3.3.3-r0)
#19 3.304 (22/23) Installing zlib-dev (1.3.1-r2)
#19 3.324 (23/23) Installing zlib-static (1.3.1-r2)
#19 3.350 Executing busybox-1.37.0-r12.trigger
#19 3.357 OK: 559 MiB in 79 packages
#19 DONE 3.5s
#20 [docker builder  5/14] WORKDIR /app
#20 DONE 0.1s
#18 [bins-linux-amd64 builder  7/15] RUN cargo new --lib whatismyip
#18 0.529     Creating library `whatismyip` package
#18 ...
#21 [docker builder  6/14] RUN cargo new --lib whatismyip
#21 0.175     Creating library `whatismyip` package
#21 2.734 note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
#21 DONE 2.8s
#22 [docker builder  7/14] WORKDIR /app/whatismyip
#22 DONE 0.1s
#23 [docker builder  8/14] COPY Cargo.toml ./Cargo.toml
#23 DONE 0.1s
#24 [docker builder  9/14] COPY Cargo.lock ./Cargo.lock
#24 DONE 0.1s
#18 [bins-windows-arm64 builder  7/15] RUN cargo new --lib whatismyip
#18 3.359 note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
#18 DONE 3.4s
#25 [bins-windows-arm64 builder  8/15] WORKDIR /app/whatismyip
#25 DONE 0.2s
#26 [bins-darwin-arm64 builder  9/15] COPY Cargo.toml ./Cargo.toml
#26 DONE 0.2s
#27 [bins-darwin-arm64 builder 10/15] COPY Cargo.lock ./Cargo.lock
#27 DONE 0.1s
#28 [bins-linux-amd64 builder 11/15] RUN xx-cargo build --release --target-dir ./build
#28 1.249 error: failed to parse manifest at `/app/whatismyip/Cargo.toml`
#28 1.249
#28 1.249 Caused by:
#28 1.249   can't find `ip_benchmarks` bench at `benches/ip_benchmarks.rs` or `benches/ip_benchmarks/main.rs`. Please specify bench.path if you want to use a non-default path.
#28 ERROR: process "/bin/sh -c xx-cargo build --release --target-dir ./build" did not complete successfully: exit code: 101
#29 [docker builder 10/14] RUN xx-cargo build --release --target-dir ./build
#29 1.463 error: failed to parse manifest at `/app/whatismyip/Cargo.toml`
#29 1.463
#29 1.463 Caused by:
#29 1.463   can't find `ip_benchmarks` bench at `benches/ip_benchmarks.rs` or `benches/ip_benchmarks/main.rs`. Please specify bench.path if you want to use a non-default path.
#29 ERROR: process "/bin/sh -c xx-cargo build --release --target-dir ./build" did not complete successfully: exit code: 101
------
> [docker builder 10/14] RUN xx-cargo build --release --target-dir ./build:
1.463 error: failed to parse manifest at `/app/whatismyip/Cargo.toml`
1.463
1.463 Caused by:
1.463   can't find `ip_benchmarks` bench at `benches/ip_benchmarks.rs` or `benches/ip_benchmarks/main.rs`. Please specify bench.path if you want to use a non-default path.
------
------
> [bins-alpine-arm64 builder 11/15] RUN xx-cargo build --release --target-dir ./build:
1.249 error: failed to parse manifest at `/app/whatismyip/Cargo.toml`
1.249
1.249 Caused by:
1.249   can't find `ip_benchmarks` bench at `benches/ip_benchmarks.rs` or `benches/ip_benchmarks/main.rs`. Please specify bench.path if you want to use a non-default path.
------
WARNING: No output specified for docker target(s) with docker-container driver. Build result will only remain in the build cache. To push result image into registry use --push or to load image into docker use --load
Dockerfile.bins:24
--------------------
22 |     COPY Cargo.toml ./Cargo.toml
23 |     COPY Cargo.lock ./Cargo.lock
24 | >>> RUN xx-cargo build --release --target-dir ./build
25 |     COPY . ./
26 |     RUN xx-cargo build --release --target-dir ./build && \
--------------------
ERROR: target bins-darwin-arm64: failed to solve: process "/bin/sh -c xx-cargo build --release --target-dir ./build" did not complete successfully: exit code: 101
Build references
Check build summary support
::error::buildx bake failed with: ERROR: target bins-darwin-arm64: failed to solve: process "/bin/sh -c xx-cargo build --release --target-dir ./build" did not complete successfully: exit code: 101#19 [docker builder  4/14] RUN xx-apk add --no-cache musl-dev zlib-dev zlib-static openssl-dev openssl-libs-static pkgconfig alpine-sdk
#19 0.214 + apk  --root / add --no-cache musl-dev zlib-dev zlib-static openssl-dev openssl-libs-static pkgconfig alpine-sdk
#19 0.251 fetch https://dl-cdn.alpinelinux.org/alpine/v3.21/main/x86_64/APKINDEX.tar.gz
#19 0.531 fetch https://dl-cdn.alpinelinux.org/alpine/v3.21/community/x86_64/APKINDEX.tar.gz
#19 1.455 (1/23) Installing libcap2 (2.71-r0)
#19 1.478 (2/23) Installing libcap-getcap (2.71-r0)
#19 1.498 (3/23) Installing fakeroot (1.36-r0)
#19 1.524 (4/23) Installing lzip (1.24.1-r1)
#19 1.547 (5/23) Installing openssl (3.3.3-r0)
#19 1.595 (6/23) Installing patch (2.7.6-r10)
#19 1.624 (7/23) Installing acl-libs (2.3.2-r1)
#19 1.644 (8/23) Installing tar (1.35-r2)
#19 1.684 (9/23) Installing abuild (3.14.1-r4)
#19 1.709 Executing abuild-3.14.1-r4.pre-install
#19 1.742 (10/23) Installing abuild-sudo (3.14.1-r4)
#19 1.765 (11/23) Installing libmagic (5.46-r2)
#19 1.855 (12/23) Installing file (5.46-r2)
#19 1.875 (13/23) Installing g++ (14.2.0-r4)
#19 2.674 (14/23) Installing make (4.4.1-r2)
#19 2.699 (15/23) Installing build-base (0.5-r3)
#19 2.699 (16/23) Installing libexpat (2.7.0-r0)
#19 2.721 (17/23) Installing pcre2 (10.43-r0)
#19 2.753 (18/23) Installing git (2.47.2-r0)
#19 2.915 (19/23) Installing git-init-template (2.47.2-r0)
#19 2.937 (20/23) Installing alpine-sdk (1.1-r0)
#19 2.937 (21/23) Installing openssl-libs-static (3.3.3-r0)
#19 3.304 (22/23) Installing zlib-dev (1.3.1-r2)
#19 3.324 (23/23) Installing zlib-static (1.3.1-r2)
#19 3.350 Executing busybox-1.37.0-r12.trigger
#19 3.357 OK: 559 MiB in 79 packages
#19 DONE 3.5s
#20 [docker builder  5/14] WORKDIR /app
#20 DONE 0.1s
#18 [bins-linux-amd64 builder  7/15] RUN cargo new --lib whatismyip
#18 0.529     Creating library `whatismyip` package
#18 ...
#21 [docker builder  6/14] RUN cargo new --lib whatismyip
#21 0.175     Creating library `whatismyip` package
#21 2.734 note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
#21 DONE 2.8s
#22 [docker builder  7/14] WORKDIR /app/whatismyip
#22 DONE 0.1s
#23 [docker builder  8/14] COPY Cargo.toml ./Cargo.toml
#23 DONE 0.1s
#24 [docker builder  9/14] COPY Cargo.lock ./Cargo.lock
#24 DONE 0.1s
#18 [bins-windows-arm64 builder  7/15] RUN cargo new --lib whatismyip
#18 3.359 note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
#18 DONE 3.4s
#25 [bins-windows-arm64 builder  8/15] WORKDIR /app/whatismyip
#25 DONE 0.2s
#26 [bins-darwin-arm64 builder  9/15] COPY Cargo.toml ./Cargo.toml
#26 DONE 0.2s
#27 [bins-darwin-arm64 builder 10/15] COPY Cargo.lock ./Cargo.lock
#27 DONE 0.1s
#28 [bins-linux-amd64 builder 11/15] RUN xx-cargo build --release --target-dir ./build
#28 1.249 error: failed to parse manifest at `/app/whatismyip/Cargo.toml`
#28 1.249
#28 1.249 Caused by:
#28 1.249   can't find `ip_benchmarks` bench at `benches/ip_benchmarks.rs` or `benches/ip_benchmarks/main.rs`. Please specify bench.path if you want to use a non-default path.
#28 ERROR: process "/bin/sh -c xx-cargo build --release --target-dir ./build" did not complete successfully: exit code: 101
#29 [docker builder 10/14] RUN xx-cargo build --release --target-dir ./build
#29 1.463 error: failed to parse manifest at `/app/whatismyip/Cargo.toml`
#29 1.463
#29 1.463 Caused by:
#29 1.463   can't find `ip_benchmarks` bench at `benches/ip_benchmarks.rs` or `benches/ip_benchmarks/main.rs`. Please specify bench.path if you want to use a non-default path.
#29 ERROR: process "/bin/sh -c xx-cargo build --release --target-dir ./build" did not complete successfully: exit code: 101
------
> [docker builder 10/14] RUN xx-cargo build --release --target-dir ./build:
1.463 error: failed to parse manifest at `/app/whatismyip/Cargo.toml`
1.463
1.463 Caused by:
1.463   can't find `ip_benchmarks` bench at `benches/ip_benchmarks.rs` or `benches/ip_benchmarks/main.rs`. Please specify bench.path if you want to use a non-default path.
------
------
> [bins-alpine-arm64 builder 11/15] RUN xx-cargo build --release --target-dir ./build:
1.249 error: failed to parse manifest at `/app/whatismyip/Cargo.toml`
1.249
1.249 Caused by:
1.249   can't find `ip_benchmarks` bench at `benches/ip_benchmarks.rs` or `benches/ip_benchmarks/main.rs`. Please specify bench.path if you want to use a non-default path.
------
WARNING: No output specified for docker target(s) with docker-container driver. Build result will only remain in the build cache. To push result image into registry use --push or to load image into docker use --load
Dockerfile.bins:24
--------------------
22 |     COPY Cargo.toml ./Cargo.toml
23 |     COPY Cargo.lock ./Cargo.lock
24 | >>> RUN xx-cargo build --release --target-dir ./build
25 |     COPY . ./
26 |     RUN xx-cargo build --release --target-dir ./build && \
--------------------
ERROR: target bins-darwin-arm64: failed to solve: process "/bin/sh -c xx-cargo build --release --target-dir ./build" did not complete successfully: exit code: 101
Build references
Check build summary support
::error::buildx bake failed with: ERROR: target bins-darwin-arm64: failed to solve: process "/bin/sh -c xx-cargo build --release --target-dir ./build" did not complete successfully: exit code: 101
