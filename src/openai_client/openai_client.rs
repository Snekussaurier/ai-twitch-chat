#[cfg(feature = "server")]
mod openai_client_operations {
    use crate::openai_client::openai_client_config;
    use dioxus::fullstack::once_cell::sync::OnceCell;
    use openai_api_rs::v1::api::OpenAIClient;
    use openai_api_rs::v1::chat_completion;
    use std::sync::OnceLock;
    use tokio::sync::Mutex;

    static CLIENT: OnceCell<Mutex<OpenAIClient>> = OnceCell::new();
    static CONFIG: OnceLock<openai_client_config::Config> = OnceLock::new();

    const OPEANI_API_KEY: &str = "";

    pub fn get_client() -> &'static Mutex<OpenAIClient> {
        CLIENT.get_or_init(|| {
            let config = get_config();
            Mutex::new(
                OpenAIClient::builder()
                    .with_api_key(config.openai_api_key.clone())
                    .build()
                    .expect("Failed to create OpenAI client"),
            )
        })
    }

    pub fn get_system_message() -> chat_completion::ChatCompletionMessage {
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
}

use dioxus::prelude::*;

#[cfg(feature = "server")]
pub async fn call_openai_api(input: String) -> Result<String, ServerFnError> {
    use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
    use openai_api_rs::v1::common::GPT3_5_TURBO;

    let mut client = openai_client_operations::get_client().lock().await;
    let system_message = openai_client_operations::get_system_message();

    let mut messages = vec![system_message];
    messages.push(chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::Text(input),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    });

    let req = ChatCompletionRequest::new(GPT3_5_TURBO.to_string(), messages);

    let result = client.chat_completion(req).await?;

    Ok(result.choices[0].message.content.clone().expect("REASON"))
}
