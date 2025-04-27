use application::dto::CreateUserDto;
use application::service::UserService;
use domain::user::User;
use std::sync::Arc;

#[cfg(test)]
use infrastructure::tests::persistence_test::TestUserRepository;

#[tokio::test]
async fn test_create_user() {
    let repo = TestUserRepository::new(());
    let service = UserService::new(Arc::new(repo));
    let dto = CreateUserDto {
        name: "Test User".to_string(),
    };
    let result = service.create_user(dto).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_user() {
    let repo = TestUserRepository::new(());
    repo.seed(vec![User {
        id: uuid::Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
        name: "Test User".to_string(),
        created_at: chrono::Utc::now(),
    }]);
    let service = UserService::new(Arc::new(repo));

    let user_id = uuid::Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    let result = service.get_user(user_id).await;

    assert!(result.is_ok());
}
