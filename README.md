# issue_watcher_bot

This bot still WIP, don't use in production!

---

This matrix bot helps you to monitor for the new issues (in the near future, right now only manual method available) in the Redmine system.

### Features:
* Fetch issues
* Notify about new issues (planned)
* Add new issues (planned)
* Modify issues (planned)
* Add issues from zabbix automatically or with administrator allowance (planned)
* Database caching (planned)
* Flexible configuration (planned)

### Build (release):
```bash
git clone https://github.com/ElXreno/issue_watcher_bot
cd issue_watcher_bot
cargo build --release
```

If you want debug build, just remove `--release` from the last command.