mod common;

use core_db::user::{
    User,
    errors::{AddNewUserError, GetUserError, GetUserIdError},
};

#[test_with::file(oppsy.db)]
#[tokio::test]
async fn user_roundtrip() {
    let db = common::init_db().await;

    let id = uuid::Uuid::now_v7();
    let email = format!("{id}@mail.com");
    db.add_new_user(id, email.clone()).await.unwrap();
    assert!(matches!(
        db.add_new_user(id, email.clone()).await.unwrap_err(),
        AddNewUserError::AlreadyExists { .. }
    ));

    let user_id = db.get_user_id(email.clone()).await.unwrap();
    assert_eq!(user_id, id);

    assert!(matches!(
        db.get_user_id("other@mail.com").await.unwrap_err(),
        GetUserIdError::NotFound(_)
    ));

    let user = db.get_user(id).await.unwrap();
    assert_eq!(user, User { id, email });

    assert!(matches!(
        db.get_user(uuid::Uuid::now_v7()).await.unwrap_err(),
        GetUserError::NotFound(_)
    ));
}
