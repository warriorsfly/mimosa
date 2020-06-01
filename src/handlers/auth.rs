use crate::{
    auth::{create_jwt, JwtAccount},
    config::CONFIG,
    database::PoolType,
    errors::ServiceError,
    helpers::respond_json,
    models::user::{Profile, User},
};
use actix_web::{
    self,
    web::{Data, Json},
};
use validator::Validate;

#[derive(Deserialize, Validate)]
struct NewUserDto {
    #[validate(length(min = 1))]
    username: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 8))]
    password: Option<String>,
}

// pub async fn register(pool: Data<PoolType>, dto: Json<NewUserDto>) {
//     // await User::register()
// }
// #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
// pub struct CertRequest {
//     pub username: String,

//     #[validate(length(
//         min = 6,
//         max = 16,
//         message = "certificate is required and must be at least 6 characters,at most 16 characters"
//     ))]
//     pub password: String,

//     pub identity_type: i32,
// }

// /// 登录返回数据
// #[derive(Debug, Deserialize, Serialize)]
// pub struct LoginResponse {
//     pub access_token: String,
//     pub user: Profile,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct WxSessionResponse {
//     pub openid: Option<String>,
//     pub session_key: Option<String>,
//     pub unionid: Option<String>,
//     pub errcode: Option<i32>,
//     pub errmsg: Option<String>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct WxUserInfoResponse {
//     /// 昵称
//     pub nickName: String,
//     /// 头像地址
//     pub avatarUrl: String,
//     /// 性别
//     pub gender: i32,
//     pub country: String,
//     pub province: String,
//     pub city: String,
// }

// /// 微信小程序登录接口
// pub async fn wx_login(
//     // id: Identity,
//     pool: Data<PoolType>,
//     jscode: Path<String>,
//     client: Data<Client>,
// ) -> Result<Json<LoginResponse>, ServiceError> {
//     let url =format!("https://api.weixin.qq.com/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",appid=&CONFIG.wechat_appid,secret=&CONFIG.wechat_secret,code=jscode);
//     let body = client
//         .get(url)
//         .send()
//         .await
//         .map_err(|err| ServiceError::BadRequest(err.to_string()))?
//         .body()
//         .await
//         .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

//     let res: WxSessionResponse =
//         serde_json::from_slice(&body).map_err(|err| ServiceError::BadRequest(err.to_string()))?;

//     // response errorcode==None means  the right openid and session_key
//     // find in redis session,if exists,return it
//     // if doesn't exist,query entity in user_auth,where identity_type=5 and identifier=res.openid
//     // if no result,insert one,identity_type=5 and identifier=res.openid,login_session=res.session_key,
//     //   insert one in user_base
//     //return user_auth entity,create jwt,create redis session
//     if let Some(ident) = res.openid {
//         let res = find_by_3rd_account(&pool, &ident, 3)?;
//         let uid =
//             Uuid::parse_str(&res.0.uid).map_err(|err| ServiceError::UuidError(err.to_string()))?;
//         let identifier = res.0.identifier.clone();
//         let jwt = create_jwt(PrivateClaim::new(uid, identifier, 3))?;
//         // id.remember(jwt);
//         respond_json(LoginResponse {
//             access_token: jwt,
//             user: res.1,
//         })
//     } else {
//         Err(ServiceError::BadRequest(res.errmsg.unwrap()))
//     }
// }

pub fn login(pool: &PoolType, user_name: &str, password: &str) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;
    let not_found = format!("User {} not found", user_name);
    let user = users
        .filter(username.eq(user_name))
        .first::<User>(&conn)
        .map_err(|_| ServiceError::NotFound(not_found))?;

    Ok(user)
}
