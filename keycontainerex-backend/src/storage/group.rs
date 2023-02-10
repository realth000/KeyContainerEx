use crate::storage::{
    collections::string_dictionary::StringDictionary, serialization::object_list::ObjectList,
    uuid::uuid,
};
use std::rc::Rc;
use std::time::SystemTime;

fn test() {
    let o: ObjectList<i32> = ObjectList::new();
}

#[derive(Debug)]
pub struct Group {
    default_auto_type_enabled: bool,
    default_search_enabled: bool,
    list_groups: ObjectList<Group>,
    list_entries: ObjectList<Group>,
    parent_group: Option<Rc<Group>>,
    parent_group_last_modified: SystemTime,

    name: String,
    notes: String,

    // icon;
    // icon id;
    creation: SystemTime,
    last_modified: SystemTime,
    last_access: SystemTime,
    expire: SystemTime,
    is_expired: bool,
    usage_count: u64,

    is_expanded: bool,
    is_virtual: bool,

    default_auto_type_sequence: String,

    enable_auto_type: Option<bool>,
    enable_search: Option<bool>,

    last_top_visible_entry: uuid,

    custom_data: StringDictionary,
}

impl Default for Group {
    fn default() -> Self {
        Group {
            default_auto_type_enabled: true,
            default_search_enabled: true,
            list_groups: ObjectList::new(),
            list_entries: ObjectList::new(),
            parent_group: None,
            parent_group_last_modified: SystemTime::now(),

            name: String::new(),
            notes: String::new(),

            creation: SystemTime::now(),
            last_modified: SystemTime::now(),
            last_access: SystemTime::now(),
            expire: SystemTime::now(),
            is_expired: false,
            usage_count: 0,

            is_expanded: true,
            is_virtual: false,

            default_auto_type_sequence: String::new(),

            enable_auto_type: None,
            enable_search: None,

            last_top_visible_entry: uuid::new(false),
            custom_data: StringDictionary::new(),
        }
    }
}
