/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{Error, configuration};

pub struct WarsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> WarsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> WarsApiClient<C> {
        WarsApiClient {
            configuration: configuration,
        }
    }
}

pub trait WarsApi {
    fn get_wars(&self, datasource: &str, if_none_match: &str, max_war_id: i32) -> Box<Future<Item = Vec<i32>, Error = Error<serde_json::Value>>>;
    fn get_wars_war_id(&self, war_id: i32, datasource: &str, if_none_match: &str) -> Box<Future<Item = ::models::GetWarsWarIdOk, Error = Error<serde_json::Value>>>;
    fn get_wars_war_id_killmails(&self, war_id: i32, datasource: &str, if_none_match: &str, page: i32) -> Box<Future<Item = Vec<::models::GetWarsWarIdKillmails200Ok>, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>WarsApi for WarsApiClient<C> {
    fn get_wars(&self, datasource: &str, if_none_match: &str, max_war_id: i32) -> Box<Future<Item = Vec<i32>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("datasource", &datasource.to_string());
            query.append_pair("max_war_id", &max_war_id.to_string());
            query.finish()
        };
        let uri_str = format!("{}/wars/?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("If-None-Match", if_none_match);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<Vec<i32>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_wars_war_id(&self, war_id: i32, datasource: &str, if_none_match: &str) -> Box<Future<Item = ::models::GetWarsWarIdOk, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("datasource", &datasource.to_string());
            query.finish()
        };
        let uri_str = format!("{}/wars/{war_id}/?{}", configuration.base_path, query_string, war_id=war_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("If-None-Match", if_none_match);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::GetWarsWarIdOk, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_wars_war_id_killmails(&self, war_id: i32, datasource: &str, if_none_match: &str, page: i32) -> Box<Future<Item = Vec<::models::GetWarsWarIdKillmails200Ok>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("datasource", &datasource.to_string());
            query.append_pair("page", &page.to_string());
            query.finish()
        };
        let uri_str = format!("{}/wars/{war_id}/killmails/?{}", configuration.base_path, query_string, war_id=war_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("If-None-Match", if_none_match);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<Vec<::models::GetWarsWarIdKillmails200Ok>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
