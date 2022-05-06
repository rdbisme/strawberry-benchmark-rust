use juniper::{execute, EmptyMutation, RootNode, Variables};
use strwb::{create_people, Database, Query};

fn main() {
    let database = Database {
        people: create_people(100000),
    };
    let schema = RootNode::new(Query {}, EmptyMutation::<Database>::new());
    let doc = r#"{
            people {
                age
                description
                address
                name
                propA
                propB
                propC
                propD
                propE
                propF
                propG
                propH
                propI
                propJ
            }
        }"#;

    execute(doc, None, &schema, &Variables::new(), &database).unwrap();
}
