use chrono::{DateTime, Utc};
use sea_orm::{prelude::*, sea_query::Mode, IntoActiveModel};

#[derive(DeriveEntityModel, Clone, Debug, PartialEq)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: String,
    name: String,
    content: String,
    state: String,
    done: bool,
    created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(name: String, content: String, state: String) -> Self {
        Model {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            content,
            state,
            done: false,
            created_at: Utc::now(),
        }
    }
    // Getters
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn state(&self) -> &String {
        &self.state
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    // Setters
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }

    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }
}
