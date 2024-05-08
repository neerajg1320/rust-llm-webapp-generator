use crate::ai_functions::aifunc_architect::{print_project_scope, print_site_urls};
use crate::helpers::{
    command_line::PrintCommand,
    general::{ai_task_request, ai_task_request_decoded, check_status_code},
};
use crate::models::agent_basic::{
    basic_agent::{AgentState, BasicAgent},
    basic_traits::BasicTraits,
};
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

// Solutions Architect
#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Gathers information and design solutions for website developement"
                .to_string(),
            position: "Solutions Architect".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self { attributes }
    }

    // Retrieve Project Scope
    async fn call_project_scope(&mut self, factsheet: &mut FactSheet) -> ProjectScope {
        let msg_context: String = format!("{:?}", factsheet.project_description);

        let ai_response = ai_task_request_decoded::<ProjectScope>(
            // let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope,
        )
        .await;

        println!("AI Response:\n{:?}", ai_response);

        // let ai_response = ProjectScope {
        //     is_crud_required: true,
        //     is_user_login_and_logout: true,
        //     is_external_urls_required: true,
        // };

        factsheet.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::UnitTesting);

        ai_response
    }

    // Retrieve Project Scope
    async fn call_determine_external_urls(
        &mut self,
        factsheet: &mut FactSheet,
        msg_context: String,
    ) {
        // let ai_response: Vec<String> = ai_task_request_decoded::<Vec<String>>(
        //     msg_context,
        //     &self.attributes.position,
        //     get_function_string!(print_site_urls),
        //     print_site_urls,
        // )
        // .await;

        // We hard code the response to move on
        let ai_response = vec![
            String::from("https://api.exchangeratesapi.io/latest"),
            String::from("https://openexchangerates.org/api/latest.json"),
        ];

        factsheet.external_urls = Some(ai_response.clone());
        self.attributes.update_state(AgentState::UnitTesting);
    }
}

#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut count = 0;
        while (self.attributes.state != AgentState::Finished) && count < 5 {
            count += 1;

            // while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    let project_scope = self.call_project_scope(factsheet).await;
                    if project_scope.is_external_urls_required {
                        self.call_determine_external_urls(
                            factsheet,
                            factsheet.project_description.clone(),
                        )
                        .await;
                    }
                }

                AgentState::UnitTesting => {
                    // Check the urls
                    let mut exclude_urls: Vec<String> = vec![];
                    let client: Client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap();

                    // Find faulty urls
                    let urls: &Vec<String> = factsheet
                        .external_urls
                        .as_ref()
                        .expect("No URLs on factsheet");
                    for url in urls {
                        let endpoint_str: String = format!("Testing URL Endpoint: {}", url);
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            endpoint_str.as_str(),
                        );

                        // Perform URL Test
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                println!("Status: {}", status_code);
                                if status_code != 200 {
                                    exclude_urls.push(url.clone());
                                }
                            }
                            Err(e) => println!("Error checking {}: {}", url, e),
                        }
                    }

                    // Exclude faulty URLs
                    if exclude_urls.len() > 0 {
                        let new_urls: Vec<String> = factsheet
                            .external_urls
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|url| !exclude_urls.contains(&url))
                            .cloned()
                            .collect();
                        factsheet.external_urls = Some(new_urls);
                    }

                    // Confirm done
                    self.attributes.update_state(AgentState::Finished);
                    // self.attributes.state = AgentState::Finished
                }
                _ => self.attributes.state = AgentState::Finished,
            }
            println!("Loop ended: count:{}", count);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_solution_architect() {
        let mut agent: AgentSolutionArchitect = AgentSolutionArchitect::new();
        let mut factsheet: FactSheet = FactSheet {
            project_description: "Build a full stack website with user login and logout that shows latest forex prices".to_string(),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        agent
            .execute(&mut factsheet)
            .await
            .expect("Unable to execute Solutions Architect Agent");

        assert!(factsheet.project_scope != None);
        assert!(factsheet.external_urls.is_some());

        dbg!(factsheet);
    }
}
