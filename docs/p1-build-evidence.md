# Task #1 — Build & Validate Evidence

## Patch diff (busy_timeout.patch)

```diff
diff --git a/crates/harness-cli/src/infrastructure.rs b/crates/harness-cli/src/infrastructure.rs
index 424177e..e7d67f8 100644
--- a/crates/harness-cli/src/infrastructure.rs
+++ b/crates/harness-cli/src/infrastructure.rs
@@ -135,12 +135,14 @@ impl SqliteHarnessRepository {
 
         let connection = Connection::open(&self.db_path)?;
         connection.pragma_update(None, "foreign_keys", "ON")?;
+        connection.busy_timeout(std::time::Duration::from_secs(10))?;
         Ok(connection)
     }
 
     fn open_or_create(&self) -> Result<Connection> {
         let connection = Connection::open(&self.db_path)?;
         connection.pragma_update(None, "foreign_keys", "ON")?;
+        connection.busy_timeout(std::time::Duration::from_secs(10))?;
         Ok(connection)
     }
```

Patch file saved at: scratchpad/busy_timeout.patch

## Build log (Docker, rust:1-slim, linux/arm64)

```
docker run --rm --platform linux/arm64 \
  -v <clone>:/work -v <target>:/target \
  -w /work/crates/harness-cli \
  -e CARGO_TARGET_DIR=/target \
  rust:1-slim cargo build --release

[dependency compilation: proc-macro2, syn, quote, clap, rusqlite, libsqlite3-sys (bundled), thiserror ...]
   Compiling harness-cli v0.1.10 (/work/crates/harness-cli)
    Finished `release` profile [optimized] target(s) in 39.90s
```

Build: SUCCESS, exit 0

## Binary info

- Path: scratchpad/target/release/harness-cli
- Architecture: ELF 64-bit LSB pie executable, ARM aarch64 (linux/arm64)
- File: harness-cli: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=9bc4a72e6ac59ddcba16c39f69aec44299397e65, for GNU/Linux 3.7.0, not stripped
- Runtime: validated inside rust:1-slim (glibc-compatible)

## harness-cli init (fresh DB)

```
=== harness-cli init ===
Creating harness database at /testdb/harness.db
Schema applied.
EXIT_CODE_INIT=0
```

## harness-cli migrate (after init)

```
=== harness-cli migrate ===
Current schema version: 5
Already up to date.
EXIT_CODE_MIGRATE=0
```

## Summary

| Step | Result |
|------|--------|
| Patch applied (2 sites) | PASS |
| Docker build (rust:1-slim, arm64) | PASS (exit 0) |
| harness-cli init | PASS (exit 0) |
| harness-cli migrate | PASS (exit 0) |
