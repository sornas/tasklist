table! {
    models (id) {
        id -> Integer,
    }
}

table! {
    routines (id) {
        id -> Integer,
        name -> Text,
        model -> Integer,
    }
}

table! {
    tasklist_partof (id) {
        id -> Integer,
        tasklist -> Integer,
        task -> Integer,
    }
}

table! {
    tasklists (id) {
        id -> Integer,
        name -> Text,
        done -> Bool,
        belongs_to -> Integer,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        name -> Text,
        done -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(models, routines, tasklist_partof, tasklists, tasks,);
