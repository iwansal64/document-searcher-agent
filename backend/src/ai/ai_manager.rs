use rig::client::ProviderClientError;
use rig::completion::CompletionModel;
use rig::http_client::ReqwestClient;
use rig::message::AssistantContent;
use rig::prelude::*;
use rig::providers::gemini;

#[derive(Clone)]
pub struct AIModel {
        completion_model: gemini::CompletionModel,
}

impl AIModel {
        pub async fn ask(&self, message: &String) -> Result<String, String> {
                let request = self.completion_model.completion_request(message).build();
                let response = self.completion_model.completion(request).await;
                let response = match response {
                        Ok(res) => res,
                        Err(err) => {
                                return Err(err.to_string());
                        }
                };

                let mut result: String = String::new();
                for item in response.choice {
                        if let AssistantContent::Text(text) = item {
                                result += text.to_string().as_str();
                                result += "\n";
                        }
                }

                Ok(result)
        }
}

pub async fn create_ai() -> Result<AIModel, ProviderClientError> {
        let client: gemini::Client<ReqwestClient> = gemini::Client::from_env()?; // reads OPENAI_API_KEY
        let completion_model: gemini::CompletionModel = client.completion_model("gemini-3.5-flash");

        Ok(AIModel {
                completion_model: completion_model,
        })
}
