#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error(transparent)]
    ServiceFailure(anyhow::Error)
}

#[allow(unused)]
#[derive(Debug)]
pub struct UserDto {
    id: String,
    name: String
}

#[allow(unused)]
#[derive(Debug)]
pub struct CreateUserDto {
    name: String
}

#[async_trait::async_trait]
#[orbital::export_service]
pub trait CreateUserService: 'static + Sync + Send {
    async fn create(&self, create: CreateUserDto) -> Result<UserDto, ErrorKind>;
}