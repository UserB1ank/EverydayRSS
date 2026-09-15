pub const TEMPLATE:&str=r#"# ==============================================
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

pub const JSON_EXAMPLE:&str=r#"# 
{
    "conclusion":"今日文章数量xxx篇，其中{section}x篇。今日内容聚焦于网络安全领域的前沿研究，包括React和V8引擎的高危漏洞分析、安全领域大模型数据集的构建与应用、二进制安全工具的优化升级以及智能网联汽车的安全评估。科恩实验室在漏洞披露、AI赋能安全、工具开源等多个方向展现出显著成果。建议安全从业者重点关注React和V8漏洞修复，同时借鉴SecCorpus和CarVal等新技术提升自身防御体系。"
    "posts":[
        {
            "section":"AI安全趋势xxxx",
            "articles":[{
            "title": "BinaryAI二进制比对功能设计与实现｜大模型下函数的语义匹配",
            "link": "https://keenlab.tencent.com/zh/2023/07/13/2023-BinaryAI-update20230713-release/",
            "content": "BinaryAI平台新增了基于大模型BAI-2.0的二进制文件比对功能，采用启发式算法提高复杂场景下的准确率和召回率。通过三阶段流程（初始匹配、扩散匹配、剩余匹配）实现高效匹配，并通过测试数据表明效果优于传统工具。"
            },
            {
                "title": "腾讯安全科恩实验室推出首款免费在线SCA平台：BinaryAI",
                "link": "https://keenlab.tencent.com/zh/2021/08/11/2021-binaryai-public-release/",
                "content": "BinaryAI是腾讯安全科恩实验室推出的首个面向日常安全研究的在线软件成分分析平台。通过自动化解包和反编译流程，可识别二进制文件中使用的第三方组件及其版本号，帮助用户发现潜在安全问题。平台已积累大量组件数据，具备较高的检测能力。"
            }],
            "analysis":"分析师认为AI安全是xxxx"
        },
        {
            "section":"xxx"
            xxxx
        }
    ]

}"#;