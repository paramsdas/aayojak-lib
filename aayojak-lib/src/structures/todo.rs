use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::traits::typed::Typed;

/// Todo is a structure which wraps all relevant information about a todo-task.
///
/// All time related structs/instances work with UTC.
#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    // compulsary attributes
    title: String,
    id: i32,
    completion_status: bool,
    date_created: DateTime<Utc>,
    date_modified: DateTime<Utc>,
    // optional attributes
    description: Option<String>,
    date_completed: Option<DateTime<Utc>>,
    date_deadline: Option<DateTime<Utc>>,
}

impl Todo {
    // getters
    /// get todo title
    pub fn title(&self) -> &str {
        &self.title
    }
    /// get todo id
    pub fn id(&self) -> &i32 {
        &self.id
    }
    /// get todo description
    ///
    /// # returns:
    ///  - ```true```, if completed
    ///  - ```false```, otherwise
    pub fn completion_status(&self) -> bool {
        self.completion_status
    }
    /// get the creation date
    pub fn date_created(&self) -> &DateTime<Utc> {
        &self.date_created
    }
    /// get the last modified date
    pub fn date_modified(&self) -> &DateTime<Utc> {
        &self.date_modified
    }
    /// get todo description, if set
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
    /// get the completion date, if set
    pub fn date_completed(&self) -> &Option<DateTime<Utc>> {
        &self.date_completed
    }
    /// get the deadline date, if set
    pub fn date_deadline(&self) -> &Option<DateTime<Utc>> {
        &self.date_deadline
    }

    // setters
    /// set todo title
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
        self.update_date_modified();
    }
    /// set todo description
    pub fn set_description(&mut self, description: &str) {
        self.description = Some(String::from(description));
        self.update_date_modified();
    }
    /// set the completion date
    pub fn set_date_completed(&mut self, date_completed: Option<DateTime<Utc>>) {
        self.date_completed = date_completed;
    }
    /// set the deadline date
    pub fn set_date_deadline(&mut self, date_deadline: Option<DateTime<Utc>>) {
        self.date_completed = date_deadline;
    }
    /// set the completion status
    pub fn set_completion_status(&mut self, is_completed: bool) {
        self.completion_status = is_completed;
        self.update_date_modified();
    }

    // additional functions
    /// Create a new Todo instance. Requires a title and an id.
    /// # Examples
    ///
    /// ```
    /// use aayojak_lib::structures::todo::Todo;
    /// let todo = Todo::new("Test", 1);
    ///
    /// assert_eq!(todo.title(), "Test");
    /// assert_eq!(*todo.id(), 1 as i32);
    /// ```
    ///
    pub fn new(title: &str, id: i32) -> Self {
        let date_time = Utc::now();
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
    /// toggle the completion status completion date depending on current status
    ///
    /// ```update_date_completed```, if true, will update the completion date
    /// if completion status will be toggled to true and will delete the
    /// completion date if completion status will be toggled to false
    pub fn toggle_completion_status(&mut self, update_date_completed: bool) {
        if update_date_completed {
            match self.completion_status {
                true => self.set_date_completed(None),
                false => self.set_date_completed(Some(Utc::now())),
            }
        }
        self.completion_status = !self.completion_status;
        self.update_date_modified();
    }

    // private functions
    /// function which updates the modification date to current time (UTC)
    fn update_date_modified(&mut self) {
        self.date_modified = Utc::now();
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
