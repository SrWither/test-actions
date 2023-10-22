use salvo::prelude::*;
use surrealdb::opt::auth::Scope;
use crate::models::auth::{SignupParams, SigninParams};

use super::super::database::get_db;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Register {
    name: String,
    email: String,
    password: String
}

#[derive(Serialize, Deserialize)]
struct Login {
    email: String,
    password: String
}

#[handler]
pub async fn register(req: &mut Request, res: &mut Response) {
    let reqbody = req.parse_json::<Register>().await;
    let user = match reqbody {
        Ok(request_body) => request_body,

        Err(_) => panic!("error al paresar el request body")
    };

    let db = get_db().await.unwrap();

    let signup = db.signup(Scope {
        namespace: "blog",
        database: "blog",
        scope: "user",
        params: SignupParams {
            name: user.name.as_str(),
            email: user.email.as_str(),
            password: user.password.as_str()
        }
    }).await;

    match signup {
        Ok(token) => res.render(Json(token.as_insecure_token())),
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(err));
        }
    }
}

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    let reqbody = req.parse_json::<Login>().await;
    let user = match reqbody {
        Ok(request_body) => request_body,

        Err(_) => panic!("error al paresar el request body")
    };

    let db = get_db().await.unwrap();

    let signin = db.signin(Scope {
        namespace: "blog",
        database: "blog",
        scope: "user",
        params: SigninParams {
            email: user.email.as_str(),
            password: user.password.as_str()
        }
    }).await;

    match signin {
        Ok(token) => res.render(Json(token.as_insecure_token())),
        Err(err) => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(err));
        }
    }

}
