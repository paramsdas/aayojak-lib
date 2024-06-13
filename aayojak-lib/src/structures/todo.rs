use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::traits::typed::Typed;

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    // compulsary attributes
    title: String,
    id: i32,
    completion_status: bool,
    date_created: OffsetDateTime,
    date_modified: OffsetDateTime,
    // optional attributes
    date_deadline: Option<OffsetDateTime>,
    date_completed: Option<OffsetDateTime>,
    description: Option<String>,
}

impl Todo {
    // getters
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
    pub fn date_created(&self) -> &OffsetDateTime {
        &self.date_created
    }
    pub fn date_modified(&self) -> &OffsetDateTime {
        &self.date_modified
    }
    pub fn date_completed(&self) -> &Option<OffsetDateTime> {
        &self.date_completed
    }
    pub fn date_deadline(&self) -> &Option<OffsetDateTime> {
        &self.date_deadline
    }
    pub fn completion_status(&self) -> bool {
        self.completion_status
    }

    // setters
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
        self.update_date_modified();
    }
    pub fn set_description(&mut self, description: &str) {
        self.description = Some(String::from(description));
        self.update_date_modified();
    }
    pub fn set_date_completed(&mut self, date_completed: Option<OffsetDateTime>) {
        self.date_completed = date_completed;
    }
    pub fn set_date_deadline(&mut self, date_deadline: Option<OffsetDateTime>) {
        self.date_completed = date_deadline;
    }
    pub fn set_completion_status(&mut self, is_completed: bool) {
        self.completion_status = is_completed;
        self.update_date_modified();
    }

    // additional functions
    pub fn new(title: &str, id: i32) -> Self {
        let date_time = OffsetDateTime::now_utc();
        Todo {
            title: String::from(title),
            id,
            description: None,
            date_created: date_time,
            date_modified: date_time,
            date_deadline: None,
            date_completed: None,
            completion_status: false,
        }
    }
    pub fn toggle_completion_status(&mut self) {
        self.completion_status = !self.completion_status;
        self.update_date_modified();
    }

    // private functions
    fn update_date_modified(&mut self) {
        self.date_modified = OffsetDateTime::now_utc();
    }
}

impl Typed for Todo {
    fn get_type(&self) -> &str {
        return "Todo";
    }
}

impl ToString for Todo {
    fn to_string(&self) -> String {
        let formatted_string = format!(
            "\ttitle: {},\n\tdate_created: {},\n\tcompletion_status:{}",
            self.title(),
            self.date_created(),
            self.completion_status()
        );
        String::from(formatted_string)
    }
}
