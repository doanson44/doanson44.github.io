use leptos::prelude::*;

use crate::domain::account_manager::{Account, Environment, OtpMode, Project};
use crate::infrastructure::account_manager::LocalAccountManagerStorage;

#[derive(Clone, Copy)]
pub struct AccountManagerState {
    storage: LocalAccountManagerStorage,
    pub projects: RwSignal<Vec<Project>>,
    pub selected_project: RwSignal<Option<String>>,
    pub selected_environment: RwSignal<Option<String>>,
    pub selected_account: RwSignal<Option<String>>,
    pub now_seconds: RwSignal<u64>,
    pub message: RwSignal<Option<String>>,
    pub transfer_json: RwSignal<String>,
}

impl Default for AccountManagerState {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountManagerState {
    pub fn new() -> Self {
        let storage = LocalAccountManagerStorage;
        let projects = storage.load().unwrap_or_default();
        let selected_project = projects.first().map(|project| project.id.clone());
        let selected_environment = projects
            .first()
            .and_then(|project| project.environments.first())
            .map(|environment| environment.id.clone());
        let selected_account = projects
            .first()
            .and_then(|project| project.environments.first())
            .and_then(|environment| environment.accounts.first())
            .map(|account| account.id.clone());

        Self {
            storage,
            projects: RwSignal::new(projects),
            selected_project: RwSignal::new(selected_project),
            selected_environment: RwSignal::new(selected_environment),
            selected_account: RwSignal::new(selected_account),
            now_seconds: RwSignal::new((js_sys::Date::now() / 1000.0) as u64),
            message: RwSignal::new(None),
            transfer_json: RwSignal::new(String::new()),
        }
    }

    pub fn save(&self) {
        match self.storage.save(&self.projects.get_untracked()) {
            Ok(()) => self.message.set(None),
            Err(error) => self.message.set(Some(error)),
        }
    }

    pub fn add_project(&self) {
        let id = uuid::Uuid::new_v4().to_string();
        self.projects.update(|projects| projects.push(Project {
            id: id.clone(),
            name: "New Project".into(),
            description: String::new(),
            environments: Vec::new(),
        }));
        self.selected_project.set(Some(id));
        self.selected_environment.set(None);
        self.selected_account.set(None);
        self.save();
    }

    pub fn add_environment(&self) {
        let Some(project_id) = self.selected_project.get_untracked() else {
            self.message.set(Some("Create or select a project first.".into()));
            return;
        };
        let id = uuid::Uuid::new_v4().to_string();
        self.projects.update(|projects| {
            if let Some(project) = projects.iter_mut().find(|project| project.id == project_id) {
                project.environments.push(Environment {
                    id: id.clone(),
                    name: "DEV".into(),
                    ..Environment::default()
                });
            }
        });
        self.selected_environment.set(Some(id));
        self.selected_account.set(None);
        self.save();
    }

    pub fn add_account(&self) {
        let (Some(project_id), Some(environment_id)) = (
            self.selected_project.get_untracked(),
            self.selected_environment.get_untracked(),
        ) else {
            self.message.set(Some("Select an environment first.".into()));
            return;
        };
        let id = uuid::Uuid::new_v4().to_string();
        self.projects.update(|projects| {
            if let Some(environment) = projects.iter_mut()
                .find(|project| project.id == project_id)
                .and_then(|project| project.environments.iter_mut().find(|env| env.id == environment_id))
            {
                environment.accounts.push(Account {
                    id: id.clone(),
                    name: "New Account".into(),
                    ..Account::default()
                });
            }
        });
        self.selected_account.set(Some(id));
        self.save();
    }

    pub fn select_project(&self, project_id: String) {
        let projects = self.projects.get_untracked();
        let environment_id = projects.iter()
            .find(|project| project.id == project_id)
            .and_then(|project| project.environments.first())
            .map(|environment| environment.id.clone());
        let account_id = projects.iter()
            .find(|project| project.id == project_id)
            .and_then(|project| project.environments.first())
            .and_then(|environment| environment.accounts.first())
            .map(|account| account.id.clone());
        self.selected_project.set(Some(project_id));
        self.selected_environment.set(environment_id);
        self.selected_account.set(account_id);
    }

    pub fn select_environment(&self, environment_id: String) {
        let projects = self.projects.get_untracked();
        let account_id = projects.iter()
            .flat_map(|project| project.environments.iter())
            .find(|environment| environment.id == environment_id)
            .and_then(|environment| environment.accounts.first())
            .map(|account| account.id.clone());
        self.selected_environment.set(Some(environment_id));
        self.selected_account.set(account_id);
    }

    pub fn update_project(&self, field: &str, value: String) {
        let Some(id) = self.selected_project.get_untracked() else { return; };
        self.projects.update(|projects| {
            if let Some(project) = projects.iter_mut().find(|project| project.id == id) {
                match field {
                    "name" => project.name = value,
                    "description" => project.description = value,
                    _ => {}
                }
            }
        });
        self.save();
    }

    pub fn update_environment(&self, field: &str, value: String) {
        let (Some(project_id), Some(environment_id)) = (
            self.selected_project.get_untracked(),
            self.selected_environment.get_untracked(),
        ) else { return; };
        self.projects.update(|projects| {
            if let Some(environment) = projects.iter_mut()
                .find(|project| project.id == project_id)
                .and_then(|project| project.environments.iter_mut().find(|env| env.id == environment_id))
            {
                match field {
                    "name" => environment.name = value,
                    "url" => environment.url = value,
                    "description" => environment.description = value,
                    _ => {}
                }
            }
        });
        self.save();
    }

    pub fn update_account(&self, field: &str, value: String) {
        let (Some(project_id), Some(environment_id), Some(account_id)) = (
            self.selected_project.get_untracked(),
            self.selected_environment.get_untracked(),
            self.selected_account.get_untracked(),
        ) else { return; };
        self.projects.update(|projects| {
            if let Some(account) = projects.iter_mut()
                .find(|project| project.id == project_id)
                .and_then(|project| project.environments.iter_mut().find(|env| env.id == environment_id))
                .and_then(|environment| environment.accounts.iter_mut().find(|account| account.id == account_id))
            {
                match field {
                    "name" => account.name = value,
                    "username" => account.username = value,
                    "password" => account.password = value,
                    "secret" => account.otp_secret = value,
                    "default_otp" => account.default_otp = value,
                    "notes" => account.notes = value,
                    "otp_mode" => account.otp_mode = match value.as_str() {
                        "totp" => OtpMode::Totp,
                        "default" => OtpMode::Default,
                        _ => OtpMode::None,
                    },
                    _ => {}
                }
            }
        });
        self.save();
    }

    pub fn delete_selected_project(&self) {
        let Some(id) = self.selected_project.get_untracked() else { return; };
        self.projects.update(|projects| projects.retain(|project| project.id != id));
        let next = self.projects.get_untracked().first().map(|project| project.id.clone());
        self.selected_project.set(None);
        self.selected_environment.set(None);
        self.selected_account.set(None);
        if let Some(id) = next { self.select_project(id); }
        self.save();
    }

    pub fn delete_selected_environment(&self) {
        let (Some(project_id), Some(environment_id)) = (
            self.selected_project.get_untracked(),
            self.selected_environment.get_untracked(),
        ) else { return; };
        self.projects.update(|projects| {
            if let Some(project) = projects.iter_mut().find(|project| project.id == project_id) {
                project.environments.retain(|environment| environment.id != environment_id);
            }
        });
        self.selected_environment.set(None);
        self.selected_account.set(None);
        self.save();
    }

    pub fn delete_selected_account(&self) {
        let (Some(project_id), Some(environment_id), Some(account_id)) = (
            self.selected_project.get_untracked(),
            self.selected_environment.get_untracked(),
            self.selected_account.get_untracked(),
        ) else { return; };
        self.projects.update(|projects| {
            if let Some(environment) = projects.iter_mut()
                .find(|project| project.id == project_id)
                .and_then(|project| project.environments.iter_mut().find(|env| env.id == environment_id))
            {
                environment.accounts.retain(|account| account.id != account_id);
            }
        });
        self.selected_account.set(None);
        self.save();
    }

    pub fn export_json(&self) {
        self.transfer_json.set(serde_json::to_string_pretty(&self.projects.get_untracked()).unwrap_or_default());
    }

    pub fn import_json(&self) {
        match serde_json::from_str::<Vec<Project>>(&self.transfer_json.get_untracked()) {
            Ok(projects) => {
                let project_id = projects.first().map(|project| project.id.clone());
                let environment_id = projects.first().and_then(|project| project.environments.first()).map(|env| env.id.clone());
                let account_id = projects.first().and_then(|project| project.environments.first()).and_then(|env| env.accounts.first()).map(|account| account.id.clone());
                self.projects.set(projects);
                self.selected_project.set(project_id);
                self.selected_environment.set(environment_id);
                self.selected_account.set(account_id);
                self.save();
                self.message.set(Some("Import completed.".into()));
            }
            Err(error) => self.message.set(Some(format!("Invalid JSON: {error}"))),
        }
    }
}
