use time::OffsetDateTime;

pub struct Todo {
    title: String,
    description: String,
    date_created: OffsetDateTime,
    date_modified: OffsetDateTime,
    date_deadline: Option<OffsetDateTime>,
    date_completed: Option<OffsetDateTime>,
    is_completed: bool,
}

impl Todo {
    // getters
    pub fn get_title(&self) -> &str {
        return &self.title;
    }
    pub fn get_description(&self) -> &str {
        return &self.description;
    }
    pub fn get_date_created(&self) -> &OffsetDateTime {
        return &self.date_created;
    }
    pub fn get_date_modified(&self) -> &OffsetDateTime {
        return &self.date_modified;
    }
    pub fn get_date_completed(&self) -> &Option<OffsetDateTime> {
        return &self.date_completed;
    }
    pub fn get_date_deadline(&self) -> &Option<OffsetDateTime> {
        return &self.date_deadline;
    }
    pub fn is_completed(&self) -> bool {
        return self.is_completed;
    }

    // setters
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
        self.update_date_modified();
    }
    pub fn set_description(&mut self, description: &str) {
        self.description = String::from(description);
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
    pub fn new(title: &str) -> Self {
        Todo {
            title: String::from(title),
            description: String::from(""),
            date_created: Self::get_current_local_time(),
            date_modified: Self::get_current_local_time(),
            date_deadline: None,
            date_completed: None,
            is_completed: false,
        }
    }
    pub fn toggle_is_completed(&mut self) {
        self.is_completed = !self.is_completed;
        self.update_date_modified();
    }

    // private functions
    fn update_date_modified(&mut self) {
        self.date_modified = Self::get_current_local_time();
    }

    fn get_current_local_time() -> OffsetDateTime {
        OffsetDateTime::now_local().unwrap_or_else(|err| {
            panic!("Error getting current local time: {:?}", err);
        })
    }
}
