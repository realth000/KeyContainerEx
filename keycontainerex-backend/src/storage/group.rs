use crate::storage::{collections::string_dictionary::StringDictionary, uuid::Uuid};
use chrono::{DateTime, Utc};
use std::os::unix::raw::mode_t;
use std::rc::Rc;

fn test() {
    let o: Vec<i32> = Vec::new();
}

#[derive(Debug)]
pub struct Group {
    default_auto_type_enabled: bool,
    default_search_enabled: bool,
    list_groups: Vec<Group>,
    list_entries: Vec<Group>,
    pub parent_group: Option<Rc<Group>>,
    pub parent_group_last_modified: DateTime<Utc>,

    pub name: String,
    pub notes: String,

    // icon;
    // pub icon id;
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub last_access_time: DateTime<Utc>,
    pub expire_time: DateTime<Utc>,
    pub is_expired: bool,
    pub usage_count: u64,

    pub is_expanded: bool,
    pub is_virtual: bool,

    pub default_auto_type_sequence: String,

    pub enable_auto_type: Option<bool>,
    pub enable_searching: Option<bool>,

    pub last_top_visible_entry: Uuid,

    custom_data: StringDictionary,

    // TODO: Implement GroupTouched here.
    // TODO: Implement Touched here.

    // Methods implemented in fields
    pub uuid: Uuid,
}

impl Default for Group {
    fn default() -> Self {
        let time = Utc::now();
        Group {
            default_auto_type_enabled: true,
            default_search_enabled: true,
            list_groups: Vec::new(),
            list_entries: Vec::new(),
            parent_group: None,
            parent_group_last_modified: time.clone(),

            name: String::new(),
            notes: String::new(),

            creation_time: time.clone(),
            last_modified_time: time.clone(),
            last_access_time: time.clone(),
            expire_time: time.clone(),
            is_expired: false,
            usage_count: 0,

            is_expanded: true,
            is_virtual: false,

            default_auto_type_sequence: String::new(),

            enable_auto_type: None,
            enable_searching: None,

            last_top_visible_entry: Uuid::new(false),
            custom_data: StringDictionary::new(),

            // Methods implemented in fields
            uuid: Uuid::new(false),
        }
    }
}

impl Group {
    // FIXME: Enable create_uuid and set_times
    pub fn new(create_uuid: bool, set_times: bool, name: Option<&str>) -> Self {
        let mut g = Group::default();
        match name {
            Some(s) => {
                g.name.clone_from(&String::from(s));
                g
            }
            None => g,
        }
    }

    pub fn groups(&self) -> &Vec<Group> {
        &self.list_groups
    }

    pub fn entries(&self) -> &Vec<Group> {
        &self.list_entries
    }

    pub fn custom_data(&self) -> &StringDictionary {
        &self.custom_data
    }

    pub fn touch(self, modified: bool) {
        self.touch_also_parent(modified, true);
    }

    pub fn touch_also_parent(mut self, modified: bool, touch_parent: bool) {
        self.last_access_time = Utc::now();
        self.usage_count += 1;
        if modified {
            self.last_modified_time = self.last_access_time;
        }
        // TODO: Implement this.Touched() here.
        // TODO: Use implemented PwGroup.GroupTouched here.
        if touch_parent {
            match self.parent_group {
                Some(p) => p.touch_also_parent(modified, true),
                None => {}
            }
        }
    }

    pub fn count(&self, recursive: bool) -> (usize, usize) {
        if recursive {
            let mut total_groups: usize = self.list_groups.len();
            let mut total_entries: usize = self.list_entries.len();
            let iter = self.list_groups.iter();
            for lg in iter {
                let (c1, c2) = lg.count(recursive);
                total_groups += c1;
                total_entries += c2;
            }
            (total_groups, total_entries)
        } else {
            (self.list_groups.len(), self.list_entries.len())
        }
    }

    pub fn count_entries(&self, recursive: bool) -> usize {
        let (_, total_entries) = self.count(recursive);
        total_entries
    }
}
