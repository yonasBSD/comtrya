use crate::atoms::cron::List as ListCronAtom;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{actions::Action, steps::Step};

#[derive(JsonSchema, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CronList {
    pub user: String,
}

impl CronList {}

impl Action for CronList {
    fn summarize(&self) -> String {
        format!("List cron for user {}", self.user)
    }

    fn plan(
        &self,
        _manifest: &crate::manifests::Manifest,
        _context: &crate::contexts::Contexts,
    ) -> anyhow::Result<Vec<crate::steps::Step>> {
        let user = String::from("");
        let privileged = false;

        let steps = vec![Step {
            atom: Box::new(ListCronAtom { user: Some(user), privileged: Some(privileged) }),
            initializers: vec![],
            finalizers: vec![],
        }];

        Ok(steps)
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::Actions;

    #[test]
    fn it_can_be_deserialized() {
        let yaml = r#"
- action: cron.list
"#;

        let mut actions: Vec<Actions> = serde_yml::from_str(yaml).unwrap();

        match actions.pop() {
            Some(Actions::CronList(action)) => {
                assert_eq!("a", action.action.target);
            }
            _ => {
                panic!("Cron List didn't deserialize to the correct type");
            }
        };
    }
}
