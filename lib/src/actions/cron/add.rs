use crate::atoms::cron::Add as AddCronAtom;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{actions::Action, steps::Step};

#[derive(JsonSchema, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CronAdd {
    pub name: Option<String>,
    pub description: Option<String>,
    pub schedule: String,
    pub user: Option<String>,
    pub privileged: Option<bool>,
}

impl CronAdd {}

impl Action for CronAdd {
    fn summarize(&self) -> String {
        format!("Add cron item {} ({}: {})", self.schedule, self.name.clone().unwrap_or_default(), self.description.clone().unwrap_or_default())
    }

    fn plan(
        &self,
        _manifest: &crate::manifests::Manifest,
        _context: &crate::contexts::Contexts,
    ) -> anyhow::Result<Vec<crate::steps::Step>> {
        let steps = vec![Step {
            atom: Box::new(AddCronAtom {
                name: self.name.clone(),
                description: self.description.clone(),
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
- action: cron.add
  schedule: 00 00 * * * * script.sh
"#;

        let mut actions: Vec<Actions> = serde_yml::from_str(yaml).unwrap();

        match actions.pop() {
            Some(Actions::CronAdd(action)) => {
                assert_eq!("00 00 * * * * script.sh", action.action.schedule);
            }
            _ => {
                panic!("Cron Add didn't deserialize to the correct type");
            }
        };
    }
}
