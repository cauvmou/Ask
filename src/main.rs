use clap::Parser;
use keyring::Entry;
use async_openai::{Client, config::OpenAIConfig};
use async_openai::types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, CreateCompletionRequestArgs, Role};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Your OpenAI API key (Cannot be set at the same time as asking a question!)
    #[arg(short, long, value_name = "KEY", id = "key")]
    api_key: Option<String>,

    /// How many tokens should be generated.
    #[arg(short, long, value_name = "MAX TOKENS", id = "tokens", default_value_t = 1024)]
    max_tokens: u16,

    /// Your prompt for ChatGPT
    #[arg(id = "prompt")]
    prompt: Option<String>
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = OpenAIConfig::default();
    let entry = Entry::new("ask-gpt", "cli").expect("Failed to retrieve keystore!");

    if let Some(api_key) = args.api_key {
        entry.set_password(api_key.as_str()).expect("Failed to store API-KEY!");
    }

    let prompt = if let Some(prompt) = args.prompt { prompt } else { return };
    let api_key = entry.get_password().expect("Failed to retrieve secret, have you set the API-KEY?");
    let config = config.with_api_key(api_key);
    let client = Client::with_config(config);

    let system = ChatCompletionRequestMessage {
        role: Role::System,
        content: Some("You are a helpful copilot that lives inside the terminal of the user, therefore it is a given that everything is already happening inside a terminal. Your name is Patrik. Give precise short answers and point the user to possible online resources, use bullet points. Limit your responses to only a few words and sentences. No response should be more than five sentences max. Use linebreaks to make your output more readable.".to_string()),
        ..Default::default()
    };

    let message = ChatCompletionRequestMessage {
        content: Some(prompt),
        ..Default::default()
    };

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages(vec![system, message])
        .max_tokens(args.max_tokens)
        .n(1)
        .temperature(0.0)
        .build()
        .unwrap();
    let response = client.chat().create(request).await.expect("Failed to make request to OpenAI!");
    let content = response.choices.get(0).expect("HUH?").message.content.as_ref().unwrap().to_owned();
    println!("{content}");
}