table! {
    model (id) {
        id -> Integer,
        routine -> Integer,
    }
}

table! {
    routine (id) {
        id -> Integer,
        name -> Text,
        model -> Integer,
    }
}

table! {
    task (id) {
        id -> Integer,
        name -> Text,
        state -> Text,
    }
}

table! {
    task_partof_model (id) {
        id -> Integer,
        model -> Integer,
        task -> Integer,
    }
}

table! {
    task_partof_regular (id) {
        id -> Integer,
        regular -> Integer,
        task -> Integer,
    }
}

table! {
    tasklist (id) {
        id -> Integer,
        name -> Text,
        state -> Text,
        routine_id -> Integer,
        archived -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    model,
    routine,
    task,
    task_partof_model,
    task_partof_regular,
    tasklist,
);
