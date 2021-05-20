use reqwest::{
    blocking::{Client, Response},
    header::{AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use serde::Serialize;
use serde_urlencoded::*;
use std::fmt::Debug;

pub(crate) enum REQUEST {
    POST,
    PUT,
    DELETE,
}

pub(crate) fn make_get_request<T>(
    bearer_auth: String,
    url: String,
    queries: Option<T>,
) -> Result<Response, String>
where
    T: Serialize + Debug,
{
    let reqwest_client = Client::new();
    let formatted_err_msg = format!(
        "[PAYSTACK ERROR]: Error making GET request to url: {}",
        url.to_string()
    );
    let queries = &ser::to_string(queries).unwrap();
    let full_url = format!("{}?{}", url, queries);
    let res = reqwest_client
        .get(full_url)
        .header(AUTHORIZATION, bearer_auth.to_string())
        .send()
        .expect(formatted_err_msg.as_str());

    match res.status() {
        StatusCode::OK => return Ok(res),
        StatusCode::BAD_REQUEST => return Err("Bad request. Please check the body".to_string()),
        StatusCode::INTERNAL_SERVER_ERROR => {
            return Err("An error occured on the paystack server: please try again".to_string())
        }
        _ => {
            println!("Response from google is: {:?}", res);
            return Ok(res);
        }
    }
}

pub(crate) fn make_request<T>(
    bearer_auth: String,
    url: String,
    body: Option<T>,
    verb: REQUEST,
) -> Result<Response, String>
where
    T: Debug + Serialize,
{
    let reqwest_client = Client::new();
    let formatted_err_msg = format!(
        "[PAYSTACK ERROR]: Error making POST request to paystack with URL: {} and body: {:?}",
        url, body
    );

    let serialized_body =
        serde_json::to_string(&body).expect("Error serializing POST request body");
    let builder = match verb {
        REQUEST::POST => reqwest_client.post(url),
        REQUEST::DELETE => reqwest_client.delete(url),
        REQUEST::PUT => reqwest_client.put(url),
    };

    let res = builder
        .header(AUTHORIZATION, bearer_auth)
        .header(CONTENT_TYPE, "application/json".to_string())
        .body(serialized_body)
        .send()
        .expect(formatted_err_msg.as_str());

    match res.status() {
        StatusCode::UNAUTHORIZED => {
            println!("Oops! Unauthorized request. Please ensure you've set the correct headers");
            return Err("Unauthorized request. please check header values".to_string());
        }
        StatusCode::BAD_REQUEST => {
            return Err(
                "Bad request. Please check whatever you're passing in the request. Seems broken"
                    .to_string(),
            )
        }
        StatusCode::OK => {
            println!("Yay!! you got it!!");
            return Ok(res);
        }
        _ => {
            // the below is meant as a light joke.. chill out pls
            println!("Dunno... Looks Ok but since its not an error i specially check for, here is your result, man... or woman... or they/them");
            return Ok(res);
        }
    };
}
