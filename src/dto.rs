// 提交给服务器的数据结构
#[derive(Validate, Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegisterUserDto {
    #[validate(length(min = 2, message = "用户名长度不能小于2位"))]
    pub name: String,
    #[validate(length(min = 2, message = "邮箱长度不能小于2位"), email(message = "邮箱格式不正确"))]
    pub email: String,
    #[validate(length(min = 6, message = "密码长度不能小于6位"))]
    pub password: String,
    #[validate(length(min = 6, message = "确认密码长度不能小于6位"), must_match(other = "password", message = "两次输入的密码不一致"))]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

#[derive(Validate, Debug, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    #[validate(length(min = 2, message = "邮箱长度不能小于2位"), email(message = "邮箱格式不正确"))]
    pub email: String,
    #[validate(length(min = 6, message = "密码长度不能小于6位"))]
    pub password: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct RequestUserDto {
    #[validate(range(min = 1, message = "页码不能小于1"))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            role: user.role.to_str().to_string(),
            verified: user.verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }

    pub fn filter_users(users: &[User]) -> Vec<FilterUserDto> {
        users.iter().map(FilterUserDto::filter_user).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub status: String,
    pub users: Vec<FilterUserDto>,
    pub result: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Validate, Debug, Serialize, Deserialize, Default, Clone)]
pub struct NameUpdateDto {
    #[validate(length(min = 2, message = "用户名长度不能小于2位"))]
    pub name: String,
}

#[derive(Validate, Debug, Serialize, Deserialize, Default, Clone)]
pub struct RoleUpdateDto {
    #[validate(custom = "用户角色")]
    pub role: UserRole,
}

fn validate_user_role(role: &UserRole) -> Result<(), validator::ValidationError> {
    match role {
        UserRole::Admin | UserRole::User => Ok(()),
        _ => Err(validator::ValidationError::new("用户角色不合法")),
    }

}

#[derive(Validate, Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserPasswordUpdateDto {
    #[validate(length(min = 6, message = "密码长度不能小于6位"))]
    pub new_password: String,
    #[validate(length(min = 6, message = "确认密码长度不能小于6位"), must_match(other = "new_password", message = "两次输入的密码不一致"))]
    pub new_password_confirm: String,
    #[validate(length(min = 6, message = "旧密码长度不能小于6位"))]
    pub old_password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct VerifyEmailQueryDto {
    #[validate(length(min = 1, message = "token不能为空"))]
    pub token: String,
}

#[derive(Deserialize, Validate, Serialize, Debug, Clone)]
pub struct ForgotPasswordRequestDto {
    #[validate(length(min = 2, message = "邮箱长度不能小于2位"), email(message = "邮箱格式不正确"))]
    pub email: String,
}

#[derive(Deserialize, Validate, Serialize, Debug, Clone)]
pub struct ResetPasswordRequestDto {
    #[validate(length(min = 1, message = "token不能为空"))]
    pub token: String,
    #[validate(length(min = 6, message = "密码长度不能小于6位"))]
    pub new_password: String,
    #[validate(length(min = 6, message = "确认密码长度不能小于6位"), must_match(other = "new_password", message = "两次输入的密码不一致"))]
    pub new_password_confirm: String,
}