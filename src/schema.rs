table! {
    demo (id) {
        id -> Int4,
        name -> Varchar,
        create_time -> Nullable<Timestamp>,
    }
}

table! {
    lawsuit_autocar (id) {
        id -> Int4,
        acid -> Nullable<Int4>,
        title -> Varchar,
        summary -> Varchar,
        list_img -> Nullable<Varchar>,
        visit -> Int8,
        price_base -> Money,
        current_price -> Money,
        assess_price -> Money,
        margin -> Money,
        recommended_price -> Money,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        recommend -> Int2,
        lng -> Nullable<Numeric>,
        lat -> Nullable<Numeric>,
        address -> Nullable<Varchar>,
        disposal_unit -> Nullable<Varchar>,
        external_url -> Nullable<Varchar>,
        belong -> Nullable<Int2>,
        stage -> Nullable<Varchar>,
        status -> Int2,
        show -> Nullable<Bool>,
        create_time -> Nullable<Timestamp>,
    }
}

table! {
    lawsuit_autocar_category (acid) {
        acid -> Int4,
        cname -> Nullable<Varchar>,
        parent_id -> Int4,
        level -> Nullable<Int2>,
        seo_title -> Nullable<Varchar>,
        seo_keywords -> Nullable<Varchar>,
        seo_description -> Nullable<Varchar>,
        order_by -> Int2,
        is_show -> Nullable<Bool>,
    }
}

table! {
    lawsuit_autocar_photo (lapid) {
        lapid -> Int4,
        laid -> Int4,
        external_small -> Nullable<Varchar>,
        external_middle -> Nullable<Varchar>,
        external_original -> Nullable<Varchar>,
        small -> Nullable<Varchar>,
        middle -> Nullable<Varchar>,
        original -> Nullable<Varchar>,
        path -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        extension -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        front_cover -> Nullable<Bool>,
        create_time -> Nullable<Timestamp>,
    }
}

table! {
    lawsuit_reptile (id) {
        id -> Int4,
        title -> Varchar,
        list_img -> Nullable<Varchar>,
        price_base -> Money,
        current_price -> Money,
        assess_price -> Money,
        margin -> Money,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        lng -> Nullable<Numeric>,
        lat -> Nullable<Numeric>,
        address -> Nullable<Varchar>,
        disposal_unit -> Nullable<Varchar>,
        external_url -> Nullable<Varchar>,
        belong -> Nullable<Int2>,
        stage -> Nullable<Varchar>,
        status -> Int2,
        push -> Nullable<Bool>,
        create_time -> Nullable<Timestamp>,
    }
}

table! {
    lawsuit_reptile_photo (lrpid) {
        lrpid -> Int4,
        lrid -> Int4,
        external_small -> Nullable<Varchar>,
        external_middle -> Nullable<Varchar>,
        external_original -> Nullable<Varchar>,
        front_cover -> Nullable<Bool>,
    }
}

table! {
    oauth_access_tokens (access_token) {
        access_token -> Bpchar,
        client_id -> Bpchar,
        user_id -> Int4,
        expires -> Nullable<Timestamp>,
        scope -> Nullable<Varchar>,
    }
}

table! {
    oauth_authorization_codes (authorization_code) {
        authorization_code -> Bpchar,
        client_id -> Bpchar,
        user_id -> Int4,
        redirect_uri -> Nullable<Varchar>,
        expires -> Nullable<Timestamp>,
        scope -> Nullable<Varchar>,
        id_token -> Nullable<Varchar>,
    }
}

table! {
    oauth_clients (client_id) {
        client_id -> Bpchar,
        client_secret -> Nullable<Varchar>,
        redirect_uri -> Nullable<Varchar>,
        grant_types -> Nullable<Varchar>,
        scope -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
    }
}

table! {
    oauth_jwt (client_id) {
        client_id -> Bpchar,
        subject -> Nullable<Varchar>,
        public_key -> Nullable<Varchar>,
    }
}

table! {
    oauth_refresh_tokens (refresh_token) {
        refresh_token -> Bpchar,
        client_id -> Bpchar,
        user_id -> Int4,
        expires -> Nullable<Timestamp>,
        scope -> Nullable<Varchar>,
    }
}

table! {
    oauth_scopes (scope) {
        scope -> Bpchar,
        is_default -> Nullable<Int2>,
    }
}

table! {
    oauth_users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        password -> Varchar,
        salt -> Nullable<Bpchar>,
        scope -> Nullable<Varchar>,
        create_time -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    demo,
    lawsuit_autocar,
    lawsuit_autocar_category,
    lawsuit_autocar_photo,
    lawsuit_reptile,
    lawsuit_reptile_photo,
    oauth_access_tokens,
    oauth_authorization_codes,
    oauth_clients,
    oauth_jwt,
    oauth_refresh_tokens,
    oauth_scopes,
    oauth_users,
);
