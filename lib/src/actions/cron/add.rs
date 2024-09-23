use crate::atoms::cron::Add as AddCronAtom;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{actions::Action, steps::Step};

#[derive(JsonSchema, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CronAdd {
    pub name: String,
    pub description: String,
    pub schedule: String,
    pub user: String,
    pub privileged: bool,
}

impl CronAdd {}

impl Action for CronAdd {
    fn summarize(&self) -> String {
        format!("Add cron item {}", self.schedule)
    }

    fn plan(
        &self,
        _manifest: &crate::manifests::Manifest,
        _context: &crate::contexts::Contexts,
    ) -> anyhow::Result<Vec<crate::steps::Step>> {
        let steps = vec![Step {
            atom: Box::new(AddCronAtom { name: Some(self.name.clone()), description: Some(self.description.clone()), schedule: self.schedule.clone(), user: Some(self.user.clone()), privileged: Some(self.privileged) }),
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
                assert_eq!("a", action.action.target);
            }
            _ => {
                panic!("Cron Add didn't deserialize to the correct type");
            }
        };
    }
}
