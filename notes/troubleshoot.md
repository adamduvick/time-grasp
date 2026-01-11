Problem

```txt2026-01-05T14:32:40.143500Z  INFO applying new distribution
2026-01-05T14:32:40.144107Z  INFO ✅ success
2026-01-05T14:32:40.144155Z  INFO 📡 serving static assets at -> /
2026-01-05T14:32:40.144200Z  INFO 📡 server listening at:
2026-01-05T14:32:40.144205Z  INFO     🏠 http://127.0.0.1:1420/
2026-01-05T14:32:40.144207Z  INFO     🏠 http://localhost.time-grasp.com:1420
2026-01-05T14:32:40.144319Z  INFO     🏠 http://localhost.:1420/
2026-01-05T14:32:40.144379Z ERROR error from server task error=Address already in use (os error 48)
2026-01-05T14:32:40.144413Z ERROR Address already in use (os error 48)
```

solution

```bash
pgrep trunk # check to see what trunk processes are running
pkill trunk # run this only after confirming that you won't be killing something you don't intend to
```