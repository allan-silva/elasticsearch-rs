/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
// -----------------------------------------------
// This file is generated, Please do not edit it manually.
// Run the following in the root of the repo to regenerate:
//
// cargo run -p api_generator
// -----------------------------------------------
#![allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        transport::Transport,
        Method,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Close Job API"]
pub enum MlCloseJobParts<'b> {
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlCloseJobParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Close Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlCloseJobParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/_close");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Close Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-close-job.html)\n\nCloses one or more anomaly detection jobs. A job can be opened and closed multiple times throughout its lifecycle."]
pub struct MlCloseJob<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlCloseJobParts<'b>,
    allow_no_jobs: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    force: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> MlCloseJob<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlCloseJob] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlCloseJobParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlCloseJob {
            transport,
            parts,
            headers,
            allow_no_jobs: None,
            body: None,
            error_trace: None,
            filter_path: None,
            force: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: bool) -> Self {
        self.allow_no_jobs = Some(allow_no_jobs);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlCloseJob<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlCloseJob {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_jobs: self.allow_no_jobs,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            force: self.force,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "True if the job should be forcefully closed"]
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait until a job has closed. Default to 30 minutes"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Close Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_jobs")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_jobs: self.allow_no_jobs,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                force: self.force,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Delete Calendar API"]
pub enum MlDeleteCalendarParts<'b> {
    #[doc = "CalendarId"]
    CalendarId(&'b str),
}
impl<'b> MlDeleteCalendarParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Calendar API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteCalendarParts::CalendarId(ref calendar_id) => {
                let encoded_calendar_id: Cow<str> =
                    percent_encode(calendar_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_calendar_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(encoded_calendar_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Delete Calendar API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-calendar.html)\n\nDeletes a calendar."]
pub struct MlDeleteCalendar<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteCalendarParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteCalendar<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteCalendar] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteCalendarParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteCalendar {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Delete Calendar API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Delete Calendar Event API"]
pub enum MlDeleteCalendarEventParts<'b> {
    #[doc = "CalendarId and EventId"]
    CalendarIdEventId(&'b str, &'b str),
}
impl<'b> MlDeleteCalendarEventParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Calendar Event API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteCalendarEventParts::CalendarIdEventId(ref calendar_id, ref event_id) => {
                let encoded_calendar_id: Cow<str> =
                    percent_encode(calendar_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_event_id: Cow<str> =
                    percent_encode(event_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    23usize + encoded_calendar_id.len() + encoded_event_id.len(),
                );
                p.push_str("/_ml/calendars/");
                p.push_str(encoded_calendar_id.as_ref());
                p.push_str("/events/");
                p.push_str(encoded_event_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Delete Calendar Event API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-calendar-event.html)\n\nDeletes scheduled events from a calendar."]
pub struct MlDeleteCalendarEvent<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteCalendarEventParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteCalendarEvent<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteCalendarEvent] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteCalendarEventParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteCalendarEvent {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Delete Calendar Event API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Delete Calendar Job API"]
pub enum MlDeleteCalendarJobParts<'b> {
    #[doc = "CalendarId and JobId"]
    CalendarIdJobId(&'b str, &'b str),
}
impl<'b> MlDeleteCalendarJobParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Calendar Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteCalendarJobParts::CalendarIdJobId(ref calendar_id, ref job_id) => {
                let encoded_calendar_id: Cow<str> =
                    percent_encode(calendar_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    21usize + encoded_calendar_id.len() + encoded_job_id.len(),
                );
                p.push_str("/_ml/calendars/");
                p.push_str(encoded_calendar_id.as_ref());
                p.push_str("/jobs/");
                p.push_str(encoded_job_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Delete Calendar Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-calendar-job.html)\n\nDeletes anomaly detection jobs from a calendar."]
pub struct MlDeleteCalendarJob<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteCalendarJobParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteCalendarJob<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteCalendarJob] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteCalendarJobParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteCalendarJob {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Delete Calendar Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Delete Datafeed API"]
pub enum MlDeleteDatafeedParts<'b> {
    #[doc = "DatafeedId"]
    DatafeedId(&'b str),
}
impl<'b> MlDeleteDatafeedParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Datafeed API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteDatafeedParts::DatafeedId(ref datafeed_id) => {
                let encoded_datafeed_id: Cow<str> =
                    percent_encode(datafeed_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(encoded_datafeed_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Delete Datafeed API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-datafeed.html)\n\nDeletes an existing datafeed."]
pub struct MlDeleteDatafeed<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteDatafeedParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    force: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteDatafeed<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteDatafeed] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteDatafeedParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteDatafeed {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            force: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "True if the datafeed should be forcefully deleted"]
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Delete Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                force: self.force,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Delete Expired Data API"]
pub enum MlDeleteExpiredDataParts {
    #[doc = "No parts"]
    None,
}
impl MlDeleteExpiredDataParts {
    #[doc = "Builds a relative URL path to the Ml Delete Expired Data API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteExpiredDataParts::None => "/_ml/_delete_expired_data".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Delete Expired Data API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-expired-data.html)\n\nDeletes expired and unused machine learning data."]
pub struct MlDeleteExpiredData<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlDeleteExpiredDataParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlDeleteExpiredData<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlDeleteExpiredData]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlDeleteExpiredData {
            transport,
            parts: MlDeleteExpiredDataParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlDeleteExpiredData<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlDeleteExpiredData {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Delete Expired Data API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Delete Filter API"]
pub enum MlDeleteFilterParts<'b> {
    #[doc = "FilterId"]
    FilterId(&'b str),
}
impl<'b> MlDeleteFilterParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Filter API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteFilterParts::FilterId(ref filter_id) => {
                let encoded_filter_id: Cow<str> =
                    percent_encode(filter_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_filter_id.len());
                p.push_str("/_ml/filters/");
                p.push_str(encoded_filter_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Delete Filter API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-filter.html)\n\nDeletes a filter."]
pub struct MlDeleteFilter<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteFilterParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteFilter<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteFilter] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteFilterParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteFilter {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Delete Filter API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Delete Forecast API"]
pub enum MlDeleteForecastParts<'b> {
    #[doc = "JobId"]
    JobId(&'b str),
    #[doc = "JobId and ForecastId"]
    JobIdForecastId(&'b str, &'b str),
}
impl<'b> MlDeleteForecastParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Forecast API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteForecastParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/_forecast");
                p.into()
            }
            MlDeleteForecastParts::JobIdForecastId(ref job_id, ref forecast_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_forecast_id: Cow<str> =
                    percent_encode(forecast_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    34usize + encoded_job_id.len() + encoded_forecast_id.len(),
                );
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/_forecast/");
                p.push_str(encoded_forecast_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Delete Forecast API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-forecast.html)\n\nDeletes forecasts from a machine learning job."]
pub struct MlDeleteForecast<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteForecastParts<'b>,
    allow_no_forecasts: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> MlDeleteForecast<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteForecast] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteForecastParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteForecast {
            transport,
            parts,
            headers,
            allow_no_forecasts: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Whether to ignore if `_all` matches no forecasts"]
    pub fn allow_no_forecasts(mut self, allow_no_forecasts: bool) -> Self {
        self.allow_no_forecasts = Some(allow_no_forecasts);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait until the forecast(s) are deleted. Default to 30 seconds"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Delete Forecast API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_forecasts")]
                allow_no_forecasts: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_forecasts: self.allow_no_forecasts,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Delete Job API"]
pub enum MlDeleteJobParts<'b> {
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlDeleteJobParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteJobParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Delete Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-job.html)\n\nDeletes an existing anomaly detection job."]
pub struct MlDeleteJob<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteJobParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    force: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b> MlDeleteJob<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteJob] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteJobParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteJob {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            force: None,
            human: None,
            pretty: None,
            source: None,
            wait_for_completion: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "True if the job should be forcefully deleted"]
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Should this request wait until the operation has completed before returning"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Delete Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                force: self.force,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Delete Model Snapshot API"]
pub enum MlDeleteModelSnapshotParts<'b> {
    #[doc = "JobId and SnapshotId"]
    JobIdSnapshotId(&'b str, &'b str),
}
impl<'b> MlDeleteModelSnapshotParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Model Snapshot API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteModelSnapshotParts::JobIdSnapshotId(ref job_id, ref snapshot_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_snapshot_id: Cow<str> =
                    percent_encode(snapshot_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    40usize + encoded_job_id.len() + encoded_snapshot_id.len(),
                );
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/model_snapshots/");
                p.push_str(encoded_snapshot_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Delete Model Snapshot API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-snapshot.html)\n\nDeletes an existing model snapshot."]
pub struct MlDeleteModelSnapshot<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteModelSnapshotParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteModelSnapshot<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteModelSnapshot] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteModelSnapshotParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteModelSnapshot {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Delete Model Snapshot API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Estimate Model Memory API"]
pub enum MlEstimateModelMemoryParts {
    #[doc = "No parts"]
    None,
}
impl MlEstimateModelMemoryParts {
    #[doc = "Builds a relative URL path to the Ml Estimate Model Memory API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlEstimateModelMemoryParts::None => {
                "/_ml/anomaly_detectors/_estimate_model_memory".into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Estimate Model Memory API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-apis.html)\n\nEstimates the model memory"]
pub struct MlEstimateModelMemory<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlEstimateModelMemoryParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlEstimateModelMemory<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlEstimateModelMemory]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlEstimateModelMemory {
            transport,
            parts: MlEstimateModelMemoryParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlEstimateModelMemory<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlEstimateModelMemory {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Estimate Model Memory API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Flush Job API"]
pub enum MlFlushJobParts<'b> {
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlFlushJobParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Flush Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlFlushJobParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/_flush");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Flush Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-flush-job.html)\n\nForces any buffered data to be processed by the job."]
pub struct MlFlushJob<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlFlushJobParts<'b>,
    advance_time: Option<&'b str>,
    body: Option<B>,
    calc_interim: Option<bool>,
    end: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    skip_time: Option<&'b str>,
    source: Option<&'b str>,
    start: Option<&'b str>,
}
impl<'a, 'b, B> MlFlushJob<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlFlushJob] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlFlushJobParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlFlushJob {
            transport,
            parts,
            headers,
            advance_time: None,
            body: None,
            calc_interim: None,
            end: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            skip_time: None,
            source: None,
            start: None,
        }
    }
    #[doc = "Advances time to the given value generating results and updating the model for the advanced interval"]
    pub fn advance_time(mut self, advance_time: &'b str) -> Self {
        self.advance_time = Some(advance_time);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlFlushJob<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlFlushJob {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            advance_time: self.advance_time,
            calc_interim: self.calc_interim,
            end: self.end,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            skip_time: self.skip_time,
            source: self.source,
            start: self.start,
        }
    }
    #[doc = "Calculates interim results for the most recent bucket or all buckets within the latency period"]
    pub fn calc_interim(mut self, calc_interim: bool) -> Self {
        self.calc_interim = Some(calc_interim);
        self
    }
    #[doc = "When used in conjunction with calc_interim, specifies the range of buckets on which to calculate interim results"]
    pub fn end(mut self, end: &'b str) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Skips time to the given value without generating results or updating the model for the skipped interval"]
    pub fn skip_time(mut self, skip_time: &'b str) -> Self {
        self.skip_time = Some(skip_time);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "When used in conjunction with calc_interim, specifies the range of buckets on which to calculate interim results"]
    pub fn start(mut self, start: &'b str) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Flush Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "advance_time")]
                advance_time: Option<&'b str>,
                #[serde(rename = "calc_interim")]
                calc_interim: Option<bool>,
                #[serde(rename = "end")]
                end: Option<&'b str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "skip_time")]
                skip_time: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "start")]
                start: Option<&'b str>,
            }
            let query_params = QueryParams {
                advance_time: self.advance_time,
                calc_interim: self.calc_interim,
                end: self.end,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                skip_time: self.skip_time,
                source: self.source,
                start: self.start,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Forecast API"]
pub enum MlForecastParts<'b> {
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlForecastParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Forecast API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlForecastParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/_forecast");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Forecast API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-forecast.html)\n\nPredicts the future behavior of a time series by using its historical behavior."]
pub struct MlForecast<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlForecastParts<'b>,
    body: Option<B>,
    duration: Option<&'b str>,
    error_trace: Option<bool>,
    expires_in: Option<&'b str>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlForecast<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlForecast] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlForecastParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlForecast {
            transport,
            parts,
            headers,
            body: None,
            duration: None,
            error_trace: None,
            expires_in: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlForecast<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlForecast {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            duration: self.duration,
            error_trace: self.error_trace,
            expires_in: self.expires_in,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "The duration of the forecast"]
    pub fn duration(mut self, duration: &'b str) -> Self {
        self.duration = Some(duration);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "The time interval after which the forecast expires. Expired forecasts will be deleted at the first opportunity."]
    pub fn expires_in(mut self, expires_in: &'b str) -> Self {
        self.expires_in = Some(expires_in);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Forecast API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "duration")]
                duration: Option<&'b str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expires_in")]
                expires_in: Option<&'b str>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                duration: self.duration,
                error_trace: self.error_trace,
                expires_in: self.expires_in,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Get Buckets API"]
pub enum MlGetBucketsParts<'b> {
    #[doc = "JobId and Timestamp"]
    JobIdTimestamp(&'b str, &'b str),
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlGetBucketsParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Buckets API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetBucketsParts::JobIdTimestamp(ref job_id, ref timestamp) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_timestamp: Cow<str> =
                    percent_encode(timestamp.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(40usize + encoded_job_id.len() + encoded_timestamp.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/results/buckets/");
                p.push_str(encoded_timestamp.as_ref());
                p.into()
            }
            MlGetBucketsParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(39usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/results/buckets");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Buckets API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-bucket.html)\n\nRetrieves anomaly detection job results for one or more buckets."]
pub struct MlGetBuckets<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlGetBucketsParts<'b>,
    anomaly_score: Option<f64>,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<&'b str>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    expand: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    sort: Option<&'b str>,
    source: Option<&'b str>,
    start: Option<&'b str>,
}
impl<'a, 'b, B> MlGetBuckets<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlGetBuckets] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetBucketsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetBuckets {
            transport,
            parts,
            headers,
            anomaly_score: None,
            body: None,
            desc: None,
            end: None,
            error_trace: None,
            exclude_interim: None,
            expand: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            sort: None,
            source: None,
            start: None,
        }
    }
    #[doc = "Filter for the most anomalous buckets"]
    pub fn anomaly_score(mut self, anomaly_score: f64) -> Self {
        self.anomaly_score = Some(anomaly_score);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlGetBuckets<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetBuckets {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            anomaly_score: self.anomaly_score,
            desc: self.desc,
            end: self.end,
            error_trace: self.error_trace,
            exclude_interim: self.exclude_interim,
            expand: self.expand,
            filter_path: self.filter_path,
            from: self.from,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            size: self.size,
            sort: self.sort,
            source: self.source,
            start: self.start,
        }
    }
    #[doc = "Set the sort direction"]
    pub fn desc(mut self, desc: bool) -> Self {
        self.desc = Some(desc);
        self
    }
    #[doc = "End time filter for buckets"]
    pub fn end(mut self, end: &'b str) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Exclude interim results"]
    pub fn exclude_interim(mut self, exclude_interim: bool) -> Self {
        self.exclude_interim = Some(exclude_interim);
        self
    }
    #[doc = "Include anomaly records"]
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of buckets"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "specifies a max number of buckets to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "Sort buckets by a particular field"]
    pub fn sort(mut self, sort: &'b str) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Start time filter for buckets"]
    pub fn start(mut self, start: &'b str) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Buckets API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "anomaly_score")]
                anomaly_score: Option<f64>,
                #[serde(rename = "desc")]
                desc: Option<bool>,
                #[serde(rename = "end")]
                end: Option<&'b str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "exclude_interim")]
                exclude_interim: Option<bool>,
                #[serde(rename = "expand")]
                expand: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "sort")]
                sort: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "start")]
                start: Option<&'b str>,
            }
            let query_params = QueryParams {
                anomaly_score: self.anomaly_score,
                desc: self.desc,
                end: self.end,
                error_trace: self.error_trace,
                exclude_interim: self.exclude_interim,
                expand: self.expand,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                size: self.size,
                sort: self.sort,
                source: self.source,
                start: self.start,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Get Calendar Events API"]
pub enum MlGetCalendarEventsParts<'b> {
    #[doc = "CalendarId"]
    CalendarId(&'b str),
}
impl<'b> MlGetCalendarEventsParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Calendar Events API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetCalendarEventsParts::CalendarId(ref calendar_id) => {
                let encoded_calendar_id: Cow<str> =
                    percent_encode(calendar_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_calendar_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(encoded_calendar_id.as_ref());
                p.push_str("/events");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Calendar Events API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-calendar-event.html)\n\nRetrieves information about the scheduled events in calendars."]
pub struct MlGetCalendarEvents<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetCalendarEventsParts<'b>,
    end: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    job_id: Option<&'b str>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<&'b str>,
    start: Option<&'b str>,
}
impl<'a, 'b> MlGetCalendarEvents<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetCalendarEvents] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetCalendarEventsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetCalendarEvents {
            transport,
            parts,
            headers,
            end: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            job_id: None,
            pretty: None,
            size: None,
            source: None,
            start: None,
        }
    }
    #[doc = "Get events before this time"]
    pub fn end(mut self, end: &'b str) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Skips a number of events"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Get events for the job. When this option is used calendar_id must be '_all'"]
    pub fn job_id(mut self, job_id: &'b str) -> Self {
        self.job_id = Some(job_id);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specifies a max number of events to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Get events after this time"]
    pub fn start(mut self, start: &'b str) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Calendar Events API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "end")]
                end: Option<&'b str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "job_id")]
                job_id: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "start")]
                start: Option<&'b str>,
            }
            let query_params = QueryParams {
                end: self.end,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                job_id: self.job_id,
                pretty: self.pretty,
                size: self.size,
                source: self.source,
                start: self.start,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Get Calendars API"]
pub enum MlGetCalendarsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "CalendarId"]
    CalendarId(&'b str),
}
impl<'b> MlGetCalendarsParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Calendars API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetCalendarsParts::None => "/_ml/calendars".into(),
            MlGetCalendarsParts::CalendarId(ref calendar_id) => {
                let encoded_calendar_id: Cow<str> =
                    percent_encode(calendar_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_calendar_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(encoded_calendar_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Calendars API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-calendar.html)\n\nRetrieves configuration information for calendars."]
pub struct MlGetCalendars<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlGetCalendarsParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlGetCalendars<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlGetCalendars] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetCalendarsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetCalendars {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlGetCalendars<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetCalendars {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            from: self.from,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            size: self.size,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of calendars"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "specifies a max number of calendars to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Calendars API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                size: self.size,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Get Categories API"]
pub enum MlGetCategoriesParts<'b> {
    #[doc = "JobId and CategoryId"]
    JobIdCategoryId(&'b str, i64),
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlGetCategoriesParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Categories API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetCategoriesParts::JobIdCategoryId(ref job_id, ref category_id) => {
                let category_id_str = category_id.to_string();
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_category_id: Cow<str> =
                    percent_encode(category_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    43usize + encoded_job_id.len() + encoded_category_id.len(),
                );
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/results/categories/");
                p.push_str(encoded_category_id.as_ref());
                p.into()
            }
            MlGetCategoriesParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(43usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/results/categories/");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Categories API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-category.html)\n\nRetrieves anomaly detection job results for one or more categories."]
pub struct MlGetCategories<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlGetCategoriesParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlGetCategories<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlGetCategories] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetCategoriesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetCategories {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlGetCategories<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetCategories {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            from: self.from,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            size: self.size,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of categories"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "specifies a max number of categories to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Categories API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                size: self.size,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Get Datafeed Stats API"]
pub enum MlGetDatafeedStatsParts<'b> {
    #[doc = "DatafeedId"]
    DatafeedId(&'b str),
    #[doc = "No parts"]
    None,
}
impl<'b> MlGetDatafeedStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Datafeed Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetDatafeedStatsParts::DatafeedId(ref datafeed_id) => {
                let encoded_datafeed_id: Cow<str> =
                    percent_encode(datafeed_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(encoded_datafeed_id.as_ref());
                p.push_str("/_stats");
                p.into()
            }
            MlGetDatafeedStatsParts::None => "/_ml/datafeeds/_stats".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Datafeed Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-datafeed-stats.html)\n\nRetrieves usage information for datafeeds."]
pub struct MlGetDatafeedStats<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetDatafeedStatsParts<'b>,
    allow_no_datafeeds: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetDatafeedStats<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetDatafeedStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetDatafeedStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetDatafeedStats {
            transport,
            parts,
            headers,
            allow_no_datafeeds: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no datafeeds. (This includes `_all` string or when no datafeeds have been specified)"]
    pub fn allow_no_datafeeds(mut self, allow_no_datafeeds: bool) -> Self {
        self.allow_no_datafeeds = Some(allow_no_datafeeds);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Datafeed Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_datafeeds")]
                allow_no_datafeeds: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_datafeeds: self.allow_no_datafeeds,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Get Datafeeds API"]
pub enum MlGetDatafeedsParts<'b> {
    #[doc = "DatafeedId"]
    DatafeedId(&'b str),
    #[doc = "No parts"]
    None,
}
impl<'b> MlGetDatafeedsParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Datafeeds API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetDatafeedsParts::DatafeedId(ref datafeed_id) => {
                let encoded_datafeed_id: Cow<str> =
                    percent_encode(datafeed_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(encoded_datafeed_id.as_ref());
                p.into()
            }
            MlGetDatafeedsParts::None => "/_ml/datafeeds".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Datafeeds API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-datafeed.html)\n\nRetrieves configuration information for datafeeds."]
pub struct MlGetDatafeeds<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetDatafeedsParts<'b>,
    allow_no_datafeeds: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetDatafeeds<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetDatafeeds] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetDatafeedsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetDatafeeds {
            transport,
            parts,
            headers,
            allow_no_datafeeds: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no datafeeds. (This includes `_all` string or when no datafeeds have been specified)"]
    pub fn allow_no_datafeeds(mut self, allow_no_datafeeds: bool) -> Self {
        self.allow_no_datafeeds = Some(allow_no_datafeeds);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Datafeeds API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_datafeeds")]
                allow_no_datafeeds: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_datafeeds: self.allow_no_datafeeds,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Get Filters API"]
pub enum MlGetFiltersParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "FilterId"]
    FilterId(&'b str),
}
impl<'b> MlGetFiltersParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Filters API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetFiltersParts::None => "/_ml/filters".into(),
            MlGetFiltersParts::FilterId(ref filter_id) => {
                let encoded_filter_id: Cow<str> =
                    percent_encode(filter_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_filter_id.len());
                p.push_str("/_ml/filters/");
                p.push_str(encoded_filter_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Filters API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-filter.html)\n\nRetrieves filters."]
pub struct MlGetFilters<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetFiltersParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetFilters<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetFilters] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetFiltersParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetFilters {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of filters"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "specifies a max number of filters to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Filters API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                size: self.size,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Get Influencers API"]
pub enum MlGetInfluencersParts<'b> {
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlGetInfluencersParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Influencers API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetInfluencersParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(43usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/results/influencers");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Influencers API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-influencer.html)\n\nRetrieves anomaly detection job results for one or more influencers."]
pub struct MlGetInfluencers<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlGetInfluencersParts<'b>,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<&'b str>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    influencer_score: Option<f64>,
    pretty: Option<bool>,
    size: Option<i32>,
    sort: Option<&'b str>,
    source: Option<&'b str>,
    start: Option<&'b str>,
}
impl<'a, 'b, B> MlGetInfluencers<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlGetInfluencers] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetInfluencersParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetInfluencers {
            transport,
            parts,
            headers,
            body: None,
            desc: None,
            end: None,
            error_trace: None,
            exclude_interim: None,
            filter_path: None,
            from: None,
            human: None,
            influencer_score: None,
            pretty: None,
            size: None,
            sort: None,
            source: None,
            start: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlGetInfluencers<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetInfluencers {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            desc: self.desc,
            end: self.end,
            error_trace: self.error_trace,
            exclude_interim: self.exclude_interim,
            filter_path: self.filter_path,
            from: self.from,
            headers: self.headers,
            human: self.human,
            influencer_score: self.influencer_score,
            pretty: self.pretty,
            size: self.size,
            sort: self.sort,
            source: self.source,
            start: self.start,
        }
    }
    #[doc = "whether the results should be sorted in decending order"]
    pub fn desc(mut self, desc: bool) -> Self {
        self.desc = Some(desc);
        self
    }
    #[doc = "end timestamp for the requested influencers"]
    pub fn end(mut self, end: &'b str) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Exclude interim results"]
    pub fn exclude_interim(mut self, exclude_interim: bool) -> Self {
        self.exclude_interim = Some(exclude_interim);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of influencers"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "influencer score threshold for the requested influencers"]
    pub fn influencer_score(mut self, influencer_score: f64) -> Self {
        self.influencer_score = Some(influencer_score);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "specifies a max number of influencers to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "sort field for the requested influencers"]
    pub fn sort(mut self, sort: &'b str) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "start timestamp for the requested influencers"]
    pub fn start(mut self, start: &'b str) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Influencers API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "desc")]
                desc: Option<bool>,
                #[serde(rename = "end")]
                end: Option<&'b str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "exclude_interim")]
                exclude_interim: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "influencer_score")]
                influencer_score: Option<f64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "sort")]
                sort: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "start")]
                start: Option<&'b str>,
            }
            let query_params = QueryParams {
                desc: self.desc,
                end: self.end,
                error_trace: self.error_trace,
                exclude_interim: self.exclude_interim,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                influencer_score: self.influencer_score,
                pretty: self.pretty,
                size: self.size,
                sort: self.sort,
                source: self.source,
                start: self.start,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Get Job Stats API"]
pub enum MlGetJobStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlGetJobStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Job Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetJobStatsParts::None => "/_ml/anomaly_detectors/_stats".into(),
            MlGetJobStatsParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/_stats");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Job Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-job-stats.html)\n\nRetrieves usage information for anomaly detection jobs."]
pub struct MlGetJobStats<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetJobStatsParts<'b>,
    allow_no_jobs: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetJobStats<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetJobStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetJobStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetJobStats {
            transport,
            parts,
            headers,
            allow_no_jobs: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: bool) -> Self {
        self.allow_no_jobs = Some(allow_no_jobs);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Job Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_jobs")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_jobs: self.allow_no_jobs,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Get Jobs API"]
pub enum MlGetJobsParts<'b> {
    #[doc = "JobId"]
    JobId(&'b str),
    #[doc = "No parts"]
    None,
}
impl<'b> MlGetJobsParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Jobs API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetJobsParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.into()
            }
            MlGetJobsParts::None => "/_ml/anomaly_detectors".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Jobs API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-job.html)\n\nRetrieves configuration information for anomaly detection jobs."]
pub struct MlGetJobs<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetJobsParts<'b>,
    allow_no_jobs: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetJobs<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetJobs] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetJobsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetJobs {
            transport,
            parts,
            headers,
            allow_no_jobs: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: bool) -> Self {
        self.allow_no_jobs = Some(allow_no_jobs);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Jobs API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_jobs")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_jobs: self.allow_no_jobs,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Get Model Snapshots API"]
pub enum MlGetModelSnapshotsParts<'b> {
    #[doc = "JobId and SnapshotId"]
    JobIdSnapshotId(&'b str, &'b str),
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlGetModelSnapshotsParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Model Snapshots API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetModelSnapshotsParts::JobIdSnapshotId(ref job_id, ref snapshot_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_snapshot_id: Cow<str> =
                    percent_encode(snapshot_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    40usize + encoded_job_id.len() + encoded_snapshot_id.len(),
                );
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/model_snapshots/");
                p.push_str(encoded_snapshot_id.as_ref());
                p.into()
            }
            MlGetModelSnapshotsParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(39usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/model_snapshots");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Model Snapshots API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-snapshot.html)\n\nRetrieves information about model snapshots."]
pub struct MlGetModelSnapshots<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlGetModelSnapshotsParts<'b>,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    sort: Option<&'b str>,
    source: Option<&'b str>,
    start: Option<&'b str>,
}
impl<'a, 'b, B> MlGetModelSnapshots<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlGetModelSnapshots] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetModelSnapshotsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetModelSnapshots {
            transport,
            parts,
            headers,
            body: None,
            desc: None,
            end: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            sort: None,
            source: None,
            start: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlGetModelSnapshots<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetModelSnapshots {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            desc: self.desc,
            end: self.end,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            from: self.from,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            size: self.size,
            sort: self.sort,
            source: self.source,
            start: self.start,
        }
    }
    #[doc = "True if the results should be sorted in descending order"]
    pub fn desc(mut self, desc: bool) -> Self {
        self.desc = Some(desc);
        self
    }
    #[doc = "The filter 'end' query parameter"]
    pub fn end(mut self, end: &'b str) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Skips a number of documents"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The default number of documents returned in queries as a string."]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "Name of the field to sort on"]
    pub fn sort(mut self, sort: &'b str) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The filter 'start' query parameter"]
    pub fn start(mut self, start: &'b str) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Model Snapshots API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "desc")]
                desc: Option<bool>,
                #[serde(rename = "end")]
                end: Option<&'b str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "sort")]
                sort: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "start")]
                start: Option<&'b str>,
            }
            let query_params = QueryParams {
                desc: self.desc,
                end: self.end,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                size: self.size,
                sort: self.sort,
                source: self.source,
                start: self.start,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Get Overall Buckets API"]
pub enum MlGetOverallBucketsParts<'b> {
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlGetOverallBucketsParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Overall Buckets API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetOverallBucketsParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(47usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/results/overall_buckets");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Overall Buckets API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-overall-buckets.html)\n\nRetrieves overall bucket results that summarize the bucket results of multiple anomaly detection jobs."]
pub struct MlGetOverallBuckets<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlGetOverallBucketsParts<'b>,
    allow_no_jobs: Option<bool>,
    body: Option<B>,
    bucket_span: Option<&'b str>,
    end: Option<&'b str>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    overall_score: Option<f64>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    start: Option<&'b str>,
    top_n: Option<i32>,
}
impl<'a, 'b, B> MlGetOverallBuckets<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlGetOverallBuckets] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetOverallBucketsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetOverallBuckets {
            transport,
            parts,
            headers,
            allow_no_jobs: None,
            body: None,
            bucket_span: None,
            end: None,
            error_trace: None,
            exclude_interim: None,
            filter_path: None,
            human: None,
            overall_score: None,
            pretty: None,
            source: None,
            start: None,
            top_n: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: bool) -> Self {
        self.allow_no_jobs = Some(allow_no_jobs);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlGetOverallBuckets<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetOverallBuckets {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_jobs: self.allow_no_jobs,
            bucket_span: self.bucket_span,
            end: self.end,
            error_trace: self.error_trace,
            exclude_interim: self.exclude_interim,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            overall_score: self.overall_score,
            pretty: self.pretty,
            source: self.source,
            start: self.start,
            top_n: self.top_n,
        }
    }
    #[doc = "The span of the overall buckets. Defaults to the longest job bucket_span"]
    pub fn bucket_span(mut self, bucket_span: &'b str) -> Self {
        self.bucket_span = Some(bucket_span);
        self
    }
    #[doc = "Returns overall buckets with timestamps earlier than this time"]
    pub fn end(mut self, end: &'b str) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "If true overall buckets that include interim buckets will be excluded"]
    pub fn exclude_interim(mut self, exclude_interim: bool) -> Self {
        self.exclude_interim = Some(exclude_interim);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Returns overall buckets with overall scores higher than this value"]
    pub fn overall_score(mut self, overall_score: f64) -> Self {
        self.overall_score = Some(overall_score);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Returns overall buckets with timestamps after this time"]
    pub fn start(mut self, start: &'b str) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "The number of top job bucket scores to be used in the overall_score calculation"]
    pub fn top_n(mut self, top_n: i32) -> Self {
        self.top_n = Some(top_n);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Overall Buckets API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_jobs")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "bucket_span")]
                bucket_span: Option<&'b str>,
                #[serde(rename = "end")]
                end: Option<&'b str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "exclude_interim")]
                exclude_interim: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "overall_score")]
                overall_score: Option<f64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "start")]
                start: Option<&'b str>,
                #[serde(rename = "top_n")]
                top_n: Option<i32>,
            }
            let query_params = QueryParams {
                allow_no_jobs: self.allow_no_jobs,
                bucket_span: self.bucket_span,
                end: self.end,
                error_trace: self.error_trace,
                exclude_interim: self.exclude_interim,
                filter_path: self.filter_path,
                human: self.human,
                overall_score: self.overall_score,
                pretty: self.pretty,
                source: self.source,
                start: self.start,
                top_n: self.top_n,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Get Records API"]
pub enum MlGetRecordsParts<'b> {
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlGetRecordsParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Records API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetRecordsParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(39usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/results/records");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Records API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-record.html)\n\nRetrieves anomaly records for an anomaly detection job."]
pub struct MlGetRecords<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlGetRecordsParts<'b>,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<&'b str>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    record_score: Option<f64>,
    size: Option<i32>,
    sort: Option<&'b str>,
    source: Option<&'b str>,
    start: Option<&'b str>,
}
impl<'a, 'b, B> MlGetRecords<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlGetRecords] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetRecordsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetRecords {
            transport,
            parts,
            headers,
            body: None,
            desc: None,
            end: None,
            error_trace: None,
            exclude_interim: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            record_score: None,
            size: None,
            sort: None,
            source: None,
            start: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlGetRecords<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetRecords {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            desc: self.desc,
            end: self.end,
            error_trace: self.error_trace,
            exclude_interim: self.exclude_interim,
            filter_path: self.filter_path,
            from: self.from,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            record_score: self.record_score,
            size: self.size,
            sort: self.sort,
            source: self.source,
            start: self.start,
        }
    }
    #[doc = "Set the sort direction"]
    pub fn desc(mut self, desc: bool) -> Self {
        self.desc = Some(desc);
        self
    }
    #[doc = "End time filter for records"]
    pub fn end(mut self, end: &'b str) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Exclude interim results"]
    pub fn exclude_interim(mut self, exclude_interim: bool) -> Self {
        self.exclude_interim = Some(exclude_interim);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of records"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Returns records with anomaly scores greater or equal than this value"]
    pub fn record_score(mut self, record_score: f64) -> Self {
        self.record_score = Some(record_score);
        self
    }
    #[doc = "specifies a max number of records to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "Sort records by a particular field"]
    pub fn sort(mut self, sort: &'b str) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Start time filter for records"]
    pub fn start(mut self, start: &'b str) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Records API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "desc")]
                desc: Option<bool>,
                #[serde(rename = "end")]
                end: Option<&'b str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "exclude_interim")]
                exclude_interim: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "record_score")]
                record_score: Option<f64>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "sort")]
                sort: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "start")]
                start: Option<&'b str>,
            }
            let query_params = QueryParams {
                desc: self.desc,
                end: self.end,
                error_trace: self.error_trace,
                exclude_interim: self.exclude_interim,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                record_score: self.record_score,
                size: self.size,
                sort: self.sort,
                source: self.source,
                start: self.start,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Info API"]
pub enum MlInfoParts {
    #[doc = "No parts"]
    None,
}
impl MlInfoParts {
    #[doc = "Builds a relative URL path to the Ml Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlInfoParts::None => "/_ml/info".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Info API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/get-ml-info.html)\n\nReturns defaults and limits used by machine learning."]
pub struct MlInfo<'a, 'b> {
    transport: &'a Transport,
    parts: MlInfoParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlInfo<'a, 'b> {
    #[doc = "Creates a new instance of [MlInfo]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlInfo {
            transport,
            parts: MlInfoParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Info API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Open Job API"]
pub enum MlOpenJobParts<'b> {
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlOpenJobParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Open Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlOpenJobParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/_open");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Open Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-open-job.html)\n\nOpens one or more anomaly detection jobs."]
pub struct MlOpenJob<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlOpenJobParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlOpenJob<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlOpenJob] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlOpenJobParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlOpenJob {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlOpenJob<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlOpenJob {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Open Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Post Calendar Events API"]
pub enum MlPostCalendarEventsParts<'b> {
    #[doc = "CalendarId"]
    CalendarId(&'b str),
}
impl<'b> MlPostCalendarEventsParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Post Calendar Events API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPostCalendarEventsParts::CalendarId(ref calendar_id) => {
                let encoded_calendar_id: Cow<str> =
                    percent_encode(calendar_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_calendar_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(encoded_calendar_id.as_ref());
                p.push_str("/events");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Post Calendar Events API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-post-calendar-event.html)\n\nPosts scheduled events in a calendar."]
pub struct MlPostCalendarEvents<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlPostCalendarEventsParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlPostCalendarEvents<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlPostCalendarEvents] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlPostCalendarEventsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlPostCalendarEvents {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlPostCalendarEvents<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPostCalendarEvents {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Post Calendar Events API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Post Data API"]
pub enum MlPostDataParts<'b> {
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlPostDataParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Post Data API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPostDataParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/_data");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Post Data API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-post-data.html)\n\nSends data to an anomaly detection job for analysis."]
pub struct MlPostData<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlPostDataParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    reset_end: Option<&'b str>,
    reset_start: Option<&'b str>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlPostData<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlPostData] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlPostDataParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlPostData {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            reset_end: None,
            reset_start: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: Vec<T>) -> MlPostData<'a, 'b, NdBody<T>>
    where
        T: Body,
    {
        MlPostData {
            transport: self.transport,
            parts: self.parts,
            body: Some(NdBody(body)),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            reset_end: self.reset_end,
            reset_start: self.reset_start,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Optional parameter to specify the end of the bucket resetting range"]
    pub fn reset_end(mut self, reset_end: &'b str) -> Self {
        self.reset_end = Some(reset_end);
        self
    }
    #[doc = "Optional parameter to specify the start of the bucket resetting range"]
    pub fn reset_start(mut self, reset_start: &'b str) -> Self {
        self.reset_start = Some(reset_start);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Post Data API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "reset_end")]
                reset_end: Option<&'b str>,
                #[serde(rename = "reset_start")]
                reset_start: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                reset_end: self.reset_end,
                reset_start: self.reset_start,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Preview Datafeed API"]
pub enum MlPreviewDatafeedParts<'b> {
    #[doc = "DatafeedId"]
    DatafeedId(&'b str),
}
impl<'b> MlPreviewDatafeedParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Preview Datafeed API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPreviewDatafeedParts::DatafeedId(ref datafeed_id) => {
                let encoded_datafeed_id: Cow<str> =
                    percent_encode(datafeed_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(encoded_datafeed_id.as_ref());
                p.push_str("/_preview");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Preview Datafeed API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-preview-datafeed.html)\n\nPreviews a datafeed."]
pub struct MlPreviewDatafeed<'a, 'b> {
    transport: &'a Transport,
    parts: MlPreviewDatafeedParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlPreviewDatafeed<'a, 'b> {
    #[doc = "Creates a new instance of [MlPreviewDatafeed] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlPreviewDatafeedParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlPreviewDatafeed {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Preview Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Put Calendar API"]
pub enum MlPutCalendarParts<'b> {
    #[doc = "CalendarId"]
    CalendarId(&'b str),
}
impl<'b> MlPutCalendarParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Put Calendar API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPutCalendarParts::CalendarId(ref calendar_id) => {
                let encoded_calendar_id: Cow<str> =
                    percent_encode(calendar_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_calendar_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(encoded_calendar_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Put Calendar API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-put-calendar.html)\n\nInstantiates a calendar."]
pub struct MlPutCalendar<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlPutCalendarParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlPutCalendar<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlPutCalendar] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlPutCalendarParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlPutCalendar {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlPutCalendar<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPutCalendar {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Put Calendar API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Put Calendar Job API"]
pub enum MlPutCalendarJobParts<'b> {
    #[doc = "CalendarId and JobId"]
    CalendarIdJobId(&'b str, &'b str),
}
impl<'b> MlPutCalendarJobParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Put Calendar Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPutCalendarJobParts::CalendarIdJobId(ref calendar_id, ref job_id) => {
                let encoded_calendar_id: Cow<str> =
                    percent_encode(calendar_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    21usize + encoded_calendar_id.len() + encoded_job_id.len(),
                );
                p.push_str("/_ml/calendars/");
                p.push_str(encoded_calendar_id.as_ref());
                p.push_str("/jobs/");
                p.push_str(encoded_job_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Put Calendar Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-put-calendar-job.html)\n\nAdds an anomaly detection job to a calendar."]
pub struct MlPutCalendarJob<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlPutCalendarJobParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlPutCalendarJob<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlPutCalendarJob] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlPutCalendarJobParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlPutCalendarJob {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlPutCalendarJob<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPutCalendarJob {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Put Calendar Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Put Datafeed API"]
pub enum MlPutDatafeedParts<'b> {
    #[doc = "DatafeedId"]
    DatafeedId(&'b str),
}
impl<'b> MlPutDatafeedParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Put Datafeed API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPutDatafeedParts::DatafeedId(ref datafeed_id) => {
                let encoded_datafeed_id: Cow<str> =
                    percent_encode(datafeed_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(encoded_datafeed_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Put Datafeed API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-put-datafeed.html)\n\nInstantiates a datafeed."]
pub struct MlPutDatafeed<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlPutDatafeedParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlPutDatafeed<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlPutDatafeed] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlPutDatafeedParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlPutDatafeed {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Ignore if the source indices expressions resolves to no concrete indices (default: true)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlPutDatafeed<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPutDatafeed {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_throttled: self.ignore_throttled,
            ignore_unavailable: self.ignore_unavailable,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether source index expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Ignore indices that are marked as throttled (default: true)"]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "Ignore unavailable indexes (default: false)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Put Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "expand_wildcards",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_throttled")]
                ignore_throttled: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Put Filter API"]
pub enum MlPutFilterParts<'b> {
    #[doc = "FilterId"]
    FilterId(&'b str),
}
impl<'b> MlPutFilterParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Put Filter API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPutFilterParts::FilterId(ref filter_id) => {
                let encoded_filter_id: Cow<str> =
                    percent_encode(filter_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_filter_id.len());
                p.push_str("/_ml/filters/");
                p.push_str(encoded_filter_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Put Filter API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-put-filter.html)\n\nInstantiates a filter."]
pub struct MlPutFilter<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlPutFilterParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlPutFilter<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlPutFilter] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlPutFilterParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlPutFilter {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlPutFilter<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPutFilter {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Put Filter API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Put Job API"]
pub enum MlPutJobParts<'b> {
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlPutJobParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Put Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPutJobParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Put Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-put-job.html)\n\nInstantiates an anomaly detection job."]
pub struct MlPutJob<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlPutJobParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlPutJob<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlPutJob] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlPutJobParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlPutJob {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlPutJob<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPutJob {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Put Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Revert Model Snapshot API"]
pub enum MlRevertModelSnapshotParts<'b> {
    #[doc = "JobId and SnapshotId"]
    JobIdSnapshotId(&'b str, &'b str),
}
impl<'b> MlRevertModelSnapshotParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Revert Model Snapshot API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlRevertModelSnapshotParts::JobIdSnapshotId(ref job_id, ref snapshot_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_snapshot_id: Cow<str> =
                    percent_encode(snapshot_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    48usize + encoded_job_id.len() + encoded_snapshot_id.len(),
                );
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/model_snapshots/");
                p.push_str(encoded_snapshot_id.as_ref());
                p.push_str("/_revert");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Revert Model Snapshot API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-revert-snapshot.html)\n\nReverts to a specific snapshot."]
pub struct MlRevertModelSnapshot<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlRevertModelSnapshotParts<'b>,
    body: Option<B>,
    delete_intervening_results: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlRevertModelSnapshot<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlRevertModelSnapshot] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlRevertModelSnapshotParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlRevertModelSnapshot {
            transport,
            parts,
            headers,
            body: None,
            delete_intervening_results: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlRevertModelSnapshot<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlRevertModelSnapshot {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            delete_intervening_results: self.delete_intervening_results,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Should we reset the results back to the time of the snapshot?"]
    pub fn delete_intervening_results(mut self, delete_intervening_results: bool) -> Self {
        self.delete_intervening_results = Some(delete_intervening_results);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Revert Model Snapshot API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "delete_intervening_results")]
                delete_intervening_results: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                delete_intervening_results: self.delete_intervening_results,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Set Upgrade Mode API"]
pub enum MlSetUpgradeModeParts {
    #[doc = "No parts"]
    None,
}
impl MlSetUpgradeModeParts {
    #[doc = "Builds a relative URL path to the Ml Set Upgrade Mode API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlSetUpgradeModeParts::None => "/_ml/set_upgrade_mode".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Set Upgrade Mode API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-set-upgrade-mode.html)\n\nSets a cluster wide upgrade_mode setting that prepares machine learning indices for an upgrade."]
pub struct MlSetUpgradeMode<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlSetUpgradeModeParts,
    body: Option<B>,
    enabled: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> MlSetUpgradeMode<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlSetUpgradeMode]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlSetUpgradeMode {
            transport,
            parts: MlSetUpgradeModeParts::None,
            headers,
            body: None,
            enabled: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlSetUpgradeMode<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlSetUpgradeMode {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            enabled: self.enabled,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Whether to enable upgrade_mode ML setting or not. Defaults to false."]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait before action times out. Defaults to 30 seconds"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Set Upgrade Mode API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "enabled")]
                enabled: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                enabled: self.enabled,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Start Datafeed API"]
pub enum MlStartDatafeedParts<'b> {
    #[doc = "DatafeedId"]
    DatafeedId(&'b str),
}
impl<'b> MlStartDatafeedParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Start Datafeed API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlStartDatafeedParts::DatafeedId(ref datafeed_id) => {
                let encoded_datafeed_id: Cow<str> =
                    percent_encode(datafeed_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(encoded_datafeed_id.as_ref());
                p.push_str("/_start");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Start Datafeed API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-start-datafeed.html)\n\nStarts one or more datafeeds."]
pub struct MlStartDatafeed<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlStartDatafeedParts<'b>,
    body: Option<B>,
    end: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    start: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> MlStartDatafeed<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlStartDatafeed] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlStartDatafeedParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlStartDatafeed {
            transport,
            parts,
            headers,
            body: None,
            end: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            start: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlStartDatafeed<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlStartDatafeed {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            end: self.end,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            start: self.start,
            timeout: self.timeout,
        }
    }
    #[doc = "The end time when the datafeed should stop. When not set, the datafeed continues in real time"]
    pub fn end(mut self, end: &'b str) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The start time from where the datafeed should begin"]
    pub fn start(mut self, start: &'b str) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Controls the time to wait until a datafeed has started. Default to 20 seconds"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Start Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "end")]
                end: Option<&'b str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "start")]
                start: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                end: self.end,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                start: self.start,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Stop Datafeed API"]
pub enum MlStopDatafeedParts<'b> {
    #[doc = "DatafeedId"]
    DatafeedId(&'b str),
}
impl<'b> MlStopDatafeedParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Stop Datafeed API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlStopDatafeedParts::DatafeedId(ref datafeed_id) => {
                let encoded_datafeed_id: Cow<str> =
                    percent_encode(datafeed_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(encoded_datafeed_id.as_ref());
                p.push_str("/_stop");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Stop Datafeed API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-stop-datafeed.html)\n\nStops one or more datafeeds."]
pub struct MlStopDatafeed<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlStopDatafeedParts<'b>,
    allow_no_datafeeds: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    force: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> MlStopDatafeed<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlStopDatafeed] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlStopDatafeedParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlStopDatafeed {
            transport,
            parts,
            headers,
            allow_no_datafeeds: None,
            body: None,
            error_trace: None,
            filter_path: None,
            force: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no datafeeds. (This includes `_all` string or when no datafeeds have been specified)"]
    pub fn allow_no_datafeeds(mut self, allow_no_datafeeds: bool) -> Self {
        self.allow_no_datafeeds = Some(allow_no_datafeeds);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlStopDatafeed<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlStopDatafeed {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_datafeeds: self.allow_no_datafeeds,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            force: self.force,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "True if the datafeed should be forcefully stopped."]
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait until a datafeed has stopped. Default to 20 seconds"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Stop Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_datafeeds")]
                allow_no_datafeeds: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_datafeeds: self.allow_no_datafeeds,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                force: self.force,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Update Datafeed API"]
pub enum MlUpdateDatafeedParts<'b> {
    #[doc = "DatafeedId"]
    DatafeedId(&'b str),
}
impl<'b> MlUpdateDatafeedParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Update Datafeed API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateDatafeedParts::DatafeedId(ref datafeed_id) => {
                let encoded_datafeed_id: Cow<str> =
                    percent_encode(datafeed_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(encoded_datafeed_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Update Datafeed API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-update-datafeed.html)\n\nUpdates certain properties of a datafeed."]
pub struct MlUpdateDatafeed<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUpdateDatafeedParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUpdateDatafeed<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUpdateDatafeed] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUpdateDatafeedParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUpdateDatafeed {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Ignore if the source indices expressions resolves to no concrete indices (default: true)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlUpdateDatafeed<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateDatafeed {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_throttled: self.ignore_throttled,
            ignore_unavailable: self.ignore_unavailable,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether source index expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Ignore indices that are marked as throttled (default: true)"]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "Ignore unavailable indexes (default: false)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Update Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "expand_wildcards",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_throttled")]
                ignore_throttled: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Update Filter API"]
pub enum MlUpdateFilterParts<'b> {
    #[doc = "FilterId"]
    FilterId(&'b str),
}
impl<'b> MlUpdateFilterParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Update Filter API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateFilterParts::FilterId(ref filter_id) => {
                let encoded_filter_id: Cow<str> =
                    percent_encode(filter_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_filter_id.len());
                p.push_str("/_ml/filters/");
                p.push_str(encoded_filter_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Update Filter API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-update-filter.html)\n\nUpdates the description of a filter, adds items, or removes items."]
pub struct MlUpdateFilter<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUpdateFilterParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUpdateFilter<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUpdateFilter] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUpdateFilterParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUpdateFilter {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlUpdateFilter<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateFilter {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Update Filter API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Update Job API"]
pub enum MlUpdateJobParts<'b> {
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> MlUpdateJobParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Update Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateJobParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(31usize + encoded_job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Update Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-update-job.html)\n\nUpdates certain properties of an anomaly detection job."]
pub struct MlUpdateJob<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUpdateJobParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUpdateJob<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUpdateJob] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUpdateJobParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUpdateJob {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlUpdateJob<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateJob {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Update Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Update Model Snapshot API"]
pub enum MlUpdateModelSnapshotParts<'b> {
    #[doc = "JobId and SnapshotId"]
    JobIdSnapshotId(&'b str, &'b str),
}
impl<'b> MlUpdateModelSnapshotParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Update Model Snapshot API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateModelSnapshotParts::JobIdSnapshotId(ref job_id, ref snapshot_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_snapshot_id: Cow<str> =
                    percent_encode(snapshot_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    48usize + encoded_job_id.len() + encoded_snapshot_id.len(),
                );
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.push_str("/model_snapshots/");
                p.push_str(encoded_snapshot_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Update Model Snapshot API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-update-snapshot.html)\n\nUpdates certain properties of a snapshot."]
pub struct MlUpdateModelSnapshot<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUpdateModelSnapshotParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUpdateModelSnapshot<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUpdateModelSnapshot] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUpdateModelSnapshotParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUpdateModelSnapshot {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlUpdateModelSnapshot<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateModelSnapshot {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Update Model Snapshot API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Validate API"]
pub enum MlValidateParts {
    #[doc = "No parts"]
    None,
}
impl MlValidateParts {
    #[doc = "Builds a relative URL path to the Ml Validate API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlValidateParts::None => "/_ml/anomaly_detectors/_validate".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Validate API](https://www.elastic.co/guide/en/machine-learning/7.8/ml-jobs.html)\n\nValidates an anomaly detection job."]
pub struct MlValidate<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlValidateParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlValidate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlValidate]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlValidate {
            transport,
            parts: MlValidateParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlValidate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlValidate {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Validate API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Validate Detector API"]
pub enum MlValidateDetectorParts {
    #[doc = "No parts"]
    None,
}
impl MlValidateDetectorParts {
    #[doc = "Builds a relative URL path to the Ml Validate Detector API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlValidateDetectorParts::None => "/_ml/anomaly_detectors/_validate/detector".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Validate Detector API](https://www.elastic.co/guide/en/machine-learning/7.8/ml-jobs.html)\n\nValidates an anomaly detection detector."]
pub struct MlValidateDetector<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlValidateDetectorParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlValidateDetector<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlValidateDetector]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlValidateDetector {
            transport,
            parts: MlValidateDetectorParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlValidateDetector<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlValidateDetector {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Validate Detector API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Machine Learning APIs"]
pub struct Ml<'a> {
    transport: &'a Transport,
}
impl<'a> Ml<'a> {
    #[doc = "Creates a new instance of [Ml]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Ml Close Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-close-job.html)\n\nCloses one or more anomaly detection jobs. A job can be opened and closed multiple times throughout its lifecycle."]
    pub fn close_job<'b>(&'a self, parts: MlCloseJobParts<'b>) -> MlCloseJob<'a, 'b, ()> {
        MlCloseJob::new(self.transport(), parts)
    }
    #[doc = "[Ml Delete Calendar API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-calendar.html)\n\nDeletes a calendar."]
    pub fn delete_calendar<'b>(
        &'a self,
        parts: MlDeleteCalendarParts<'b>,
    ) -> MlDeleteCalendar<'a, 'b> {
        MlDeleteCalendar::new(self.transport(), parts)
    }
    #[doc = "[Ml Delete Calendar Event API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-calendar-event.html)\n\nDeletes scheduled events from a calendar."]
    pub fn delete_calendar_event<'b>(
        &'a self,
        parts: MlDeleteCalendarEventParts<'b>,
    ) -> MlDeleteCalendarEvent<'a, 'b> {
        MlDeleteCalendarEvent::new(self.transport(), parts)
    }
    #[doc = "[Ml Delete Calendar Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-calendar-job.html)\n\nDeletes anomaly detection jobs from a calendar."]
    pub fn delete_calendar_job<'b>(
        &'a self,
        parts: MlDeleteCalendarJobParts<'b>,
    ) -> MlDeleteCalendarJob<'a, 'b> {
        MlDeleteCalendarJob::new(self.transport(), parts)
    }
    #[doc = "[Ml Delete Datafeed API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-datafeed.html)\n\nDeletes an existing datafeed."]
    pub fn delete_datafeed<'b>(
        &'a self,
        parts: MlDeleteDatafeedParts<'b>,
    ) -> MlDeleteDatafeed<'a, 'b> {
        MlDeleteDatafeed::new(self.transport(), parts)
    }
    #[doc = "[Ml Delete Expired Data API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-expired-data.html)\n\nDeletes expired and unused machine learning data."]
    pub fn delete_expired_data<'b>(&'a self) -> MlDeleteExpiredData<'a, 'b, ()> {
        MlDeleteExpiredData::new(self.transport())
    }
    #[doc = "[Ml Delete Filter API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-filter.html)\n\nDeletes a filter."]
    pub fn delete_filter<'b>(&'a self, parts: MlDeleteFilterParts<'b>) -> MlDeleteFilter<'a, 'b> {
        MlDeleteFilter::new(self.transport(), parts)
    }
    #[doc = "[Ml Delete Forecast API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-forecast.html)\n\nDeletes forecasts from a machine learning job."]
    pub fn delete_forecast<'b>(
        &'a self,
        parts: MlDeleteForecastParts<'b>,
    ) -> MlDeleteForecast<'a, 'b> {
        MlDeleteForecast::new(self.transport(), parts)
    }
    #[doc = "[Ml Delete Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-job.html)\n\nDeletes an existing anomaly detection job."]
    pub fn delete_job<'b>(&'a self, parts: MlDeleteJobParts<'b>) -> MlDeleteJob<'a, 'b> {
        MlDeleteJob::new(self.transport(), parts)
    }
    #[doc = "[Ml Delete Model Snapshot API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-delete-snapshot.html)\n\nDeletes an existing model snapshot."]
    pub fn delete_model_snapshot<'b>(
        &'a self,
        parts: MlDeleteModelSnapshotParts<'b>,
    ) -> MlDeleteModelSnapshot<'a, 'b> {
        MlDeleteModelSnapshot::new(self.transport(), parts)
    }
    #[doc = "[Ml Estimate Model Memory API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-apis.html)\n\nEstimates the model memory"]
    pub fn estimate_model_memory<'b>(&'a self) -> MlEstimateModelMemory<'a, 'b, ()> {
        MlEstimateModelMemory::new(self.transport())
    }
    #[doc = "[Ml Flush Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-flush-job.html)\n\nForces any buffered data to be processed by the job."]
    pub fn flush_job<'b>(&'a self, parts: MlFlushJobParts<'b>) -> MlFlushJob<'a, 'b, ()> {
        MlFlushJob::new(self.transport(), parts)
    }
    #[doc = "[Ml Forecast API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-forecast.html)\n\nPredicts the future behavior of a time series by using its historical behavior."]
    pub fn forecast<'b>(&'a self, parts: MlForecastParts<'b>) -> MlForecast<'a, 'b, ()> {
        MlForecast::new(self.transport(), parts)
    }
    #[doc = "[Ml Get Buckets API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-bucket.html)\n\nRetrieves anomaly detection job results for one or more buckets."]
    pub fn get_buckets<'b>(&'a self, parts: MlGetBucketsParts<'b>) -> MlGetBuckets<'a, 'b, ()> {
        MlGetBuckets::new(self.transport(), parts)
    }
    #[doc = "[Ml Get Calendar Events API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-calendar-event.html)\n\nRetrieves information about the scheduled events in calendars."]
    pub fn get_calendar_events<'b>(
        &'a self,
        parts: MlGetCalendarEventsParts<'b>,
    ) -> MlGetCalendarEvents<'a, 'b> {
        MlGetCalendarEvents::new(self.transport(), parts)
    }
    #[doc = "[Ml Get Calendars API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-calendar.html)\n\nRetrieves configuration information for calendars."]
    pub fn get_calendars<'b>(
        &'a self,
        parts: MlGetCalendarsParts<'b>,
    ) -> MlGetCalendars<'a, 'b, ()> {
        MlGetCalendars::new(self.transport(), parts)
    }
    #[doc = "[Ml Get Categories API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-category.html)\n\nRetrieves anomaly detection job results for one or more categories."]
    pub fn get_categories<'b>(
        &'a self,
        parts: MlGetCategoriesParts<'b>,
    ) -> MlGetCategories<'a, 'b, ()> {
        MlGetCategories::new(self.transport(), parts)
    }
    #[doc = "[Ml Get Datafeed Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-datafeed-stats.html)\n\nRetrieves usage information for datafeeds."]
    pub fn get_datafeed_stats<'b>(
        &'a self,
        parts: MlGetDatafeedStatsParts<'b>,
    ) -> MlGetDatafeedStats<'a, 'b> {
        MlGetDatafeedStats::new(self.transport(), parts)
    }
    #[doc = "[Ml Get Datafeeds API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-datafeed.html)\n\nRetrieves configuration information for datafeeds."]
    pub fn get_datafeeds<'b>(&'a self, parts: MlGetDatafeedsParts<'b>) -> MlGetDatafeeds<'a, 'b> {
        MlGetDatafeeds::new(self.transport(), parts)
    }
    #[doc = "[Ml Get Filters API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-filter.html)\n\nRetrieves filters."]
    pub fn get_filters<'b>(&'a self, parts: MlGetFiltersParts<'b>) -> MlGetFilters<'a, 'b> {
        MlGetFilters::new(self.transport(), parts)
    }
    #[doc = "[Ml Get Influencers API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-influencer.html)\n\nRetrieves anomaly detection job results for one or more influencers."]
    pub fn get_influencers<'b>(
        &'a self,
        parts: MlGetInfluencersParts<'b>,
    ) -> MlGetInfluencers<'a, 'b, ()> {
        MlGetInfluencers::new(self.transport(), parts)
    }
    #[doc = "[Ml Get Job Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-job-stats.html)\n\nRetrieves usage information for anomaly detection jobs."]
    pub fn get_job_stats<'b>(&'a self, parts: MlGetJobStatsParts<'b>) -> MlGetJobStats<'a, 'b> {
        MlGetJobStats::new(self.transport(), parts)
    }
    #[doc = "[Ml Get Jobs API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-job.html)\n\nRetrieves configuration information for anomaly detection jobs."]
    pub fn get_jobs<'b>(&'a self, parts: MlGetJobsParts<'b>) -> MlGetJobs<'a, 'b> {
        MlGetJobs::new(self.transport(), parts)
    }
    #[doc = "[Ml Get Model Snapshots API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-snapshot.html)\n\nRetrieves information about model snapshots."]
    pub fn get_model_snapshots<'b>(
        &'a self,
        parts: MlGetModelSnapshotsParts<'b>,
    ) -> MlGetModelSnapshots<'a, 'b, ()> {
        MlGetModelSnapshots::new(self.transport(), parts)
    }
    #[doc = "[Ml Get Overall Buckets API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-overall-buckets.html)\n\nRetrieves overall bucket results that summarize the bucket results of multiple anomaly detection jobs."]
    pub fn get_overall_buckets<'b>(
        &'a self,
        parts: MlGetOverallBucketsParts<'b>,
    ) -> MlGetOverallBuckets<'a, 'b, ()> {
        MlGetOverallBuckets::new(self.transport(), parts)
    }
    #[doc = "[Ml Get Records API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-get-record.html)\n\nRetrieves anomaly records for an anomaly detection job."]
    pub fn get_records<'b>(&'a self, parts: MlGetRecordsParts<'b>) -> MlGetRecords<'a, 'b, ()> {
        MlGetRecords::new(self.transport(), parts)
    }
    #[doc = "[Ml Info API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/get-ml-info.html)\n\nReturns defaults and limits used by machine learning."]
    pub fn info<'b>(&'a self) -> MlInfo<'a, 'b> {
        MlInfo::new(self.transport())
    }
    #[doc = "[Ml Open Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-open-job.html)\n\nOpens one or more anomaly detection jobs."]
    pub fn open_job<'b>(&'a self, parts: MlOpenJobParts<'b>) -> MlOpenJob<'a, 'b, ()> {
        MlOpenJob::new(self.transport(), parts)
    }
    #[doc = "[Ml Post Calendar Events API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-post-calendar-event.html)\n\nPosts scheduled events in a calendar."]
    pub fn post_calendar_events<'b>(
        &'a self,
        parts: MlPostCalendarEventsParts<'b>,
    ) -> MlPostCalendarEvents<'a, 'b, ()> {
        MlPostCalendarEvents::new(self.transport(), parts)
    }
    #[doc = "[Ml Post Data API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-post-data.html)\n\nSends data to an anomaly detection job for analysis."]
    pub fn post_data<'b>(&'a self, parts: MlPostDataParts<'b>) -> MlPostData<'a, 'b, ()> {
        MlPostData::new(self.transport(), parts)
    }
    #[doc = "[Ml Preview Datafeed API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-preview-datafeed.html)\n\nPreviews a datafeed."]
    pub fn preview_datafeed<'b>(
        &'a self,
        parts: MlPreviewDatafeedParts<'b>,
    ) -> MlPreviewDatafeed<'a, 'b> {
        MlPreviewDatafeed::new(self.transport(), parts)
    }
    #[doc = "[Ml Put Calendar API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-put-calendar.html)\n\nInstantiates a calendar."]
    pub fn put_calendar<'b>(&'a self, parts: MlPutCalendarParts<'b>) -> MlPutCalendar<'a, 'b, ()> {
        MlPutCalendar::new(self.transport(), parts)
    }
    #[doc = "[Ml Put Calendar Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-put-calendar-job.html)\n\nAdds an anomaly detection job to a calendar."]
    pub fn put_calendar_job<'b>(
        &'a self,
        parts: MlPutCalendarJobParts<'b>,
    ) -> MlPutCalendarJob<'a, 'b, ()> {
        MlPutCalendarJob::new(self.transport(), parts)
    }
    #[doc = "[Ml Put Datafeed API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-put-datafeed.html)\n\nInstantiates a datafeed."]
    pub fn put_datafeed<'b>(&'a self, parts: MlPutDatafeedParts<'b>) -> MlPutDatafeed<'a, 'b, ()> {
        MlPutDatafeed::new(self.transport(), parts)
    }
    #[doc = "[Ml Put Filter API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-put-filter.html)\n\nInstantiates a filter."]
    pub fn put_filter<'b>(&'a self, parts: MlPutFilterParts<'b>) -> MlPutFilter<'a, 'b, ()> {
        MlPutFilter::new(self.transport(), parts)
    }
    #[doc = "[Ml Put Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-put-job.html)\n\nInstantiates an anomaly detection job."]
    pub fn put_job<'b>(&'a self, parts: MlPutJobParts<'b>) -> MlPutJob<'a, 'b, ()> {
        MlPutJob::new(self.transport(), parts)
    }
    #[doc = "[Ml Revert Model Snapshot API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-revert-snapshot.html)\n\nReverts to a specific snapshot."]
    pub fn revert_model_snapshot<'b>(
        &'a self,
        parts: MlRevertModelSnapshotParts<'b>,
    ) -> MlRevertModelSnapshot<'a, 'b, ()> {
        MlRevertModelSnapshot::new(self.transport(), parts)
    }
    #[doc = "[Ml Set Upgrade Mode API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-set-upgrade-mode.html)\n\nSets a cluster wide upgrade_mode setting that prepares machine learning indices for an upgrade."]
    pub fn set_upgrade_mode<'b>(&'a self) -> MlSetUpgradeMode<'a, 'b, ()> {
        MlSetUpgradeMode::new(self.transport())
    }
    #[doc = "[Ml Start Datafeed API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-start-datafeed.html)\n\nStarts one or more datafeeds."]
    pub fn start_datafeed<'b>(
        &'a self,
        parts: MlStartDatafeedParts<'b>,
    ) -> MlStartDatafeed<'a, 'b, ()> {
        MlStartDatafeed::new(self.transport(), parts)
    }
    #[doc = "[Ml Stop Datafeed API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-stop-datafeed.html)\n\nStops one or more datafeeds."]
    pub fn stop_datafeed<'b>(
        &'a self,
        parts: MlStopDatafeedParts<'b>,
    ) -> MlStopDatafeed<'a, 'b, ()> {
        MlStopDatafeed::new(self.transport(), parts)
    }
    #[doc = "[Ml Update Datafeed API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-update-datafeed.html)\n\nUpdates certain properties of a datafeed."]
    pub fn update_datafeed<'b>(
        &'a self,
        parts: MlUpdateDatafeedParts<'b>,
    ) -> MlUpdateDatafeed<'a, 'b, ()> {
        MlUpdateDatafeed::new(self.transport(), parts)
    }
    #[doc = "[Ml Update Filter API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-update-filter.html)\n\nUpdates the description of a filter, adds items, or removes items."]
    pub fn update_filter<'b>(
        &'a self,
        parts: MlUpdateFilterParts<'b>,
    ) -> MlUpdateFilter<'a, 'b, ()> {
        MlUpdateFilter::new(self.transport(), parts)
    }
    #[doc = "[Ml Update Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-update-job.html)\n\nUpdates certain properties of an anomaly detection job."]
    pub fn update_job<'b>(&'a self, parts: MlUpdateJobParts<'b>) -> MlUpdateJob<'a, 'b, ()> {
        MlUpdateJob::new(self.transport(), parts)
    }
    #[doc = "[Ml Update Model Snapshot API](https://www.elastic.co/guide/en/elasticsearch/reference/7.8/ml-update-snapshot.html)\n\nUpdates certain properties of a snapshot."]
    pub fn update_model_snapshot<'b>(
        &'a self,
        parts: MlUpdateModelSnapshotParts<'b>,
    ) -> MlUpdateModelSnapshot<'a, 'b, ()> {
        MlUpdateModelSnapshot::new(self.transport(), parts)
    }
    #[doc = "[Ml Validate API](https://www.elastic.co/guide/en/machine-learning/7.8/ml-jobs.html)\n\nValidates an anomaly detection job."]
    pub fn validate<'b>(&'a self) -> MlValidate<'a, 'b, ()> {
        MlValidate::new(self.transport())
    }
    #[doc = "[Ml Validate Detector API](https://www.elastic.co/guide/en/machine-learning/7.8/ml-jobs.html)\n\nValidates an anomaly detection detector."]
    pub fn validate_detector<'b>(&'a self) -> MlValidateDetector<'a, 'b, ()> {
        MlValidateDetector::new(self.transport())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Machine Learning APIs"]
    pub fn ml(&self) -> Ml {
        Ml::new(self.transport())
    }
}
