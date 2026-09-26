pub const EXAMPLE_SUMMARY_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">

    <title>Daily News Digest</title>

    <style>
        :root {
            --cobalt: #3f5793;
            --cobalt-deep: #263b70;
            --cobalt-soft: #edf1fb;

            --teal: #2d847d;
            --teal-soft: #e8f4f1;

            --coral: #cf6b57;
            --coral-soft: #fbeee9;

            --amber: #c78a2c;
            --amber-soft: #fcf3df;

            --plum: #735c85;
            --plum-soft: #f2edf6;

            --page-start: #f8f3eb;
            --page-middle: #f4f6f2;
            --page-end: #edf2f4;
            --card: rgba(255, 253, 249, 0.94);
            --article-bg: rgba(250, 248, 244, 0.92);

            --text: #202839;
            --muted: #687080;
            --border: #dddcd7;

            --shadow-sm: 0 5px 16px rgba(48, 66, 92, 0.08);
            --shadow-md: 0 20px 52px rgba(48, 66, 92, 0.13);

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
                    circle at 8% 4%,
                    rgba(207, 107, 87, 0.18),
                    transparent 28%
                ),
                radial-gradient(
                    circle at 92% 8%,
                    rgba(45, 132, 125, 0.16),
                    transparent 26%
                ),
                radial-gradient(
                    circle at 72% 78%,
                    rgba(199, 138, 44, 0.10),
                    transparent 30%
                ),
                linear-gradient(
                    180deg,
                    var(--page-start),
                    var(--page-middle) 48%,
                    var(--page-end)
                );
        }

        ::selection {
            color: #ffffff;
            background: var(--coral);
        }

        a {
            color: inherit;
        }

        .container {
            width: 100%;
            max-width: 1080px;
            margin: 0 auto;
        }

        /* Page header */
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
            color: #6c3f36;
            font-size: 0.88rem;
            font-weight: 700;
            letter-spacing: 0.04em;
            background: linear-gradient(
                90deg,
                rgba(251, 238, 233, 0.94),
                rgba(252, 243, 223, 0.94)
            );
            border: 1px solid #edc8b8;
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

        /* Groups */
        .group {
            --group-strong: var(--cobalt);
            --group-deep: var(--cobalt-deep);
            --group-secondary: var(--teal);
            --group-soft: var(--cobalt-soft);
            --group-border: #cbd5ed;
            margin-bottom: 64px;
        }

        .group:nth-of-type(4n + 2) {
            --group-strong: var(--teal);
            --group-deep: #1e625d;
            --group-secondary: var(--amber);
            --group-soft: var(--teal-soft);
            --group-border: #bddbd5;
        }

        .group:nth-of-type(4n + 3) {
            --group-strong: var(--coral);
            --group-deep: #914437;
            --group-secondary: var(--plum);
            --group-soft: var(--coral-soft);
            --group-border: #edc8bd;
        }

        .group:nth-of-type(4n) {
            --group-strong: var(--plum);
            --group-deep: #503d61;
            --group-secondary: var(--cobalt);
            --group-soft: var(--plum-soft);
            --group-border: #d7c9df;
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
                var(--group-strong),
                var(--group-secondary)
            );
            border-radius: 14px;
            box-shadow: var(--shadow-sm);
        }

        .group-heading {
            min-width: 0;
        }

        .group-label {
            margin-bottom: 2px;
            color: var(--group-strong);
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

        /* Timeline */
        .timeline {
            position: relative;
            margin-left: 21px;
            padding-left: 34px;
            border-left: 3px solid var(--group-border);
        }

        /* Summary card */
        .summary-card {
            position: relative;
            margin-bottom: 34px;
            padding: 28px;
            overflow: hidden;
            background: var(--card);
            border: 1px solid rgba(218, 216, 208, 0.92);
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
                var(--group-strong),
                var(--group-secondary)
            );
            border: 5px solid var(--group-soft);
            border-radius: 50%;
            box-shadow: 0 0 0 2px var(--group-border);
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
                var(--group-soft),
                transparent 70%
            );
        }

        /* Overall summary */
        .conclusion {
            position: relative;
            z-index: 1;
            margin-bottom: 28px;
            padding: 20px 22px;
            background: linear-gradient(
                135deg,
                var(--group-soft),
                rgba(252, 243, 223, 0.72)
            );
            border: 1px solid var(--group-border);
            border-radius: var(--radius-md);
        }

        .conclusion-title {
            display: flex;
            align-items: center;
            gap: 8px;
            margin-bottom: 9px;
            color: var(--group-deep);
            font-size: 0.92rem;
            font-weight: 800;
            letter-spacing: 0.03em;
        }

        .conclusion-content {
            color: #334155;
            white-space: pre-wrap;
            overflow-wrap: anywhere;
        }

        /* Content section */
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
                var(--group-strong),
                var(--group-secondary)
            );
            border-radius: 99px;
        }

        .section-title {
            margin: 0;
            font-size: 1.12rem;
            line-height: 1.4;
            color: var(--group-deep);
        }

        /* Article */
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
            background: #fffefa;
            border-color: var(--group-border);
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
            color: var(--group-strong);
        }

        .article a {
            color: var(--group-deep);
            font-weight: 700;
            line-height: 1.5;
            text-decoration: none;
            overflow-wrap: anywhere;
        }

        .article a:hover {
            color: var(--group-strong);
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

        /* Analysis */
        .analysis {
            margin-top: 18px;
            padding: 17px 19px;
            background: linear-gradient(
                110deg,
                var(--amber-soft),
                rgba(251, 238, 233, 0.78)
            );
            border: 1px solid #ead19e;
            border-left: 5px solid var(--amber);
            border-radius: var(--radius-sm);
        }

        .analysis-title {
            display: flex;
            align-items: center;
            gap: 8px;
            margin-bottom: 7px;
            color: #81551d;
            font-size: 0.9rem;
            font-weight: 800;
        }

        .analysis-content {
            color: #654a2b;
            font-size: 0.95rem;
            white-space: pre-wrap;
            overflow-wrap: anywhere;
        }

        /* Empty state */
        .empty-state {
            padding: 34px 24px;
            text-align: center;
            color: var(--muted);
            background: rgba(255, 253, 249, 0.76);
            border: 1px dashed #c9c6bd;
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

        /* Footer */
        footer {
            margin-top: 80px;
            padding-top: 24px;
            text-align: center;
            color: var(--muted);
            font-size: 0.86rem;
            border-top: 1px solid rgba(203, 213, 225, 0.8);
        }

        /* Mobile */
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

        /* Print and PDF export */
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

        <h1 class="page-title">Daily News Digest</h1>

        <p class="page-subtitle">
            Important updates, key perspectives, and trend analysis from your feeds.
        </p>
    </header>

    {% if groups | length == 0 %}
    <section class="empty-state">
        <i class="empty-icon">📭</i>
        <p class="empty-title">No feed groups</p>
        <p class="empty-description">
            There is no feed content to display yet.
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
                <p class="empty-title">No updates in this group</p>
                <p class="empty-description">
                    No summaries are currently available for this group.
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
                            <span>Overall Summary</span>
                        </div>

                        <div class="conclusion-content">
                            {{ summary.conclusion }}
                        </div>
                    </section>
                    {% endif %}

                    {% if summary.posts | length == 0 %}
                    <div class="empty-state">
                        <i class="empty-icon">🗂️</i>
                        <p class="empty-title">No details available</p>
                        <p class="empty-description">
                            This summary does not contain any article categories.
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
                                <p class="empty-title">No articles in this category</p>
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
                                    <span>Trends and Perspective</span>
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
        Aggregated information is for reference only · Generated with Rust and Tera
    </footer>
</main>
</body>
</html>
"#;

pub const EXAMPLE_FEED_TOML: &str = r#"# ==============================================
# Feed groups organized by format
# The group name is a display label only and does not affect parsing
# ==============================================

# ---------- RSS 2.0 ----------
[[group]]
name = "RSS 2.0"

[[group.feed]]
name = "OpenAI Blog"
url = "https://openai.com/news/rss.xml"
enabled = true

[[group.feed]]
name = "Personal Technology Blog Example"
url = "http://yatagarasu.nekoweb.org/feed.xml"
enabled = true

# ---------- Atom 1.0 ----------
[[group]]
name = "Atom 1.0"

[[group.feed]]
name = "Ruanyifeng's Blog"
url = "https://www.ruanyifeng.com/blog/atom.xml"
enabled = true

[[group.feed]]
name = "Mirror.xyz Example"
url = "https://ens.mirror.xyz/feed/atom"
enabled = true

# ---------- JSON Feed ----------
[[group]]
name = "JSON Feed"

[[group.feed]]
name = "Official JSON Feed Example"
url = "https://www.jsonfeed.org/feed.json"
enabled = true

[[group.feed]]
name = "Daring Fireball"
url = "https://daringfireball.net/feed/json"
enabled = true
"#;

#[cfg(test)]
pub const EXAMPLE_SUMMARY_OUTPUT: &str = r#"{
  "conclusion": "The OpenAI blog published roughly 500 updates spanning model releases, Codex and agents, enterprise adoption, safety governance, health and science, education, infrastructure, public policy, consumer products, and corporate governance. The main themes were rapid frontier-model iteration and Codex's evolution from a coding tool into an agent platform. OpenAI also expanded red-team work, threat intelligence, infrastructure investment, and partnerships across healthcare, education, and government. Organizations should assess the return on investment of new models and Codex workflows while establishing security, governance, and compliance controls that can keep pace with improving capabilities.",
  "posts": [
    {
      "section": "Infrastructure and partnerships",
      "articles": [
        {
          "title": "Announcing The Stargate Project",
          "url": "https://openai.com/index/announcing-the-stargate-project",
          "description": "OpenAI announced the Stargate project, a planned investment in AI infrastructure with initial data centers in the United States. The project brings together partners including Oracle and SoftBank to expand computing capacity, support advanced AI development, create jobs, and strengthen the domestic AI ecosystem."
        },
        {
          "title": "OpenAI and Broadcom unveil LLM-optimized inference chip",
          "url": "https://openai.com/index/openai-broadcom-jalapeno-inference-chip",
          "description": "OpenAI and Broadcom introduced Jalapeno, a custom chip optimized for large-model inference. It targets the throughput and latency requirements of modern models and is intended to improve performance, energy efficiency, and deployment scale while reducing inference cost and reliance on general-purpose GPUs."
        },
        {
          "title": "AWS and OpenAI announce multi-year strategic partnership",
          "url": "https://openai.com/index/aws-and-openai-partnership",
          "description": "OpenAI and AWS announced a multi-year strategic partnership under which AWS will provide infrastructure and computing capacity for next-generation model training. The agreement broadens OpenAI's cloud strategy and illustrates how rapidly growing AI demand is driving deeper relationships with hyperscale cloud providers."
        }
      ],
      "analysis": "Compute and infrastructure are central constraints in AI competition. OpenAI is combining large infrastructure programs, custom chips, multi-cloud partnerships, and software optimization into a full-stack strategy. Organizations should plan for compute cost, data residency, portability, and scalable deployment capacity."
    },
    {
      "section": "Policy and government",
      "articles": [
        {
          "title": "Expanding AI access and cyber defense for federal, state, local, and tribal governments",
          "url": "https://openai.com/index/expanding-ai-access-us-government",
          "description": "OpenAI partnered with the GSA to offer eligible United States federal, state, local, and tribal governments discounted access and expanded cyber-defense support. The program is intended to accelerate responsible public-sector AI adoption while strengthening government cybersecurity capabilities."
        },
        {
          "title": "Industrial policy for the Intelligence Age",
          "url": "https://openai.com/index/industrial-policy-for-the-intelligence-age",
          "description": "OpenAI proposed an industrial policy framework for the intelligence age focused on expanding opportunity, sharing prosperity, and building resilient institutions. The recommendations emphasize investment, education, and social protections so that AI benefits are broadly distributed while labor markets adapt."
        }
      ],
      "analysis": "OpenAI is participating in AI governance through government partnerships, compliance programs, and policy proposals. Organizations should monitor how changing rules affect deployment, cross-border data movement, procurement, and risk management in major markets."
    },
    {
      "section": "Consumer product updates",
      "articles": [
        {
          "title": "Introducing ChatGPT Images 2.5",
          "url": "https://openai.com/index/introducing-chatgpt-images-2-5",
          "description": "OpenAI introduced ChatGPT Images 2.5, which turns ideas, sketches, and reference photos into more personalized and polished images. The release improves text rendering, multilingual support, and visual reasoning to make high-quality visual creation easier."
        },
        {
          "title": "Introducing ChatGPT search",
          "url": "https://openai.com/index/introducing-chatgpt-search",
          "description": "OpenAI introduced ChatGPT search, providing timely answers with links to relevant web sources. The feature combines search and conversation so users can retrieve current information while retaining source visibility and verifiability."
        }
      ],
      "analysis": "ChatGPT is evolving from a conversational assistant into a broader interface spanning images, voice, search, shopping, and memory. Organizations should watch how multimodal agents, conversational commerce, and voice interaction create new customer channels and business models."
    }
  ]
}"#;
