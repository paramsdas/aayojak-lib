use time::{error::IndeterminateOffset, OffsetDateTime};

pub struct Todo {
    // compulsary attributes
    title: String,
    is_completed: bool,
    date_created: OffsetDateTime,
    date_modified: OffsetDateTime,
    // optional attributes
    date_deadline: Option<OffsetDateTime>,
    date_completed: Option<OffsetDateTime>,
    description: Option<String>,
}

impl Todo {
    // getters
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_description(&self) -> &Option<String> {
        &self.description
    }
    pub fn get_date_created(&self) -> &OffsetDateTime {
        &self.date_created
    }
    pub fn get_date_modified(&self) -> &OffsetDateTime {
        &self.date_modified
    }
    pub fn get_date_completed(&self) -> &Option<OffsetDateTime> {
        &self.date_completed
    }
    pub fn get_date_deadline(&self) -> &Option<OffsetDateTime> {
        &self.date_deadline
    }
    pub fn is_completed(&self) -> bool {
        self.is_completed
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
    pub fn set_completed(&mut self, is_completed: bool) {
        self.is_completed = is_completed;
        self.update_date_modified();
    }

    // additional functions
    pub fn new(title: &str) -> Result<Self, IndeterminateOffset> {
        let current_time = Self::get_current_local_time();
        match current_time {
            Ok(current_time_extracted) => Ok(Todo {
                title: String::from(title),
                description: None,
                date_created: current_time_extracted,
                date_modified: current_time_extracted,
                date_deadline: None,
                date_completed: None,
                is_completed: false,
            }),
            Err(err) => Err(err),
        }
    }
    pub fn toggle_is_completed(&mut self) {
        self.is_completed = !self.is_completed;
        self.update_date_modified();
    }

    // private functions
    fn update_date_modified(&mut self) {
        let current_time = Self::get_current_local_time();
        match current_time {
            Ok(x) => self.date_modified = x,
            Err(err) => println!("Could not get current timestamp: {}", err),
        }
    }

    fn get_current_local_time() -> Result<OffsetDateTime, IndeterminateOffset> {
        OffsetDateTime::now_local()
    }
}
