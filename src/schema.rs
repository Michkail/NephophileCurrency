// @generated automatically by Diesel CLI.

diesel::table! {
    auth_group (id) {
        id -> Int4,
        #[max_length = 150]
        name -> Varchar,
    }
}

diesel::table! {
    auth_group_permissions (id) {
        id -> Int8,
        group_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    auth_permission (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        content_type_id -> Int4,
        #[max_length = 100]
        codename -> Varchar,
    }
}

diesel::table! {
    django_admin_log (id) {
        id -> Int4,
        action_time -> Timestamptz,
        object_id -> Nullable<Text>,
        #[max_length = 200]
        object_repr -> Varchar,
        action_flag -> Int2,
        change_message -> Text,
        content_type_id -> Nullable<Int4>,
        user_id -> Uuid,
    }
}

diesel::table! {
    django_content_type (id) {
        id -> Int4,
        #[max_length = 100]
        app_label -> Varchar,
        #[max_length = 100]
        model -> Varchar,
    }
}

diesel::table! {
    django_migrations (id) {
        id -> Int8,
        #[max_length = 255]
        app -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        applied -> Timestamptz,
    }
}

diesel::table! {
    django_session (session_key) {
        #[max_length = 40]
        session_key -> Varchar,
        session_data -> Text,
        expire_date -> Timestamptz,
    }
}

diesel::table! {
    todo (id) {
        id -> Int4,
        #[max_length = 1]
        description -> Nullable<Bpchar>,
        completed -> Nullable<Bool>,
        due_date -> Nullable<Date>,
    }
}

diesel::table! {
    user_user (user_id) {
        user_id -> Uuid,
        #[max_length = 254]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
        #[max_length = 15]
        phone_number -> Nullable<Varchar>,
        #[max_length = 100]
        photo_profile -> Nullable<Varchar>,
        is_client -> Bool,
        is_staff -> Bool,
        is_superuser -> Bool,
        is_active -> Bool,
        last_login -> Nullable<Timestamptz>,
        date_updated -> Timestamptz,
        date_joined -> Date,
    }
}

diesel::table! {
    user_user_groups (id) {
        id -> Int8,
        user_id -> Uuid,
        group_id -> Int4,
    }
}

diesel::table! {
    user_user_user_permissions (id) {
        id -> Int8,
        user_id -> Uuid,
        permission_id -> Int4,
    }
}

diesel::joinable!(auth_group_permissions -> auth_group (group_id));
diesel::joinable!(auth_group_permissions -> auth_permission (permission_id));
diesel::joinable!(auth_permission -> django_content_type (content_type_id));
diesel::joinable!(django_admin_log -> django_content_type (content_type_id));
diesel::joinable!(django_admin_log -> user_user (user_id));
diesel::joinable!(user_user_groups -> auth_group (group_id));
diesel::joinable!(user_user_groups -> user_user (user_id));
diesel::joinable!(user_user_user_permissions -> auth_permission (permission_id));
diesel::joinable!(user_user_user_permissions -> user_user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_group,
    auth_group_permissions,
    auth_permission,
    django_admin_log,
    django_content_type,
    django_migrations,
    django_session,
    todo,
    user_user,
    user_user_groups,
    user_user_user_permissions,
);
