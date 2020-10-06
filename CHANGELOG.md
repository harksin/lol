# 0.3.0 (2020-10-6)

- Add apply_index to apply_message and install_snapshot so state machine can remember the last applied index to skip the previous messages after reboot.
- Implement copy snapshot. Now you can make a snapshot by either folding the log before the last applied index or returning snapshot from apply_message.
- Linking with RocksDB backend is now optional.

# 0.2.1 (2020-10-1)

- Implement RocksDB implementation of the RaftStorage.

# 0.2 (2020-9-26)

- Introduce RaftStorage abstraction.