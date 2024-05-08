use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::env;

// Call Large Language Model (i.e. GPT-4)
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API Key
    let openai_api_key: String =
        env::var("OPENAI_API_MBOX_KEY").expect("OPENAI_API_KEY not found in .env");
    let openai_org: String = env::var("OPENAI_MBOX_ORG").expect("OPENAI_ORG not found in .env");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create header
    let mut headers = HeaderMap::new();

    // Create api key header
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", openai_api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(openai_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        // model: "gpt-3.5-turbo".to_string(),
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1,
    };

    // println!("API Request:\n{:?}", chat_completion);

    // Get API Response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    dbg!(&res);

    let api_response = res.choices[0].message.content.clone();
    // println!("API Response:\n{:?}", api_response.to_string());

    // Send response
    Ok(api_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Give me a short poem of four sentences".to_string(),
        };

        let messages = vec![message];

        let res = call_gpt(messages).await;
        if let Ok(res_str) = res {
            println!("{}", res_str);
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
