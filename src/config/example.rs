pub const EXAMPLE_FEED_TOML:&str=r#"# ==============================================
# 订阅源分组（按格式分类）
# group下name字段仅为名称，无解析作用
# ==============================================

# ---------- RSS 2.0 格式 ----------
[[group]]
name = "RSS 2.0"

[[group.feed]]
name = "OpenAI 官方博客"
url = "https://openai.com/news/rss.xml"
enabled = true

[[group.feed]]
name = "个人技术博客示例"
url = "http://yatagarasu.nekoweb.org/feed.xml"
enabled = true

# ---------- Atom 1.0 格式 ----------
[[group]]
name = "Atom 1.0"

[[group.feed]]
name = "阮一峰的网络日志"
url = "https://www.ruanyifeng.com/blog/atom.xml"
enabled = true

[[group.feed]]
name = "Mirror.xyz 示例"
url = "https://ens.mirror.xyz/feed/atom"
enabled = true

# ---------- JSON Feed 格式 ----------
[[group]]
name = "JSON Feed"

[[group.feed]]
name = "JSON Feed 官方示例"
url = "https://www.jsonfeed.org/feed.json"
enabled = true

[[group.feed]]
name = "Daring Fireball 博客"
url = "https://daringfireball.net/feed/json"
enabled = true
"#;

pub const EXAMPLE_SUMMARY_OUTPUT:&str=r#"{
  "conclusion": "今日文章数量xxx篇，其中{section}x篇。今日内容聚焦于网络安全领域的前沿研究，包括React和V8引擎的高危漏洞分析、安全领域大模型数据集的构建与应用、二进制安全工具的优化升级以及智能网联汽车的安全评估。科恩实验室在漏洞披露、AI赋能安全、工具开源等多个方向展现出显著成果。建议安全从业者重点关注React和V8漏洞修复，同时借鉴SecCorpus和CarVal等新技术提升自身防御体系。",
  "posts": [
    {
      "section": "AI安全趋势xxxx",
      "articles": [
        {
          "title": "BinaryAI二进制比对功能设计与实现｜大模型下函数的语义匹配",
          "url": "https://keenlab.tencent.com/zh/2023/07/13/2023-BinaryAI-update20230713-release/",
          "description": "BinaryAI平台新增了基于大模型BAI-2.0的二进制文件比对功能，采用启发式算法提高复杂场景下的准确率和召回率。通过三阶段流程（初始匹配、扩散匹配、剩余匹配）实现高效匹配，并通过测试数据表明效果优于传统工具。"
        },
        {
          "title": "腾讯安全科恩实验室推出首款免费在线SCA平台：BinaryAI",
          "url": "https://keenlab.tencent.com/zh/2021/08/11/2021-binaryai-public-release/",
          "description": ""
        }
      ]
    }
  ]
}"#;

pub const EXAMPLE_SUMMARY_TEMPLATE:&str=r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <title>安全情报纵览</title>

    <style>
        :root {
            --primary: #2563eb;
            --secondary: #22c55e;
            --danger: #ef4444;
            --warning: #f59e0b;
            --bg: #f3f4f6;
            --card: #ffffff;
            --text: #1f2937;
            --muted: #6b7280;
        }

        * {
            box-sizing: border-box;
        }

        body {
            margin: 0;
            padding: 40px 20px;
            font-family: "Segoe UI Variable", "Segoe UI", system-ui, -apple-system, BlinkMacSystemFont;
            background: linear-gradient(180deg, #eef2ff, #f8fafc);
            color: var(--text);
        }

        .container {
            max-width: 980px;
            margin: auto;
        }

        h1 {
            text-align: center;
            font-size: 2.4rem;
            margin-bottom: 50px;
        }

        /* 分类块 */
        .category {
            margin-bottom: 60px;
        }

        .category-title {
            font-size: 1.8rem;
            margin-bottom: 25px;
            display: flex;
            align-items: center;
            gap: 10px;
        }

        /* 时间线 */
        .timeline {
            border-left: 4px solid var(--primary);
            padding-left: 25px;
        }

        /* RSS 卡片 */
        .rss-card {
            background: var(--card);
            border-radius: 16px;
            padding: 25px;
            margin-bottom: 35px;
            box-shadow: 0 12px 30px rgba(0,0,0,0.08);
            position: relative;
        }

        .rss-card::before {
            content: "";
            position: absolute;
            left: -36px;
            top: 28px;
            width: 18px;
            height: 18px;
            background: var(--primary);
            border-radius: 50%;
            border: 4px solid #e0e7ff;
        }

        .rss-title {
            font-size: 1.3rem;
            font-weight: 700;
            margin-bottom: 12px;
        }

        /* 总结 */
        .conclusion {
            background: linear-gradient(135deg, #e0f2fe, #eef2ff);
            border-radius: 12px;
            padding: 18px;
            margin: 20px 0;
            white-space: pre-wrap;
        }

        /* 分区 */
        .section {
            margin-top: 30px;
        }

        .section-title {
            font-size: 1.1rem;
            font-weight: 700;
            margin-bottom: 12px;
            color: var(--primary);
        }

        /* 文章 */
        .article {
            padding: 12px 15px;
            margin-bottom: 12px;
            border-radius: 10px;
            background: #f9fafb;
            border: 1px solid #e5e7eb;
        }

        .article a {
            color: var(--primary);
            font-weight: 600;
            text-decoration: none;
        }

        .article a:hover {
            text-decoration: underline;
        }

        .article p {
            margin: 6px 0 0;
            color: var(--muted);
            font-size: 0.95rem;
        }

        /* 分析 */
        .analysis {
            margin-top: 18px;
            padding: 15px;
            border-radius: 12px;
            background: #fff7ed;
            border-left: 6px solid var(--warning);
            white-space: pre-wrap;
        }

        .empty {
            color: var(--muted);
            font-style: italic;
        }

        footer {
            text-align: center;
            margin-top: 80px;
            color: var(--muted);
        }
    </style>
</head>
<body>

<div class="container">
    <h1>🛡️ 安全情报 · 技术纵览</h1>

    {% if summaries %}
        <div class="timeline">
            {% for summary in summaries %}
                <div class="rss-card">
                    <div class="rss-title">
                        📰 RSS 摘要 #{{ loop.index }}
                    </div>

                    {% if summary.conclusion %}
                        <div class="conclusion">🧠 <strong>今日总结</strong><br>
{{ summary.conclusion }}
                        </div>
                    {% endif %}

                    {% if summary.posts %}
                        {% for post in summary.posts %}
                            <div class="section">
                                <div class="section-title">
                                    🔍 {{ post.section }}
                                </div>

                                {% if post.articles %}
                                    {% for article in post.articles %}
                                        <div class="article">
                                            🔗
                                            <a href="{{ article.url }}" target="_blank">
                                                {{ article.title }}
                                            </a>
                                            {% if article.description %}
                                                <p>{{ article.description }}</p>
                                            {% endif %}
                                        </div>
                                    {% endfor %}
                                {% endif %}

                                {% if post.analysis %}
                                    <div class="analysis">
                                        <strong>🧪 分析视角</strong><br>{{ post.analysis }}
                                    </div>
                                {% endif %}
                            </div>
                        {% endfor %}
                    {% endif %}
                </div>
            {% endfor %}
        </div>
    {% else %}
        <p class="empty">😴 今日暂无内容</p>
    {% endif %}

    <footer>
        ⚙️ Generated by Tera · Security never sleeps
    </footer>
</div>

</body>
</html>
"#;