use uuid::Uuid;

pub struct Person {
    guid: String,
    first_name: String,
    last_name: String,
}

pub struct Task {
    guid: String
}

pub struct Role {
    guid: String
}

pub fn build_person(first_name: String, last_name: String) -> Person {
    Person {
        guid: Uuid::new_v4().to_string(),
        first_name,
        last_name,
    }
}

pub fn build_task() -> Task {
    Task {
        guid: Uuid::new_v4().to_string(), 
    }
}

pub fn build_role() -> Role {
    Role {
        guid: Uuid::new_v4().to_string(), 
    }
}