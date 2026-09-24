# EverydayRSS

EverydayRSS 会抓取指定时间窗口内的 RSS / Atom / JSON Feed 文章，调用 OpenAI 兼容接口生成摘要，并输出一份 HTML 日报。摘要语言可以配置，默认使用中文。它可作为命令行工具单次运行，也可注册为系统每日任务。

## 安装

需要 Rust 1.85 或更高版本（项目使用 Rust 2024 edition）。

```bash
cargo install --path .
everydayrss init
```

首次直接运行 `everydayrss` 时，如果默认配置不存在，也会自动进入初始化向导。向导会配置 LLM、订阅目录、报告路径、日期窗口、邮件/企业微信推送，并可选择立即注册计划任务。

默认数据目录是 `~/.everydayrss`：

```text
~/.everydayrss/
├── config.toml
├── resources/feeds.example.toml
├── templates/example.template.html
├── output/daily.html
└── logs/
```

编辑 `resources/*.toml` 即可维护订阅源：

```toml
[[group]]
name = "技术"

[[group.feed]]
name = "OpenAI"
url = "https://openai.com/news/rss.xml"
enabled = true
```

## 使用

```bash
# 生成一次日报
everydayrss run

# 临时使用另一个订阅目录
everydayrss run --resource-dir ./my-feeds

# 使用另一个配置文件
everydayrss --config ./config.toml run

# 每天 08:30 运行
everydayrss schedule install --hour 8 --minute 30

# 或每 6 小时运行
everydayrss schedule install --every-hours 6

# 本次运行临时改为只取最近 2 小时
everydayrss run --lookback-hours 2

# 查看或移除系统任务
everydayrss schedule status
everydayrss schedule remove

# 推送最近生成的一份报告
everydayrss push

# 推送指定的报告文件
everydayrss push --file ./output/2026-09-24.html
```

macOS 使用当前用户的 `launchd`，Linux 使用用户级 `systemd`。建议先通过 `cargo install --path .` 安装稳定路径下的二进制，再注册计划任务。

## 日期判断

默认不配置 `lookback_hours`，日期窗口会自动采用已注册计划任务的周期：每日任务是 24 小时，间隔任务则使用对应的 N 小时。可以在配置中设置 `lookback_hours` 固定覆盖，也可以用 `run --lookback-hours N` 仅覆盖本次运行。尚未注册计划任务且没有自定义值时回退为 24 小时。

程序优先使用文章的 `published` 时间，没有时使用 `updated`；超过当前时间五分钟以上的未来文章会被忽略，以避免错误日期污染日报。

部分订阅源完全不提供文章日期。`include_undated = true`（默认）会保留这些文章；设为 `false` 可严格只处理带日期的内容。

所有相对路径都以 `config.toml` 所在目录为基准，因此计划任务与手动运行会得到一致结果。配置文件可能包含 API Key，在 Unix 系统上会以 `0600` 权限保存。

## 摘要语言

初始化向导中的 `Summary language` 可以设置摘要语言，默认值为 `Chinese`。也可以直接编辑配置：

```toml
[llm]
summary_language = "Chinese"
```

可改为 `English`、`Japanese`、`Simplified Chinese` 等语言名称。该值会写入 LLM 的 system prompt；JSON 字段名始终保持英文，原始文章标题和 URL 不会翻译。

## 报告推送

重新运行 `everydayrss init` 可以交互式配置推送。邮件使用 SMTP，将完整 HTML 作为邮件正文发送；企业微信使用内部群的自定义机器人 Webhook，上传并发送生成的 HTML 文件。

除了在 `everydayrss run` 时自动推送，也可以用 `everydayrss push` 单独推送已生成的报告：默认推送输出目录中最新的 HTML 文件，或用 `--file` 指定任意 HTML 文件。

也可以直接编辑配置：

```toml
[notifications.email]
enabled = true
smtp_host = "smtp.example.com"
smtp_port = 587
security = "starttls" # starttls / tls / none
username = "rss@example.com"
password = "SMTP_PASSWORD"
from = "rss@example.com"
to = ["you@example.com"]
subject = "EverydayRSS 日报"

[notifications.wecom]
enabled = true
webhook_url = "https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=YOUR_KEY"
```

推送失败不会删除已经生成的本地报告，命令会返回错误并给出报告路径。这里的“微信机器人”指企业微信内部群机器人，不是个人微信机器人。

## 开发

```bash
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```
