use time::OffsetDateTime;

pub struct Todo {
    title: String,
    description: String,
    date_created: OffsetDateTime,
    date_modified: OffsetDateTime,
    date_completed: OffsetDateTime,
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
    pub fn get_date_completed(&self) -> &OffsetDateTime {
        return &self.date_completed;
    }
    pub fn is_completed(&self) -> bool {
        return self.is_completed;
    }

    // setters
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
        self.date_modified = OffsetDateTime::now_local().unwrap();
    }
    pub fn set_description(&mut self, description: &str) {
        self.description = String::from(description);
        self.date_modified = OffsetDateTime::now_local().unwrap();
    }
    pub fn set_date_completed(&mut self, date_completed: &OffsetDateTime) {
        self.date_completed = *date_completed;
        self.date_modified = OffsetDateTime::now_local().unwrap();
    }
    pub fn set_completed(&mut self, is_completed: bool) {
        self.is_completed = is_completed;
        self.date_modified = OffsetDateTime::now_local().unwrap();
    }

    // additional functions
    pub fn toggle_is_completed(&mut self) {
        self.is_completed = !self.is_completed;
        self.date_modified = OffsetDateTime::now_local().unwrap();
    }
}
