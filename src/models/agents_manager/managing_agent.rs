use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
use crate::helpers::general::ai_task_request;
use crate::models::agents::agent_architect::AgentSolutionArchitect;
use crate::models::general::llm::Message;
use crate::models::{
    agent_basic::basic_agent::{AgentState, BasicAgent},
    agents::agent_traits::{FactSheet, SpecialFunctions},
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}

impl ManagingAgent {
    pub async fn new(usr_req: String) -> Result<Self, Box<dyn std::error::Error>> {
        let position = "Project Manager".to_string();

        let attributes = BasicAgent {
            objective: "Managing agents who are building a great website".to_string(),
            position: position.clone(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        let project_description: String = ai_task_request(
            usr_req,
            &position,
            get_function_string!(convert_user_input_to_goal),
            convert_user_input_to_goal,
        )
        .await;

        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];

        let factsheet = FactSheet {
            project_description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        Ok(Self {
            attributes,
            factsheet,
            agents,
        })
    }

    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
        // self.add_agent(Box::new(AgentSolutionArchitect::new()));
    }

    pub async fn execute_project(&mut self) {
        self.create_agents();

        for agent in &mut self.agents {
            let agent_result: Result<(), Box<dyn std::error::Error>> =
                agent.execute(&mut self.factsheet).await;

            let agent_info = agent.get_attributes_from_agent();
            dbg!(agent_info);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_managing_agent() {
        let usr_request: &str =
            "need a full stack app that that tracks and fetches my fitness progress. Needs to include timezone info from the web. Need to include login and logout";

        let mut managing_agent = ManagingAgent::new(usr_request.to_string())
            .await
            .expect("Error! coult not create managing agent");

        managing_agent.execute_project().await;

        dbg!(managing_agent.factsheet);
    }
}
