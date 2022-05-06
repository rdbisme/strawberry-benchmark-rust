use juniper::{graphql_object, GraphQLObject};

#[derive(GraphQLObject, Clone)]
pub struct Person {
    name: String,
    age: i32,
    description: String,
    address: String,

    prop_a: String,
    prop_b: String,
    prop_c: String,
    prop_d: String,
    prop_e: String,
    prop_f: String,
    prop_g: String,
    prop_h: String,
    prop_i: String,
    prop_j: String,
}

pub fn create_people(n: i32) -> Vec<Person> {
    let mut people = Vec::new();
    for i in 0..n {
        let person = Person {
            name: format!("Person {}", i),
            age: i,
            description: format!("Description {}", i),
            address: format!("Address {}", i),

            prop_a: format!("Prop X {}", i),
            prop_b: format!("Prop X {}", i),
            prop_c: format!("Prop X {}", i),
            prop_d: format!("Prop X {}", i),
            prop_e: format!("Prop X {}", i),
            prop_f: format!("Prop X {}", i),
            prop_g: format!("Prop X {}", i),
            prop_h: format!("Prop X {}", i),
            prop_i: format!("Prop X {}", i),
            prop_j: format!("Prop X {}", i),
        };

        people.push(person);
    }
    return people;
}

pub struct Database {
    pub people: Vec<Person>,
}
impl juniper::Context for Database {}

pub struct Query {}

graphql_object!(
    Query: Database | &self | {
        field hello () -> &str {
            "Hello World!"
        }

        field people(&executor) -> Vec<Person> {
            let db = &executor.context();
            return db.people.clone();
        }
    }
);
