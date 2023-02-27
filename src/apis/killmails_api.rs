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

pub struct KillmailsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> KillmailsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> KillmailsApiClient<C> {
        KillmailsApiClient {
            configuration: configuration,
        }
    }
}

pub trait KillmailsApi {
    fn get_characters_character_id_killmails_recent(&self, character_id: i32, datasource: &str, if_none_match: &str, page: i32, token: &str) -> Box<Future<Item = Vec<::models::GetCharactersCharacterIdKillmailsRecent200Ok>, Error = Error<serde_json::Value>>>;
    fn get_corporations_corporation_id_killmails_recent(&self, corporation_id: i32, datasource: &str, if_none_match: &str, page: i32, token: &str) -> Box<Future<Item = Vec<::models::GetCorporationsCorporationIdKillmailsRecent200Ok>, Error = Error<serde_json::Value>>>;
    fn get_killmails_killmail_id_killmail_hash(&self, killmail_hash: &str, killmail_id: i32, datasource: &str, if_none_match: &str) -> Box<Future<Item = ::models::GetKillmailsKillmailIdKillmailHashOk, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>KillmailsApi for KillmailsApiClient<C> {
    fn get_characters_character_id_killmails_recent(&self, character_id: i32, datasource: &str, if_none_match: &str, page: i32, token: &str) -> Box<Future<Item = Vec<::models::GetCharactersCharacterIdKillmailsRecent200Ok>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref token) = configuration.oauth_access_token {
            let auth = hyper::header::Authorization(
                hyper::header::Bearer {
                    token: token.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("datasource", &datasource.to_string());
            query.append_pair("page", &page.to_string());
            query.append_pair("token", &token.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/characters/{character_id}/killmails/recent/?{}", configuration.base_path, query_string, character_id=character_id);

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

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
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
                let parsed: Result<Vec<::models::GetCharactersCharacterIdKillmailsRecent200Ok>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_corporations_corporation_id_killmails_recent(&self, corporation_id: i32, datasource: &str, if_none_match: &str, page: i32, token: &str) -> Box<Future<Item = Vec<::models::GetCorporationsCorporationIdKillmailsRecent200Ok>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref token) = configuration.oauth_access_token {
            let auth = hyper::header::Authorization(
                hyper::header::Bearer {
                    token: token.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("datasource", &datasource.to_string());
            query.append_pair("page", &page.to_string());
            query.append_pair("token", &token.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/corporations/{corporation_id}/killmails/recent/?{}", configuration.base_path, query_string, corporation_id=corporation_id);

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

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
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
                let parsed: Result<Vec<::models::GetCorporationsCorporationIdKillmailsRecent200Ok>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_killmails_killmail_id_killmail_hash(&self, killmail_hash: &str, killmail_id: i32, datasource: &str, if_none_match: &str) -> Box<Future<Item = ::models::GetKillmailsKillmailIdKillmailHashOk, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("datasource", &datasource.to_string());
            query.finish()
        };
        let uri_str = format!("{}/killmails/{killmail_id}/{killmail_hash}/?{}", configuration.base_path, query_string, killmail_hash=killmail_hash, killmail_id=killmail_id);

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
                let parsed: Result<::models::GetKillmailsKillmailIdKillmailHashOk, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
