pub const EXAMPLE_SUMMARY_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">

    <title>每日资讯纵览</title>

    <style>
        :root {
            --primary: #4f46e5;
            --primary-light: #eef2ff;
            --primary-dark: #3730a3;

            --accent: #0891b2;
            --accent-light: #ecfeff;

            --warning: #f59e0b;
            --warning-light: #fffbeb;

            --success: #16a34a;

            --page-start: #f8fafc;
            --page-end: #eef2ff;
            --card: rgba(255, 255, 255, 0.94);
            --article-bg: #f8fafc;

            --text: #172033;
            --muted: #64748b;
            --border: #e2e8f0;

            --shadow-sm: 0 4px 12px rgba(15, 23, 42, 0.06);
            --shadow-md: 0 16px 40px rgba(15, 23, 42, 0.10);

            --radius-sm: 10px;
            --radius-md: 16px;
            --radius-lg: 22px;
        }

        * {
            box-sizing: border-box;
        }

        html {
            scroll-behavior: smooth;
        }

        body {
            margin: 0;
            min-height: 100vh;
            padding: 48px 20px;
            font-family:
                "Segoe UI Variable",
                "Segoe UI",
                "PingFang SC",
                "Microsoft YaHei",
                system-ui,
                -apple-system,
                BlinkMacSystemFont,
                sans-serif;
            line-height: 1.7;
            color: var(--text);
            background:
                radial-gradient(
                    circle at top left,
                    rgba(79, 70, 229, 0.12),
                    transparent 32%
                ),
                radial-gradient(
                    circle at top right,
                    rgba(8, 145, 178, 0.10),
                    transparent 28%
                ),
                linear-gradient(
                    180deg,
                    var(--page-start),
                    var(--page-end)
                );
        }

        a {
            color: inherit;
        }

        .container {
            width: 100%;
            max-width: 1080px;
            margin: 0 auto;
        }

        /* 页面头部 */
        .page-header {
            margin-bottom: 50px;
            text-align: center;
        }

        .page-badge {
            display: inline-flex;
            align-items: center;
            gap: 8px;
            margin-bottom: 16px;
            padding: 7px 14px;
            color: var(--primary-dark);
            font-size: 0.88rem;
            font-weight: 700;
            letter-spacing: 0.04em;
            background: rgba(238, 242, 255, 0.86);
            border: 1px solid #c7d2fe;
            border-radius: 999px;
        }

        .page-title {
            margin: 0;
            font-size: clamp(2rem, 6vw, 3.25rem);
            line-height: 1.2;
            letter-spacing: -0.04em;
            color: #111827;
        }

        .page-subtitle {
            max-width: 680px;
            margin: 16px auto 0;
            color: var(--muted);
            font-size: 1rem;
        }

        /* 分组 */
        .group {
            margin-bottom: 64px;
        }

        .group-header {
            display: flex;
            align-items: center;
            gap: 14px;
            margin-bottom: 24px;
        }

        .group-icon {
            display: inline-flex;
            flex: 0 0 auto;
            align-items: center;
            justify-content: center;
            width: 44px;
            height: 44px;
            color: white;
            font-size: 1.15rem;
            background: linear-gradient(
                135deg,
                var(--primary),
                var(--accent)
            );
            border-radius: 14px;
            box-shadow: var(--shadow-sm);
        }

        .group-heading {
            min-width: 0;
        }

        .group-label {
            margin-bottom: 2px;
            color: var(--muted);
            font-size: 0.76rem;
            font-weight: 700;
            letter-spacing: 0.12em;
            text-transform: uppercase;
        }

        .group-title {
            margin: 0;
            overflow-wrap: anywhere;
            font-size: 1.55rem;
            line-height: 1.3;
        }

        /* 时间线 */
        .timeline {
            position: relative;
            margin-left: 21px;
            padding-left: 34px;
            border-left: 3px solid #c7d2fe;
        }

        /* 摘要卡片 */
        .summary-card {
            position: relative;
            margin-bottom: 34px;
            padding: 28px;
            overflow: hidden;
            background: var(--card);
            border: 1px solid rgba(226, 232, 240, 0.9);
            border-radius: var(--radius-lg);
            box-shadow: var(--shadow-md);
            backdrop-filter: blur(12px);
        }

        .summary-card:last-child {
            margin-bottom: 0;
        }

        .summary-card::before {
            position: absolute;
            top: 30px;
            left: -45px;
            width: 17px;
            height: 17px;
            content: "";
            background: linear-gradient(
                135deg,
                var(--primary),
                var(--accent)
            );
            border: 5px solid #eef2ff;
            border-radius: 50%;
            box-shadow: 0 0 0 2px #c7d2fe;
        }

        .summary-card::after {
            position: absolute;
            top: 0;
            right: 0;
            width: 180px;
            height: 180px;
            pointer-events: none;
            content: "";
            background: radial-gradient(
                circle at top right,
                rgba(79, 70, 229, 0.08),
                transparent 68%
            );
        }

        /* 综合摘要 */
        .conclusion {
            position: relative;
            z-index: 1;
            margin-bottom: 28px;
            padding: 20px 22px;
            background: linear-gradient(
                135deg,
                var(--primary-light),
                var(--accent-light)
            );
            border: 1px solid #c7d2fe;
            border-radius: var(--radius-md);
        }

        .conclusion-title {
            display: flex;
            align-items: center;
            gap: 8px;
            margin-bottom: 9px;
            color: var(--primary-dark);
            font-size: 0.92rem;
            font-weight: 800;
            letter-spacing: 0.03em;
        }

        .conclusion-content {
            color: #334155;
            white-space: pre-wrap;
            overflow-wrap: anywhere;
        }

        /* 内容分区 */
        .section {
            position: relative;
            z-index: 1;
            margin-top: 30px;
        }

        .section:first-child {
            margin-top: 0;
        }

        .section-header {
            display: flex;
            align-items: center;
            gap: 10px;
            margin-bottom: 14px;
        }

        .section-marker {
            width: 8px;
            height: 22px;
            flex: 0 0 auto;
            background: linear-gradient(
                180deg,
                var(--primary),
                var(--accent)
            );
            border-radius: 99px;
        }

        .section-title {
            margin: 0;
            font-size: 1.12rem;
            line-height: 1.4;
            color: #273449;
        }

        /* 文章 */
        .article-list {
            display: grid;
            gap: 12px;
        }

        .article {
            padding: 16px 18px;
            background: var(--article-bg);
            border: 1px solid var(--border);
            border-radius: var(--radius-sm);
            transition:
                transform 160ms ease,
                box-shadow 160ms ease,
                border-color 160ms ease,
                background-color 160ms ease;
        }

        .article:hover {
            transform: translateY(-2px);
            background: #ffffff;
            border-color: #a5b4fc;
            box-shadow: var(--shadow-sm);
        }

        .article-title {
            display: flex;
            align-items: flex-start;
            gap: 9px;
        }

        .article-link-icon {
            flex: 0 0 auto;
            margin-top: 2px;
            color: var(--primary);
        }

        .article a {
            color: var(--primary-dark);
            font-weight: 700;
            line-height: 1.5;
            text-decoration: none;
            overflow-wrap: anywhere;
        }

        .article a:hover {
            color: var(--primary);
            text-decoration: underline;
            text-underline-offset: 3px;
        }

        .article-description {
            margin: 8px 0 0 27px;
            color: var(--muted);
            font-size: 0.94rem;
            white-space: pre-wrap;
            overflow-wrap: anywhere;
        }

        /* 分析内容 */
        .analysis {
            margin-top: 18px;
            padding: 17px 19px;
            background: var(--warning-light);
            border: 1px solid #fde68a;
            border-left: 5px solid var(--warning);
            border-radius: var(--radius-sm);
        }

        .analysis-title {
            display: flex;
            align-items: center;
            gap: 8px;
            margin-bottom: 7px;
            color: #92400e;
            font-size: 0.9rem;
            font-weight: 800;
        }

        .analysis-content {
            color: #78350f;
            font-size: 0.95rem;
            white-space: pre-wrap;
            overflow-wrap: anywhere;
        }

        /* 空状态 */
        .empty-state {
            padding: 34px 24px;
            text-align: center;
            color: var(--muted);
            background: rgba(255, 255, 255, 0.72);
            border: 1px dashed #cbd5e1;
            border-radius: var(--radius-md);
        }

        .empty-icon {
            display: block;
            margin-bottom: 10px;
            font-size: 2rem;
            font-style: normal;
        }

        .empty-title {
            margin: 0 0 4px;
            color: #475569;
            font-size: 1rem;
            font-weight: 700;
        }

        .empty-description {
            margin: 0;
            font-size: 0.9rem;
        }

        /* 页脚 */
        footer {
            margin-top: 80px;
            padding-top: 24px;
            text-align: center;
            color: var(--muted);
            font-size: 0.86rem;
            border-top: 1px solid rgba(203, 213, 225, 0.8);
        }

        /* 移动端 */
        @media (max-width: 680px) {
            body {
                padding: 30px 14px;
            }

            .page-header {
                margin-bottom: 36px;
            }

            .group {
                margin-bottom: 46px;
            }

            .group-header {
                align-items: flex-start;
            }

            .timeline {
                margin-left: 10px;
                padding-left: 20px;
            }

            .summary-card {
                padding: 20px;
                border-radius: 16px;
            }

            .summary-card::before {
                left: -31px;
                width: 14px;
                height: 14px;
                border-width: 4px;
            }

            .conclusion {
                padding: 17px;
            }

            .article {
                padding: 14px;
            }

            .article-description {
                margin-left: 0;
            }
        }

        /* 打印与导出 PDF */
        @media print {
            body {
                padding: 0;
                background: #ffffff;
            }

            .container {
                max-width: none;
            }

            .summary-card {
                break-inside: avoid;
                box-shadow: none;
                backdrop-filter: none;
            }

            .article {
                break-inside: avoid;
            }

            .article:hover {
                transform: none;
                box-shadow: none;
            }

            a {
                color: #000000;
            }
        }
    </style>
</head>

<body>
<main class="container">
    <header class="page-header">
        <div class="page-badge">
            ✦ RSS DIGEST
        </div>

        <h1 class="page-title">每日资讯纵览</h1>

        <p class="page-subtitle">
            聚合不同信息源的重要内容，快速了解今日动态、核心观点与趋势分析。
        </p>
    </header>

    {% if groups | length == 0 %}
    <section class="empty-state">
        <i class="empty-icon">📭</i>
        <p class="empty-title">暂无订阅分组</p>
        <p class="empty-description">
            当前没有可以展示的资讯内容。
        </p>
    </section>
    {% else %}

        {% for name, summaries in groups %}
        <section class="group">
            <header class="group-header">
                <div class="group-icon">📰</div>

                <div class="group-heading">
                    <div class="group-label">Information Group</div>

                    <h2 class="group-title">
                        {{ name }}
                    </h2>
                </div>
            </header>

            {% if summaries | length == 0 %}
            <div class="empty-state">
                <i class="empty-icon">☕</i>
                <p class="empty-title">该分组今日暂无更新</p>
                <p class="empty-description">
                    暂时没有获取到可以展示的摘要内容。
                </p>
            </div>
            {% else %}

            <div class="timeline">
                {% for summary in summaries %}
                <article class="summary-card">

                    {% if summary.conclusion | length > 0 %}
                    <section class="conclusion">
                        <div class="conclusion-title">
                            <span>✨</span>
                            <span>综合摘要</span>
                        </div>

                        <div class="conclusion-content">
                            {{ summary.conclusion }}
                        </div>
                    </section>
                    {% endif %}

                    {% if summary.posts | length == 0 %}
                    <div class="empty-state">
                        <i class="empty-icon">🗂️</i>
                        <p class="empty-title">暂无详细内容</p>
                        <p class="empty-description">
                            该摘要目前没有关联的文章分类。
                        </p>
                    </div>
                    {% else %}

                        {% for post in summary.posts %}
                        <section class="section">
                            <header class="section-header">
                                <span class="section-marker"></span>

                                <h3 class="section-title">
                                    {{ post.section }}
                                </h3>
                            </header>

                            {% if post.articles | length == 0 %}
                            <div class="empty-state">
                                <i class="empty-icon">📄</i>
                                <p class="empty-title">该分类暂无文章</p>
                            </div>
                            {% else %}

                            <div class="article-list">
                                {% for article in post.articles %}
                                <article class="article">
                                    <div class="article-title">
                                        <span class="article-link-icon">↗</span>

                                        <a
                                            href="{{ article.url }}"
                                            target="_blank"
                                            rel="noopener noreferrer"
                                        >
                                            {{ article.title }}
                                        </a>
                                    </div>

                                    {% if article.description | length > 0 %}
                                    <p class="article-description">
                                        {{ article.description }}
                                    </p>
                                    {% endif %}
                                </article>
                                {% endfor %}
                            </div>
                            {% endif %}

                            {% if post.analysis | length > 0 %}
                            <aside class="analysis">
                                <div class="analysis-title">
                                    <span>💡</span>
                                    <span>趋势与观点</span>
                                </div>

                                <div class="analysis-content">
                                    {{ post.analysis }}
                                </div>
                            </aside>
                            {% endif %}
                        </section>
                        {% endfor %}

                    {% endif %}
                </article>
                {% endfor %}
            </div>
            {% endif %}
        </section>
        {% endfor %}

    {% endif %}

    <footer>
        聚合信息仅供参考 · Generated with Rust and Tera
    </footer>
</main>
</body>
</html>
"#;


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
  "conclusion": "今日OpenAI官方博客更新文章约500篇，覆盖模型发布、Codex与智能体、企业落地、安全治理、医疗科学、教育、基础设施、政策合作、消费者产品及公司治理等十大方向。核心主线是GPT-6 Astra、GPT-5.6 等前沿模型迭代，以及Codex从编码工具升级为智能体平台。安全方面，OpenAI达到网络安全Critical级别并强化红队与威胁情报。基础设施上，Stargate、自研芯片与多云合作加速算力扩张。医疗、教育、政府合作持续深化。 分析师建议企业优先评估最新模型与Codex在工作流中的ROI，同时建立AI安全治理与合规框架，以应对能力快速提升带来的风险与机遇。",
  "posts": [
    {
      "section": "基础设施、算力与合作伙伴",
      "articles": [
        {
          "title": "Announcing The Stargate Project",
          "url": "https://openai.com/index/announcing-the-stargate-project",
          "description": "OpenAI宣布Stargate项目，计划投资5000亿美元建设AI基础设施，首批数据中心在美国展开。该项目联合Oracle、SoftBank等伙伴，旨在扩展算力以支持AGI，创造就业并巩固美国AI领导地位。"
        },
        {
          "title": "OpenAI and Broadcom unveil LLM-optimized inference chip",
          "url": "https://openai.com/index/openai-broadcom-jalapeno-inference-chip",
          "description": "OpenAI与博通推出Jalapeño，一款专为LLM推理优化的定制AI芯片，旨在提升性能、能效和规模。该芯片针对现代模型的高吞吐与低延迟需求，有望降低推理成本，并减少对通用GPU的依赖。"
        },
        {
          "title": "AWS and OpenAI announce multi-year strategic partnership",
          "url": "https://openai.com/index/aws-and-openai-partnership",
          "description": "OpenAI与AWS达成多年期380亿美元战略合作，AWS将为OpenAI提供世界级基础设施和算力，支持下一代模型训练。该合作扩展OpenAI的云战略，也反映AI算力需求正推动超大规模云厂商深度绑定。"
        }
      ],
      "analysis": "分析师认为，算力与基础设施已成为AI竞争的核心瓶颈。OpenAI通过Stargate、自研芯片、多云合作和数据库优化，构建从训练到推理的全栈能力。企业应关注AI算力成本、数据驻留与云战略，提前规划可 扩展的AI基础设施。"
    },
    {
      "section": "政策、政府与国际合作",
      "articles": [
        {
          "title": "Expanding AI access and cyber defense for federal, state, local, and tribal governments",
          "url": "https://openai.com/index/expanding-ai-access-us-government",
          "description": "OpenAI与GSA合作，向符合条件的美国联邦、州、地方和部落政府提供零许可费、50%用量折扣及扩展网络防御支持。该举措旨在加速公共部门AI采用，同时强化政府网络安全能力。"
        },
        {
          "title": "Industrial policy for the Intelligence Age",
          "url": "https://openai.com/index/industrial-policy-for-the-intelligence-age",
          "description": "OpenAI提出面向智能时代的产业政策建议，聚焦扩大机会、共享繁荣和建设韧性机构。该政策框架主张通过投资、教育与社会保障，确保AI收益广泛分配，并应对劳动力市场转型。"
        }
      ],
      "analysis": "分析师认为，OpenAI正通过政府合作、合规认证和政策倡议，深度参与AI治理。其策略兼顾安全、创新与公共利益，在欧美等关键市场推动监管对齐。企业应关注政策变化对AI部署、数据跨境和采购的影响 。"
    },
    {
      "section": "消费者产品与功能更新",
      "articles": [
        {
          "title": "Introducing ChatGPT Images 2.5",
          "url": "https://openai.com/index/introducing-chatgpt-images-2-5",
          "description": "OpenAI推出ChatGPT Images 2.5，可将想法、草图和参考照片转化为更个性化、精致的图像。该版本在文本渲染、多语言支持和视觉推理上改进，帮助用户更轻松地创建高质量视觉内容。"
        },
        {
          "title": "Introducing ChatGPT search",
          "url": "https://openai.com/index/introducing-chatgpt-search",
          "description": "OpenAI推出ChatGPT搜索，提供快速、及时的答案并附相关网页来源链接。该功能将搜索与对话结合，帮助用户获取最新信息，同时保持可验证性，对传统搜索市场形成竞争压力。"
        }
      ],
      "analysis": "分析师认为，ChatGPT正从聊天助手演进为覆盖图像、语音、搜索、购物和记忆的超级入口。OpenAI通过多模态和代理能力增强用户粘性，并探索广告与商业变现。企业应关注对话式商务和语音交互带来的新渠道。"
    }
  ]
}"#;
