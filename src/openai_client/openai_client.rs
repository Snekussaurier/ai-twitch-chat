use crate::openai_client::openai_client_config;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O_MINI;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tokio::sync::OnceCell;

static CLIENT: OnceCell<Mutex<OpenAIClient>> = OnceCell::const_new();
static CONFIG: OnceLock<openai_client_config::Config> = OnceLock::new();

pub async fn call_openai_api(
    input: String,
    image: String,
) -> Result<String, openai_api_rs::v1::error::APIError> {
    let mut client = get_client().await.lock().await;
    let system_message = get_system_message();

    let mut messages = vec![system_message];

    let message = chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::ImageUrl(vec![
            chat_completion::ImageUrl {
                r#type: chat_completion::ContentType::text,
                text: Some(input),
                image_url: None,
            },
            chat_completion::ImageUrl {
                r#type: chat_completion::ContentType::image_url,
                text: None,
                image_url: Some(chat_completion::ImageUrlType {
                    url: format!("data:image/png;base64,{}", image),
                }),
            },
        ]),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    };

    messages.push(message);

    let req = ChatCompletionRequest::new(GPT4_O_MINI.to_string(), messages);

    let result = client.chat_completion(req).await?;

    Ok(result.choices[0]
        .message
        .content
        .clone()
        .unwrap_or_default())
}

async fn get_client() -> &'static Mutex<OpenAIClient> {
    CLIENT
        .get_or_init(|| async {
            let config = get_config();
            Mutex::new(
                OpenAIClient::builder()
                    .with_api_key(config.openai_api_key.clone())
                    .build()
                    .expect("Failed to create OpenAI client"),
            )
        })
        .await
}

fn get_system_message() -> chat_completion::ChatCompletionMessage {
    let config = get_config();
    chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::system,
        content: chat_completion::Content::Text(config.system_message.clone()),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }
}

fn get_config() -> &'static openai_client_config::Config {
    CONFIG.get_or_init(openai_client_config::load_config)
}
