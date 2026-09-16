use crate::app::AppError;
use crate::config::prompts::PROMPTS;
use crate::model::article::Feed;
use crate::model::config::LLM;
use crate::model::summary::FeedSummary;
use eventsource_stream::Eventsource;
use futures_util::StreamExt;
use reqwest::header::{ACCEPT, ACCEPT_ENCODING};
use reqwest::{Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, info};

const FLUSH_EVERY: usize = 40;

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    response_format: ResponseFormat,
    thinking: Thinking,
    reasoning_effort: String,
    stream: bool
}
/// enable or disable
#[derive(Debug, Serialize, Deserialize)]
struct Thinking {
    #[serde(rename = "type")]
    think_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize,Serialize)]
struct ResponseFormat {
    #[serde(rename="type")]
    format_type:String
}

#[derive(Debug, Deserialize)]
pub struct ChatChunk {
    pub id: Option<String>,
    pub choices: Vec<ChunkChoice>,
}

#[derive(Debug, Deserialize)]
pub struct ChunkChoice {
    pub index: u32,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Delta {
    pub role: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    // 一些提供商用 `reasoning_content` 传输思考流：
    #[serde(default)]
    pub reasoning_content: Option<String>,
}

impl LLM {
    pub async  fn is_llm_reachable(&self, cli:Client)->Result<bool,AppError>{
        cli
            .get(&self.base_url)
            .header(ACCEPT, "*/*")
            .header(ACCEPT_ENCODING, "identity")
            .send()
            .await?;

        Ok(true)
    }

    pub async fn get_summary(&self,cli:Client,articles:&Feed) ->Result<FeedSummary,AppError>{
        let body:ChatRequest=ChatRequest{
            model: self.model.clone(),

            response_format:ResponseFormat{
                format_type:"json_object".to_string()
            },
            thinking: Thinking{think_type: "enabled".parse().unwrap() },
            reasoning_effort: "medium".to_string(),
            temperature:1.0,
            messages:vec![
                Message{
                    role:"system".to_string(),
                    content:PROMPTS.to_string(),
                },
                Message{
                    role:"user".to_string(),
                    content:serde_json::to_string(articles)?,
                }
            ],
            stream: true,
        };
        let res = cli
            .post(format!(
                "{}/chat/completions",
                self.base_url.trim_end_matches('/')
            ))
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await?;
            return Err(format!("llm request error: {}, {}", status, text).into());
        }

        let mut stream=res.bytes_stream().eventsource();

        let mut response:String="".to_string();
        let mut reasoning:String="".to_string();
        while let Some(event) = stream.next().await {
            let event = event?;
            if event.data == "[DONE]" {
                break;
            }
            let v: Value = serde_json::from_str(&event.data)?;
            if let Some(text) = v["choices"][0]["delta"]["content"].as_str() {
                response.push_str(text);
            }
            if let Some(text) = v["choices"][0]["delta"]["reasoning_content"].as_str() {
                reasoning.push_str(text);
            }
        }
        info!("Got summary {} bytes", response.len());
        debug!("Got reason {} bytes, content {}", reasoning.len(), reasoning);
        debug!("Got summary {} bytes, content {}",response.len(), response);
        // parse feed summary
        let data:FeedSummary=serde_json::from_str(response.as_str())?;

        Ok(data)
    }
}


#[cfg(test)]
mod tests {
    use crate::model::summary::FeedSummary;

    #[test]
    fn test_json_parse(){
        let sample=r#"{"conclusion": "今日OpenAI官方博客更新文章约500篇，覆盖模型发布、Codex与智能体、企业落地、安全治理、医疗科学、教育、基础设施、政策合作、消费者产品及公司治理等十大方向。核心主线是GPT-6 Astra、GPT-5.6 等前沿模型迭代，以及Codex从编码工具升级为智能体平台。安全方面，OpenAI达到网络安全Critical级别并强化红队与威胁情报。基础设施上，Stargate、自研芯片与多云合作加速算力扩张。医疗、教育、政府合作持续深化。 分析师建议企业优先评估最新模型与Codex在工作流中的ROI，同时建立AI安全治理与合规框架，以应对能力快速提升带来的风险与机遇。","posts": [{"section": "基础设施、算力与合作伙伴","articles": [{"title": "Announcing The Stargate Project","url": "https://openai.com/index/announcing-the-stargate-project","content": "OpenAI宣布Stargate项目，计划投资5000亿美元建设AI基础设施，首批数据中心在美国展开。该项目联合Oracle、SoftBank等伙伴，旨在扩展算力以支持AGI，创造就业并巩固美国AI领导地位。"},{"title": "OpenAI and Broadcom unveil LLM-optimized inference chip","url": "https://openai.com/index/openai-broadcom-jalapeno-inference-chip","content": "OpenAI与博通推出Jalapeño，一款专为LLM推理优化的定制AI芯片，旨在提升性能、能效和规模。该芯片针对现代模型的高吞吐与低延迟需求，有望降低推理成本，并减少对通用GPU的依赖。"},{"title": "AWS and OpenAI announce multi-year strategic partnership","url": "https://openai.com/index/aws-and-openai-partnership","content": "OpenAI与AWS达成多年期380亿美元战略合作，AWS将为OpenAI提供世界级基础设施和算力，支持下一代模型训练。该合作扩展OpenAI的云战略，也反映AI算力需求正推动超大规模云厂商深度绑定。"}],"analysis": "分析师认为，算力与基础设施已成为AI竞争的核心瓶颈。OpenAI通过Stargate、自研芯片、多云合作和数据库优化，构建从训练到推理的全栈能力。企业应关注AI算力成本、数据驻留与云战略，提前规划可 扩展的AI基础设施。"},{"section": "政策、政府与国际合作","articles": [{"title": "Expanding AI access and cyber defense for federal, state, local, and tribal governments","url": "https://openai.com/index/expanding-ai-access-us-government","content": "OpenAI与GSA合作，向符合条件的美国联邦、州、地方和部落政府提供零许可费、50%用量折扣及扩展网络防御支持。该举措旨在加速公共部门AI采用，同时强化政府网络安全能力。"},{"title": "Industrial policy for the Intelligence Age","url": "https://openai.com/index/industrial-policy-for-the-intelligence-age","content": "OpenAI提出面向智能时代的产业政策建议，聚焦扩大机会、共享繁荣和建设韧性机构。该政策框架主张通过投资、教育与社会保障，确保AI收益广泛分配，并应对劳动力市场转型。"}],"analysis": "分析师认为，OpenAI正通过政府合作、合规认证和政策倡议，深度参与AI治理。其策略兼顾安全、创新与公共利益，在欧美等关键市场推动监管对齐。企业应关注政策变化对AI部署、数据跨境和采购的影响 。"},{"section": "消费者产品与功能更新","articles": [{"title": "Introducing ChatGPT Images 2.5","url": "https://openai.com/index/introducing-chatgpt-images-2-5","content": "OpenAI推出ChatGPT Images 2.5，可将想法、草图和参考照片转化为更个性化、精致的图像。该版本在文本渲染、多语言支持和视觉推理上改进，帮助用户更轻松地创建高质量视觉内容。"},{"title": "Introducing ChatGPT search","url": "https://openai.com/index/introducing-chatgpt-search","content": "OpenAI推出ChatGPT搜索，提供快速、及时的答案并附相关网页来源链接。该功能将搜索与对话结合，帮助用户获取最新信息，同时保持可验证性，对传统搜索市场形成竞争压力。"}],"analysis": "分析师认为，ChatGPT正从聊天助手演进为覆盖图像、语音、搜索、购物和记忆的超级入口。OpenAI通过多模态和代理能力增强用户粘性，并探索广告与商业变现。企业应关注对话式商务和语音交互带来的新渠道。"}]}"#;
        let summary_ok=serde_json::from_str::<FeedSummary>(sample).is_ok();
        assert!(summary_ok);
    }
}