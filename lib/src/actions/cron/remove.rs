use crate::atoms::cron::Remove as RemoveCronAtom;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{actions::Action, steps::Step};

#[derive(JsonSchema, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CronRemove {
    pub name: Option<String>,
    pub user: Option<String>,
    pub schedule: String,
    pub privileged: Option<bool>,
}

impl CronRemove {}

impl Action for CronRemove {
    fn summarize(&self) -> String {
        format!("Remove cron item {}", self.schedule)
    }

    fn plan(
        &self,
        _manifest: &crate::manifests::Manifest,
        _context: &crate::contexts::Contexts,
    ) -> anyhow::Result<Vec<crate::steps::Step>> {
        let steps = vec![Step {
            atom: Box::new(RemoveCronAtom {
                name: self.name.clone(),
                schedule: self.schedule.clone(),
                user: self.user.clone(),
                privileged: self.privileged,
            }),
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
- action: cron.remove
  schedule: 00 00 * * * * script.sh
"#;

        let mut actions: Vec<Actions> = serde_yml::from_str(yaml).unwrap();

        match actions.pop() {
            Some(Actions::CronRemove(action)) => {
                assert_eq!("a", action.action.target);
            }
            _ => {
                panic!("Cron Remove didn't deserialize to the correct type");
            }
        };
    }
}
