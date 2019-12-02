// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
use crate::{
    client::Elasticsearch, enums::*, error::ElasticsearchError, http_method::HttpMethod,
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Bulk API"]
pub enum BulkUrlParts<'a> {
    None,
    Index(&'a str),
    IndexType(&'a str, &'a str),
}
impl<'a> BulkUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            BulkUrlParts::None => "/_bulk".into(),
            BulkUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(7usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_bulk");
                p.into()
            }
            BulkUrlParts::IndexType(ref index, ref ty) => {
                let mut p = String::with_capacity(8usize + index.len() + ty.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/_bulk");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Bulk API"]
pub struct Bulk<'a, B> {
    client: Elasticsearch,
    parts: BulkUrlParts<'a>,
    _source: Option<&'a [&'a str]>,
    _source_excludes: Option<&'a [&'a str]>,
    _source_includes: Option<&'a [&'a str]>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pipeline: Option<&'a str>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    ty: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> Bulk<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: BulkUrlParts<'a>) -> Self {
        Bulk {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pipeline: None,
            pretty: None,
            refresh: None,
            routing: None,
            source: None,
            timeout: None,
            ty: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or default list of fields to return, can be overridden on each sub-request"]
    pub fn _source(mut self, _source: &'a [&'a str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "Default list of fields to exclude from the returned _source field, can be overridden on each sub-request"]
    pub fn _source_excludes(mut self, _source_excludes: &'a [&'a str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "Default list of fields to extract and return from the _source field, can be overridden on each sub-request"]
    pub fn _source_includes(mut self, _source_includes: &'a [&'a str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Bulk<'a, T>
    where
        T: Serialize,
    {
        Bulk {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pipeline: self.pipeline,
            pretty: self.pretty,
            refresh: self.refresh,
            routing: self.routing,
            source: self.source,
            timeout: self.timeout,
            ty: self.ty,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "The pipeline id to preprocess incoming documents with"]
    pub fn pipeline(mut self, pipeline: &'a str) -> Self {
        self.pipeline = Some(pipeline);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` then refresh the effected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Default document type for items which don't provide one"]
    pub fn ty(mut self, ty: &'a str) -> Self {
        self.ty = Some(ty);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the bulk operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous request to the Bulk API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<&'a [&'a str]>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pipeline", skip_serializing_if = "Option::is_none")]
                pipeline: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
                #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
                ty: Option<&'a str>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pipeline: self.pipeline,
                pretty: self.pretty,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                ty: self.ty,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Clear Scroll API"]
pub enum ClearScrollUrlParts<'a> {
    None,
    ScrollId(&'a [&'a str]),
}
impl<'a> ClearScrollUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ClearScrollUrlParts::None => "/_search/scroll".into(),
            ClearScrollUrlParts::ScrollId(ref scroll_id) => {
                let scroll_id_str = scroll_id.join(",");
                let mut p = String::with_capacity(16usize + scroll_id_str.len());
                p.push_str("/_search/scroll/");
                p.push_str(scroll_id_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Clear Scroll API"]
pub struct ClearScroll<'a, B> {
    client: Elasticsearch,
    parts: ClearScrollUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> ClearScroll<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: ClearScrollUrlParts<'a>) -> Self {
        ClearScroll {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClearScroll<'a, T>
    where
        T: Serialize,
    {
        ClearScroll {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Clear Scroll API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Count API"]
pub enum CountUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
    IndexType(&'a [&'a str], &'a [&'a str]),
}
impl<'a> CountUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CountUrlParts::None => "/_count".into(),
            CountUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(8usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_count");
                p.into()
            }
            CountUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(9usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_count");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Count API"]
pub struct Count<'a, B> {
    client: Elasticsearch,
    parts: CountUrlParts<'a>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'a str>,
    body: Option<B>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'a str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    min_score: Option<i64>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    q: Option<&'a str>,
    routing: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    terminate_after: Option<i64>,
}
impl<'a, B> Count<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: CountUrlParts<'a>) -> Self {
        Count {
            client,
            parts,
            allow_no_indices: None,
            analyze_wildcard: None,
            analyzer: None,
            body: None,
            default_operator: None,
            df: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            lenient: None,
            min_score: None,
            preference: None,
            pretty: None,
            q: None,
            routing: None,
            source: None,
            terminate_after: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "The analyzer to use for the query string"]
    pub fn analyzer(mut self, analyzer: &'a str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Count<'a, T>
    where
        T: Serialize,
    {
        Count {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            allow_no_indices: self.allow_no_indices,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            default_operator: self.default_operator,
            df: self.df,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            human: self.human,
            ignore_throttled: self.ignore_throttled,
            ignore_unavailable: self.ignore_unavailable,
            lenient: self.lenient,
            min_score: self.min_score,
            preference: self.preference,
            pretty: self.pretty,
            q: self.q,
            routing: self.routing,
            source: self.source,
            terminate_after: self.terminate_after,
        }
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: &'a str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether specified concrete, expanded or aliased indices should be ignored when throttled"]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Include only documents with a specific `_score` value in the result"]
    pub fn min_score(mut self, min_score: i64) -> Self {
        self.min_score = Some(min_score);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: &'a str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: &'a [&'a str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The maximum count for each shard, upon reaching which the query execution will terminate early"]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Creates an asynchronous request to the Count API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard", skip_serializing_if = "Option::is_none")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer", skip_serializing_if = "Option::is_none")]
                analyzer: Option<&'a str>,
                #[serde(rename = "default_operator", skip_serializing_if = "Option::is_none")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df", skip_serializing_if = "Option::is_none")]
                df: Option<&'a str>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_throttled", skip_serializing_if = "Option::is_none")]
                ignore_throttled: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient", skip_serializing_if = "Option::is_none")]
                lenient: Option<bool>,
                #[serde(rename = "min_score", skip_serializing_if = "Option::is_none")]
                min_score: Option<i64>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
                q: Option<&'a str>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                routing: Option<&'a [&'a str]>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "terminate_after", skip_serializing_if = "Option::is_none")]
                terminate_after: Option<i64>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                min_score: self.min_score,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                routing: self.routing,
                source: self.source,
                terminate_after: self.terminate_after,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Create API"]
pub enum CreateUrlParts<'a> {
    IndexId(&'a str, &'a str),
    IndexTypeId(&'a str, &'a str, &'a str),
}
impl<'a> CreateUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CreateUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(10usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_create/");
                p.push_str(id.as_ref());
                p.into()
            }
            CreateUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.push_str("/_create");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Create API"]
pub struct Create<'a, B> {
    client: Elasticsearch,
    parts: CreateUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pipeline: Option<&'a str>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> Create<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: CreateUrlParts<'a>) -> Self {
        Create {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pipeline: None,
            pretty: None,
            refresh: None,
            routing: None,
            source: None,
            timeout: None,
            version: None,
            version_type: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Create<'a, T>
    where
        T: Serialize,
    {
        Create {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pipeline: self.pipeline,
            pretty: self.pretty,
            refresh: self.refresh,
            routing: self.routing,
            source: self.source,
            timeout: self.timeout,
            version: self.version,
            version_type: self.version_type,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "The pipeline id to preprocess incoming documents with"]
    pub fn pipeline(mut self, pipeline: &'a str) -> Self {
        self.pipeline = Some(pipeline);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the index operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous request to the Create API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pipeline", skip_serializing_if = "Option::is_none")]
                pipeline: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pipeline: self.pipeline,
                pretty: self.pretty,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                version: self.version,
                version_type: self.version_type,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Delete API"]
pub enum DeleteUrlParts<'a> {
    IndexId(&'a str, &'a str),
    IndexTypeId(&'a str, &'a str, &'a str),
}
impl<'a> DeleteUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DeleteUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(7usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc/");
                p.push_str(id.as_ref());
                p.into()
            }
            DeleteUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Delete API"]
pub struct Delete<'a> {
    client: Elasticsearch,
    parts: DeleteUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a> Delete<'a> {
    pub fn new(client: Elasticsearch, parts: DeleteUrlParts<'a>) -> Self {
        Delete {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            pretty: None,
            refresh: None,
            routing: None,
            source: None,
            timeout: None,
            version: None,
            version_type: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "only perform the delete operation if the last operation that has changed the document has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "only perform the delete operation if the last operation that has changed the document has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` then refresh the effected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the delete operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous request to the Delete API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "if_primary_term", skip_serializing_if = "Option::is_none")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no", skip_serializing_if = "Option::is_none")]
                if_seq_no: Option<i64>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                pretty: self.pretty,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                version: self.version,
                version_type: self.version_type,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Delete By Query API"]
pub enum DeleteByQueryUrlParts<'a> {
    Index(&'a [&'a str]),
    IndexType(&'a [&'a str], &'a [&'a str]),
}
impl<'a> DeleteByQueryUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DeleteByQueryUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(18usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_delete_by_query");
                p.into()
            }
            DeleteByQueryUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(19usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_delete_by_query");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Delete By Query API"]
pub struct DeleteByQuery<'a, B> {
    client: Elasticsearch,
    parts: DeleteByQueryUrlParts<'a>,
    _source: Option<&'a [&'a str]>,
    _source_excludes: Option<&'a [&'a str]>,
    _source_includes: Option<&'a [&'a str]>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    body: Option<B>,
    conflicts: Option<Conflicts>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'a str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i64>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    max_docs: Option<i64>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    q: Option<&'a str>,
    refresh: Option<bool>,
    request_cache: Option<bool>,
    requests_per_second: Option<i64>,
    routing: Option<&'a [&'a str]>,
    scroll: Option<&'a str>,
    scroll_size: Option<i64>,
    search_timeout: Option<&'a str>,
    search_type: Option<SearchType>,
    size: Option<i64>,
    slices: Option<i64>,
    sort: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    stats: Option<&'a [&'a str]>,
    terminate_after: Option<i64>,
    timeout: Option<&'a str>,
    version: Option<bool>,
    wait_for_active_shards: Option<&'a str>,
    wait_for_completion: Option<bool>,
}
impl<'a, B> DeleteByQuery<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: DeleteByQueryUrlParts<'a>) -> Self {
        DeleteByQuery {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            allow_no_indices: None,
            analyze_wildcard: None,
            body: None,
            conflicts: None,
            default_operator: None,
            df: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            from: None,
            human: None,
            ignore_unavailable: None,
            lenient: None,
            max_docs: None,
            preference: None,
            pretty: None,
            q: None,
            refresh: None,
            request_cache: None,
            requests_per_second: None,
            routing: None,
            scroll: None,
            scroll_size: None,
            search_timeout: None,
            search_type: None,
            size: None,
            slices: None,
            sort: None,
            source: None,
            stats: None,
            terminate_after: None,
            timeout: None,
            version: None,
            wait_for_active_shards: None,
            wait_for_completion: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: &'a [&'a str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'a [&'a str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'a [&'a str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> DeleteByQuery<'a, T>
    where
        T: Serialize,
    {
        DeleteByQuery {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            allow_no_indices: self.allow_no_indices,
            analyze_wildcard: self.analyze_wildcard,
            conflicts: self.conflicts,
            default_operator: self.default_operator,
            df: self.df,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            from: self.from,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            lenient: self.lenient,
            max_docs: self.max_docs,
            preference: self.preference,
            pretty: self.pretty,
            q: self.q,
            refresh: self.refresh,
            request_cache: self.request_cache,
            requests_per_second: self.requests_per_second,
            routing: self.routing,
            scroll: self.scroll,
            scroll_size: self.scroll_size,
            search_timeout: self.search_timeout,
            search_type: self.search_type,
            size: self.size,
            slices: self.slices,
            sort: self.sort,
            source: self.source,
            stats: self.stats,
            terminate_after: self.terminate_after,
            timeout: self.timeout,
            version: self.version,
            wait_for_active_shards: self.wait_for_active_shards,
            wait_for_completion: self.wait_for_completion,
        }
    }
    #[doc = "What to do when the delete by query hits version conflicts?"]
    pub fn conflicts(mut self, conflicts: Conflicts) -> Self {
        self.conflicts = Some(conflicts);
        self
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: &'a str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Starting offset (default: 0)"]
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Maximum number of documents to process (default: all documents)"]
    pub fn max_docs(mut self, max_docs: i64) -> Self {
        self.max_docs = Some(max_docs);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: &'a str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "Should the effected indexes be refreshed?"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specify if request cache should be used for this request or not, defaults to index level setting"]
    pub fn request_cache(mut self, request_cache: bool) -> Self {
        self.request_cache = Some(request_cache);
        self
    }
    #[doc = "The throttle for this request in sub-requests per second. -1 means no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: i64) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: &'a [&'a str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: &'a str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "Size on the scroll request powering the delete by query"]
    pub fn scroll_size(mut self, scroll_size: i64) -> Self {
        self.scroll_size = Some(scroll_size);
        self
    }
    #[doc = "Explicit timeout for each search request. Defaults to no timeout."]
    pub fn search_timeout(mut self, search_timeout: &'a str) -> Self {
        self.search_timeout = Some(search_timeout);
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "Deprecated, please use `max_docs` instead"]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The number of slices this task should be divided into. Defaults to 1 meaning the task isn't sliced into subtasks."]
    pub fn slices(mut self, slices: i64) -> Self {
        self.slices = Some(slices);
        self
    }
    #[doc = "A comma-separated list of <field>:<direction> pairs"]
    pub fn sort(mut self, sort: &'a [&'a str]) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specific 'tag' of the request for logging and statistical purposes"]
    pub fn stats(mut self, stats: &'a [&'a str]) -> Self {
        self.stats = Some(stats);
        self
    }
    #[doc = "The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early."]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Time each individual bulk request should wait for shards that are unavailable."]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Specify whether to return document version as part of a hit"]
    pub fn version(mut self, version: bool) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the delete by query operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Should the request should block until the delete by query is complete."]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous request to the Delete By Query API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<&'a [&'a str]>,
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard", skip_serializing_if = "Option::is_none")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "conflicts", skip_serializing_if = "Option::is_none")]
                conflicts: Option<Conflicts>,
                #[serde(rename = "default_operator", skip_serializing_if = "Option::is_none")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df", skip_serializing_if = "Option::is_none")]
                df: Option<&'a str>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i64>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient", skip_serializing_if = "Option::is_none")]
                lenient: Option<bool>,
                #[serde(rename = "max_docs", skip_serializing_if = "Option::is_none")]
                max_docs: Option<i64>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
                q: Option<&'a str>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "request_cache", skip_serializing_if = "Option::is_none")]
                request_cache: Option<bool>,
                #[serde(
                    rename = "requests_per_second",
                    skip_serializing_if = "Option::is_none"
                )]
                requests_per_second: Option<i64>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                routing: Option<&'a [&'a str]>,
                #[serde(rename = "scroll", skip_serializing_if = "Option::is_none")]
                scroll: Option<&'a str>,
                #[serde(rename = "scroll_size", skip_serializing_if = "Option::is_none")]
                scroll_size: Option<i64>,
                #[serde(rename = "search_timeout", skip_serializing_if = "Option::is_none")]
                search_timeout: Option<&'a str>,
                #[serde(rename = "search_type", skip_serializing_if = "Option::is_none")]
                search_type: Option<SearchType>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i64>,
                #[serde(rename = "slices", skip_serializing_if = "Option::is_none")]
                slices: Option<i64>,
                #[serde(
                    rename = "sort",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                sort: Option<&'a [&'a str]>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(
                    rename = "stats",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stats: Option<&'a [&'a str]>,
                #[serde(rename = "terminate_after", skip_serializing_if = "Option::is_none")]
                terminate_after: Option<i64>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<bool>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<&'a str>,
                #[serde(
                    rename = "wait_for_completion",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                allow_no_indices: self.allow_no_indices,
                analyze_wildcard: self.analyze_wildcard,
                conflicts: self.conflicts,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                max_docs: self.max_docs,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                refresh: self.refresh,
                request_cache: self.request_cache,
                requests_per_second: self.requests_per_second,
                routing: self.routing,
                scroll: self.scroll,
                scroll_size: self.scroll_size,
                search_timeout: self.search_timeout,
                search_type: self.search_type,
                size: self.size,
                slices: self.slices,
                sort: self.sort,
                source: self.source,
                stats: self.stats,
                terminate_after: self.terminate_after,
                timeout: self.timeout,
                version: self.version,
                wait_for_active_shards: self.wait_for_active_shards,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Delete By Query Rethrottle API"]
pub enum DeleteByQueryRethrottleUrlParts<'a> {
    TaskId(&'a str),
}
impl<'a> DeleteByQueryRethrottleUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DeleteByQueryRethrottleUrlParts::TaskId(ref task_id) => {
                let mut p = String::with_capacity(30usize + task_id.len());
                p.push_str("/_delete_by_query/");
                p.push_str(task_id.as_ref());
                p.push_str("/_rethrottle");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Delete By Query Rethrottle API"]
pub struct DeleteByQueryRethrottle<'a, B> {
    client: Elasticsearch,
    parts: DeleteByQueryRethrottleUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<&'a str>,
}
impl<'a, B> DeleteByQueryRethrottle<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: DeleteByQueryRethrottleUrlParts<'a>) -> Self {
        DeleteByQueryRethrottle {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            requests_per_second: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> DeleteByQueryRethrottle<'a, T>
    where
        T: Serialize,
    {
        DeleteByQueryRethrottle {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            requests_per_second: self.requests_per_second,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    #[doc = "The throttle to set on this request in floating sub-requests per second. -1 means set no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: i64) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Delete By Query Rethrottle API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(
                    rename = "requests_per_second",
                    skip_serializing_if = "Option::is_none"
                )]
                requests_per_second: Option<i64>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                requests_per_second: self.requests_per_second,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Delete Script API"]
pub enum DeleteScriptUrlParts<'a> {
    Id(&'a str),
}
impl<'a> DeleteScriptUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DeleteScriptUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(10usize + id.len());
                p.push_str("/_scripts/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Delete Script API"]
pub struct DeleteScript<'a> {
    client: Elasticsearch,
    parts: DeleteScriptUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> DeleteScript<'a> {
    pub fn new(client: Elasticsearch, parts: DeleteScriptUrlParts<'a>) -> Self {
        DeleteScript {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Delete Script API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Exists API"]
pub enum ExistsUrlParts<'a> {
    IndexId(&'a str, &'a str),
    IndexTypeId(&'a str, &'a str, &'a str),
}
impl<'a> ExistsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ExistsUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(7usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc/");
                p.push_str(id.as_ref());
                p.into()
            }
            ExistsUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Exists API"]
pub struct Exists<'a> {
    client: Elasticsearch,
    parts: ExistsUrlParts<'a>,
    _source: Option<&'a [&'a str]>,
    _source_excludes: Option<&'a [&'a str]>,
    _source_includes: Option<&'a [&'a str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
    stored_fields: Option<&'a [&'a str]>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a> Exists<'a> {
    pub fn new(client: Elasticsearch, parts: ExistsUrlParts<'a>) -> Self {
        Exists {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            routing: None,
            source: None,
            stored_fields: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: &'a [&'a str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'a [&'a str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'a [&'a str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "A comma-separated list of stored fields to return in the response"]
    pub fn stored_fields(mut self, stored_fields: &'a [&'a str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous request to the Exists API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Head;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<&'a [&'a str]>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stored_fields: Option<&'a [&'a str]>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                stored_fields: self.stored_fields,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Exists Source API"]
pub enum ExistsSourceUrlParts<'a> {
    IndexId(&'a str, &'a str),
    IndexTypeId(&'a str, &'a str, &'a str),
}
impl<'a> ExistsSourceUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ExistsSourceUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(10usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_source/");
                p.push_str(id.as_ref());
                p.into()
            }
            ExistsSourceUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.push_str("/_source");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Exists Source API"]
pub struct ExistsSource<'a> {
    client: Elasticsearch,
    parts: ExistsSourceUrlParts<'a>,
    _source: Option<&'a [&'a str]>,
    _source_excludes: Option<&'a [&'a str]>,
    _source_includes: Option<&'a [&'a str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a> ExistsSource<'a> {
    pub fn new(client: Elasticsearch, parts: ExistsSourceUrlParts<'a>) -> Self {
        ExistsSource {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            routing: None,
            source: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: &'a [&'a str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'a [&'a str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'a [&'a str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous request to the Exists Source API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Head;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<&'a [&'a str]>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Explain API"]
pub enum ExplainUrlParts<'a> {
    IndexId(&'a str, &'a str),
    IndexTypeId(&'a str, &'a str, &'a str),
}
impl<'a> ExplainUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ExplainUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(11usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_explain/");
                p.push_str(id.as_ref());
                p.into()
            }
            ExplainUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(12usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.push_str("/_explain");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Explain API"]
pub struct Explain<'a, B> {
    client: Elasticsearch,
    parts: ExplainUrlParts<'a>,
    _source: Option<&'a [&'a str]>,
    _source_excludes: Option<&'a [&'a str]>,
    _source_includes: Option<&'a [&'a str]>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'a str>,
    body: Option<B>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'a str>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    lenient: Option<bool>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    q: Option<&'a str>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
    stored_fields: Option<&'a [&'a str]>,
}
impl<'a, B> Explain<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: ExplainUrlParts<'a>) -> Self {
        Explain {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            analyze_wildcard: None,
            analyzer: None,
            body: None,
            default_operator: None,
            df: None,
            error_trace: None,
            filter_path: None,
            human: None,
            lenient: None,
            preference: None,
            pretty: None,
            q: None,
            routing: None,
            source: None,
            stored_fields: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: &'a [&'a str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'a [&'a str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'a [&'a str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Specify whether wildcards and prefix queries in the query string query should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "The analyzer for the query string query"]
    pub fn analyzer(mut self, analyzer: &'a str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Explain<'a, T>
    where
        T: Serialize,
    {
        Explain {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            default_operator: self.default_operator,
            df: self.df,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            lenient: self.lenient,
            preference: self.preference,
            pretty: self.pretty,
            q: self.q,
            routing: self.routing,
            source: self.source,
            stored_fields: self.stored_fields,
        }
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "The default field for query string query (default: _all)"]
    pub fn df(mut self, df: &'a str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: &'a str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "A comma-separated list of stored fields to return in the response"]
    pub fn stored_fields(mut self, stored_fields: &'a [&'a str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Creates an asynchronous request to the Explain API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<&'a [&'a str]>,
                #[serde(rename = "analyze_wildcard", skip_serializing_if = "Option::is_none")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer", skip_serializing_if = "Option::is_none")]
                analyzer: Option<&'a str>,
                #[serde(rename = "default_operator", skip_serializing_if = "Option::is_none")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df", skip_serializing_if = "Option::is_none")]
                df: Option<&'a str>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "lenient", skip_serializing_if = "Option::is_none")]
                lenient: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
                q: Option<&'a str>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stored_fields: Option<&'a [&'a str]>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                lenient: self.lenient,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                routing: self.routing,
                source: self.source,
                stored_fields: self.stored_fields,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Field Caps API"]
pub enum FieldCapsUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> FieldCapsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            FieldCapsUrlParts::None => "/_field_caps".into(),
            FieldCapsUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(13usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_field_caps");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Field Caps API"]
pub struct FieldCaps<'a, B> {
    client: Elasticsearch,
    parts: FieldCapsUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    fields: Option<&'a [&'a str]>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_unmapped: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> FieldCaps<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: FieldCapsUrlParts<'a>) -> Self {
        FieldCaps {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            fields: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            include_unmapped: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> FieldCaps<'a, T>
    where
        T: Serialize,
    {
        FieldCaps {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            fields: self.fields,
            filter_path: self.filter_path,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            include_unmapped: self.include_unmapped,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of field names"]
    pub fn fields(mut self, fields: &'a [&'a str]) -> Self {
        self.fields = Some(fields);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Indicates whether unmapped fields should be included in the response."]
    pub fn include_unmapped(mut self, include_unmapped: bool) -> Self {
        self.include_unmapped = Some(include_unmapped);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Field Caps API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "fields",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                fields: Option<&'a [&'a str]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_unmapped", skip_serializing_if = "Option::is_none")]
                include_unmapped: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                fields: self.fields,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_unmapped: self.include_unmapped,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Get API"]
pub enum GetUrlParts<'a> {
    IndexId(&'a str, &'a str),
    IndexTypeId(&'a str, &'a str, &'a str),
}
impl<'a> GetUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            GetUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(7usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc/");
                p.push_str(id.as_ref());
                p.into()
            }
            GetUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Get API"]
pub struct Get<'a> {
    client: Elasticsearch,
    parts: GetUrlParts<'a>,
    _source: Option<&'a [&'a str]>,
    _source_excludes: Option<&'a [&'a str]>,
    _source_includes: Option<&'a [&'a str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
    stored_fields: Option<&'a [&'a str]>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a> Get<'a> {
    pub fn new(client: Elasticsearch, parts: GetUrlParts<'a>) -> Self {
        Get {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            routing: None,
            source: None,
            stored_fields: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: &'a [&'a str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'a [&'a str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'a [&'a str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "A comma-separated list of stored fields to return in the response"]
    pub fn stored_fields(mut self, stored_fields: &'a [&'a str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous request to the Get API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<&'a [&'a str]>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stored_fields: Option<&'a [&'a str]>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                stored_fields: self.stored_fields,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Get Script API"]
pub enum GetScriptUrlParts<'a> {
    Id(&'a str),
}
impl<'a> GetScriptUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            GetScriptUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(10usize + id.len());
                p.push_str("/_scripts/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Get Script API"]
pub struct GetScript<'a> {
    client: Elasticsearch,
    parts: GetScriptUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> GetScript<'a> {
    pub fn new(client: Elasticsearch, parts: GetScriptUrlParts<'a>) -> Self {
        GetScript {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Get Script API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Get Source API"]
pub enum GetSourceUrlParts<'a> {
    IndexId(&'a str, &'a str),
    IndexTypeId(&'a str, &'a str, &'a str),
}
impl<'a> GetSourceUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            GetSourceUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(10usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_source/");
                p.push_str(id.as_ref());
                p.into()
            }
            GetSourceUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.push_str("/_source");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Get Source API"]
pub struct GetSource<'a> {
    client: Elasticsearch,
    parts: GetSourceUrlParts<'a>,
    _source: Option<&'a [&'a str]>,
    _source_excludes: Option<&'a [&'a str]>,
    _source_includes: Option<&'a [&'a str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a> GetSource<'a> {
    pub fn new(client: Elasticsearch, parts: GetSourceUrlParts<'a>) -> Self {
        GetSource {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            routing: None,
            source: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: &'a [&'a str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'a [&'a str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'a [&'a str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous request to the Get Source API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<&'a [&'a str]>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Index API"]
pub enum IndexUrlParts<'a> {
    IndexId(&'a str, &'a str),
    Index(&'a str),
    IndexType(&'a str, &'a str),
    IndexTypeId(&'a str, &'a str, &'a str),
}
impl<'a> IndexUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndexUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(7usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc/");
                p.push_str(id.as_ref());
                p.into()
            }
            IndexUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(6usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc");
                p.into()
            }
            IndexUrlParts::IndexType(ref index, ref ty) => {
                let mut p = String::with_capacity(2usize + index.len() + ty.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.into()
            }
            IndexUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Index API"]
pub struct Index<'a, B> {
    client: Elasticsearch,
    parts: IndexUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    op_type: Option<OpType>,
    pipeline: Option<&'a str>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> Index<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: IndexUrlParts<'a>) -> Self {
        Index {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            op_type: None,
            pipeline: None,
            pretty: None,
            refresh: None,
            routing: None,
            source: None,
            timeout: None,
            version: None,
            version_type: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Index<'a, T>
    where
        T: Serialize,
    {
        Index {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            if_primary_term: self.if_primary_term,
            if_seq_no: self.if_seq_no,
            op_type: self.op_type,
            pipeline: self.pipeline,
            pretty: self.pretty,
            refresh: self.refresh,
            routing: self.routing,
            source: self.source,
            timeout: self.timeout,
            version: self.version,
            version_type: self.version_type,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "only perform the index operation if the last operation that has changed the document has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "only perform the index operation if the last operation that has changed the document has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
        self
    }
    #[doc = "Explicit operation type"]
    pub fn op_type(mut self, op_type: OpType) -> Self {
        self.op_type = Some(op_type);
        self
    }
    #[doc = "The pipeline id to preprocess incoming documents with"]
    pub fn pipeline(mut self, pipeline: &'a str) -> Self {
        self.pipeline = Some(pipeline);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the index operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous request to the Index API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "if_primary_term", skip_serializing_if = "Option::is_none")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no", skip_serializing_if = "Option::is_none")]
                if_seq_no: Option<i64>,
                #[serde(rename = "op_type", skip_serializing_if = "Option::is_none")]
                op_type: Option<OpType>,
                #[serde(rename = "pipeline", skip_serializing_if = "Option::is_none")]
                pipeline: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                op_type: self.op_type,
                pipeline: self.pipeline,
                pretty: self.pretty,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                version: self.version,
                version_type: self.version_type,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Info API"]
pub enum InfoUrlParts {
    None,
}
impl InfoUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            InfoUrlParts::None => "/".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Info API"]
pub struct Info<'a> {
    client: Elasticsearch,
    parts: InfoUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> Info<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        Info {
            client,
            parts: InfoUrlParts::None,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Info API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Mget API"]
pub enum MgetUrlParts<'a> {
    None,
    Index(&'a str),
    IndexType(&'a str, &'a str),
}
impl<'a> MgetUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MgetUrlParts::None => "/_mget".into(),
            MgetUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(7usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_mget");
                p.into()
            }
            MgetUrlParts::IndexType(ref index, ref ty) => {
                let mut p = String::with_capacity(8usize + index.len() + ty.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/_mget");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Mget API"]
pub struct Mget<'a, B> {
    client: Elasticsearch,
    parts: MgetUrlParts<'a>,
    _source: Option<&'a [&'a str]>,
    _source_excludes: Option<&'a [&'a str]>,
    _source_includes: Option<&'a [&'a str]>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
    stored_fields: Option<&'a [&'a str]>,
}
impl<'a, B> Mget<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MgetUrlParts<'a>) -> Self {
        Mget {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            routing: None,
            source: None,
            stored_fields: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: &'a [&'a str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'a [&'a str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'a [&'a str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Mget<'a, T>
    where
        T: Serialize,
    {
        Mget {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            preference: self.preference,
            pretty: self.pretty,
            realtime: self.realtime,
            refresh: self.refresh,
            routing: self.routing,
            source: self.source,
            stored_fields: self.stored_fields,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "A comma-separated list of stored fields to return in the response"]
    pub fn stored_fields(mut self, stored_fields: &'a [&'a str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Creates an asynchronous request to the Mget API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<&'a [&'a str]>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stored_fields: Option<&'a [&'a str]>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                stored_fields: self.stored_fields,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Msearch API"]
pub enum MsearchUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
    IndexType(&'a [&'a str], &'a [&'a str]),
}
impl<'a> MsearchUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MsearchUrlParts::None => "/_msearch".into(),
            MsearchUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_msearch");
                p.into()
            }
            MsearchUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(11usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_msearch");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Msearch API"]
pub struct Msearch<'a, B> {
    client: Elasticsearch,
    parts: MsearchUrlParts<'a>,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    max_concurrent_searches: Option<i64>,
    max_concurrent_shard_requests: Option<i64>,
    pre_filter_shard_size: Option<i64>,
    pretty: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    search_type: Option<SearchType>,
    source: Option<&'a str>,
    typed_keys: Option<bool>,
}
impl<'a, B> Msearch<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MsearchUrlParts<'a>) -> Self {
        Msearch {
            client,
            parts,
            body: None,
            ccs_minimize_roundtrips: None,
            error_trace: None,
            filter_path: None,
            human: None,
            max_concurrent_searches: None,
            max_concurrent_shard_requests: None,
            pre_filter_shard_size: None,
            pretty: None,
            rest_total_hits_as_int: None,
            search_type: None,
            source: None,
            typed_keys: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Msearch<'a, T>
    where
        T: Serialize,
    {
        Msearch {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            max_concurrent_searches: self.max_concurrent_searches,
            max_concurrent_shard_requests: self.max_concurrent_shard_requests,
            pre_filter_shard_size: self.pre_filter_shard_size,
            pretty: self.pretty,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            search_type: self.search_type,
            source: self.source,
            typed_keys: self.typed_keys,
        }
    }
    #[doc = "Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution"]
    pub fn ccs_minimize_roundtrips(mut self, ccs_minimize_roundtrips: bool) -> Self {
        self.ccs_minimize_roundtrips = Some(ccs_minimize_roundtrips);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Controls the maximum number of concurrent searches the multi search api will execute"]
    pub fn max_concurrent_searches(mut self, max_concurrent_searches: i64) -> Self {
        self.max_concurrent_searches = Some(max_concurrent_searches);
        self
    }
    #[doc = "The number of concurrent shard requests each sub search executes concurrently per node. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests"]
    pub fn max_concurrent_shard_requests(mut self, max_concurrent_shard_requests: i64) -> Self {
        self.max_concurrent_shard_requests = Some(max_concurrent_shard_requests);
        self
    }
    #[doc = "A threshold that enforces a pre-filter roundtrip to prefilter search shards based on query rewriting if the\u{a0}number of shards the search request expands to exceeds the threshold. This filter roundtrip can limit the number of shards significantly if for instance a shard can not match any documents based on it's rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint."]
    pub fn pre_filter_shard_size(mut self, pre_filter_shard_size: i64) -> Self {
        self.pre_filter_shard_size = Some(pre_filter_shard_size);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "Creates an asynchronous request to the Msearch API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "ccs_minimize_roundtrips",
                    skip_serializing_if = "Option::is_none"
                )]
                ccs_minimize_roundtrips: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(
                    rename = "max_concurrent_searches",
                    skip_serializing_if = "Option::is_none"
                )]
                max_concurrent_searches: Option<i64>,
                #[serde(
                    rename = "max_concurrent_shard_requests",
                    skip_serializing_if = "Option::is_none"
                )]
                max_concurrent_shard_requests: Option<i64>,
                #[serde(
                    rename = "pre_filter_shard_size",
                    skip_serializing_if = "Option::is_none"
                )]
                pre_filter_shard_size: Option<i64>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(
                    rename = "rest_total_hits_as_int",
                    skip_serializing_if = "Option::is_none"
                )]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "search_type", skip_serializing_if = "Option::is_none")]
                search_type: Option<SearchType>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "typed_keys", skip_serializing_if = "Option::is_none")]
                typed_keys: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                max_concurrent_searches: self.max_concurrent_searches,
                max_concurrent_shard_requests: self.max_concurrent_shard_requests,
                pre_filter_shard_size: self.pre_filter_shard_size,
                pretty: self.pretty,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                search_type: self.search_type,
                source: self.source,
                typed_keys: self.typed_keys,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Msearch Template API"]
pub enum MsearchTemplateUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
    IndexType(&'a [&'a str], &'a [&'a str]),
}
impl<'a> MsearchTemplateUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MsearchTemplateUrlParts::None => "/_msearch/template".into(),
            MsearchTemplateUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(19usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_msearch/template");
                p.into()
            }
            MsearchTemplateUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(20usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_msearch/template");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Msearch Template API"]
pub struct MsearchTemplate<'a, B> {
    client: Elasticsearch,
    parts: MsearchTemplateUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    max_concurrent_searches: Option<i64>,
    pretty: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    search_type: Option<SearchType>,
    source: Option<&'a str>,
    typed_keys: Option<bool>,
}
impl<'a, B> MsearchTemplate<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MsearchTemplateUrlParts<'a>) -> Self {
        MsearchTemplate {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            max_concurrent_searches: None,
            pretty: None,
            rest_total_hits_as_int: None,
            search_type: None,
            source: None,
            typed_keys: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MsearchTemplate<'a, T>
    where
        T: Serialize,
    {
        MsearchTemplate {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            max_concurrent_searches: self.max_concurrent_searches,
            pretty: self.pretty,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            search_type: self.search_type,
            source: self.source,
            typed_keys: self.typed_keys,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Controls the maximum number of concurrent searches the multi search api will execute"]
    pub fn max_concurrent_searches(mut self, max_concurrent_searches: i64) -> Self {
        self.max_concurrent_searches = Some(max_concurrent_searches);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "Creates an asynchronous request to the Msearch Template API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(
                    rename = "max_concurrent_searches",
                    skip_serializing_if = "Option::is_none"
                )]
                max_concurrent_searches: Option<i64>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(
                    rename = "rest_total_hits_as_int",
                    skip_serializing_if = "Option::is_none"
                )]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "search_type", skip_serializing_if = "Option::is_none")]
                search_type: Option<SearchType>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "typed_keys", skip_serializing_if = "Option::is_none")]
                typed_keys: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                max_concurrent_searches: self.max_concurrent_searches,
                pretty: self.pretty,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                search_type: self.search_type,
                source: self.source,
                typed_keys: self.typed_keys,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Mtermvectors API"]
pub enum MtermvectorsUrlParts<'a> {
    None,
    Index(&'a str),
    IndexType(&'a str, &'a str),
}
impl<'a> MtermvectorsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MtermvectorsUrlParts::None => "/_mtermvectors".into(),
            MtermvectorsUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(15usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_mtermvectors");
                p.into()
            }
            MtermvectorsUrlParts::IndexType(ref index, ref ty) => {
                let mut p = String::with_capacity(16usize + index.len() + ty.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/_mtermvectors");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Mtermvectors API"]
pub struct Mtermvectors<'a, B> {
    client: Elasticsearch,
    parts: MtermvectorsUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    field_statistics: Option<bool>,
    fields: Option<&'a [&'a str]>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ids: Option<&'a [&'a str]>,
    offsets: Option<bool>,
    payloads: Option<bool>,
    positions: Option<bool>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
    term_statistics: Option<bool>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, B> Mtermvectors<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MtermvectorsUrlParts<'a>) -> Self {
        Mtermvectors {
            client,
            parts,
            body: None,
            error_trace: None,
            field_statistics: None,
            fields: None,
            filter_path: None,
            human: None,
            ids: None,
            offsets: None,
            payloads: None,
            positions: None,
            preference: None,
            pretty: None,
            realtime: None,
            routing: None,
            source: None,
            term_statistics: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Mtermvectors<'a, T>
    where
        T: Serialize,
    {
        Mtermvectors {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            field_statistics: self.field_statistics,
            fields: self.fields,
            filter_path: self.filter_path,
            human: self.human,
            ids: self.ids,
            offsets: self.offsets,
            payloads: self.payloads,
            positions: self.positions,
            preference: self.preference,
            pretty: self.pretty,
            realtime: self.realtime,
            routing: self.routing,
            source: self.source,
            term_statistics: self.term_statistics,
            version: self.version,
            version_type: self.version_type,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn field_statistics(mut self, field_statistics: bool) -> Self {
        self.field_statistics = Some(field_statistics);
        self
    }
    #[doc = "A comma-separated list of fields to return. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn fields(mut self, fields: &'a [&'a str]) -> Self {
        self.fields = Some(fields);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "A comma-separated list of documents ids. You must define ids as parameter or set \"ids\" or \"docs\" in the request body"]
    pub fn ids(mut self, ids: &'a [&'a str]) -> Self {
        self.ids = Some(ids);
        self
    }
    #[doc = "Specifies if term offsets should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn offsets(mut self, offsets: bool) -> Self {
        self.offsets = Some(offsets);
        self
    }
    #[doc = "Specifies if term payloads should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn payloads(mut self, payloads: bool) -> Self {
        self.payloads = Some(payloads);
        self
    }
    #[doc = "Specifies if term positions should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn positions(mut self, positions: bool) -> Self {
        self.positions = Some(positions);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random) .Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specifies if requests are real-time as opposed to near-real-time (default: true)."]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Specific routing value. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specifies if total term frequency and document frequency should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn term_statistics(mut self, term_statistics: bool) -> Self {
        self.term_statistics = Some(term_statistics);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous request to the Mtermvectors API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "field_statistics", skip_serializing_if = "Option::is_none")]
                field_statistics: Option<bool>,
                #[serde(
                    rename = "fields",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                fields: Option<&'a [&'a str]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(
                    rename = "ids",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                ids: Option<&'a [&'a str]>,
                #[serde(rename = "offsets", skip_serializing_if = "Option::is_none")]
                offsets: Option<bool>,
                #[serde(rename = "payloads", skip_serializing_if = "Option::is_none")]
                payloads: Option<bool>,
                #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
                positions: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "term_statistics", skip_serializing_if = "Option::is_none")]
                term_statistics: Option<bool>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                field_statistics: self.field_statistics,
                fields: self.fields,
                filter_path: self.filter_path,
                human: self.human,
                ids: self.ids,
                offsets: self.offsets,
                payloads: self.payloads,
                positions: self.positions,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                routing: self.routing,
                source: self.source,
                term_statistics: self.term_statistics,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ping API"]
pub enum PingUrlParts {
    None,
}
impl PingUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            PingUrlParts::None => "/".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ping API"]
pub struct Ping<'a> {
    client: Elasticsearch,
    parts: PingUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> Ping<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        Ping {
            client,
            parts: PingUrlParts::None,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ping API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Head;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Put Script API"]
pub enum PutScriptUrlParts<'a> {
    Id(&'a str),
    IdContext(&'a str, &'a str),
}
impl<'a> PutScriptUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            PutScriptUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(10usize + id.len());
                p.push_str("/_scripts/");
                p.push_str(id.as_ref());
                p.into()
            }
            PutScriptUrlParts::IdContext(ref id, ref context) => {
                let mut p = String::with_capacity(11usize + id.len() + context.len());
                p.push_str("/_scripts/");
                p.push_str(id.as_ref());
                p.push_str("/");
                p.push_str(context.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Put Script API"]
pub struct PutScript<'a, B> {
    client: Elasticsearch,
    parts: PutScriptUrlParts<'a>,
    body: Option<B>,
    context: Option<&'a str>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> PutScript<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: PutScriptUrlParts<'a>) -> Self {
        PutScript {
            client,
            parts,
            body: None,
            context: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> PutScript<'a, T>
    where
        T: Serialize,
    {
        PutScript {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            context: self.context,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Context name to compile script against"]
    pub fn context(mut self, context: &'a str) -> Self {
        self.context = Some(context);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Put Script API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
                context: Option<&'a str>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                context: self.context,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Rank Eval API"]
pub enum RankEvalUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> RankEvalUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            RankEvalUrlParts::None => "/_rank_eval".into(),
            RankEvalUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(12usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_rank_eval");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Rank Eval API"]
pub struct RankEval<'a, B> {
    client: Elasticsearch,
    parts: RankEvalUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> RankEval<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: RankEvalUrlParts<'a>) -> Self {
        RankEval {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> RankEval<'a, T>
    where
        T: Serialize,
    {
        RankEval {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            human: self.human,
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Rank Eval API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Reindex API"]
pub enum ReindexUrlParts {
    None,
}
impl ReindexUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ReindexUrlParts::None => "/_reindex".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Reindex API"]
pub struct Reindex<'a, B> {
    client: Elasticsearch,
    parts: ReindexUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    max_docs: Option<i64>,
    pretty: Option<bool>,
    refresh: Option<bool>,
    requests_per_second: Option<i64>,
    scroll: Option<&'a str>,
    slices: Option<i64>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
    wait_for_completion: Option<bool>,
}
impl<'a, B> Reindex<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        Reindex {
            client,
            parts: ReindexUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            max_docs: None,
            pretty: None,
            refresh: None,
            requests_per_second: None,
            scroll: None,
            slices: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
            wait_for_completion: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Reindex<'a, T>
    where
        T: Serialize,
    {
        Reindex {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            max_docs: self.max_docs,
            pretty: self.pretty,
            refresh: self.refresh,
            requests_per_second: self.requests_per_second,
            scroll: self.scroll,
            slices: self.slices,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
            wait_for_completion: self.wait_for_completion,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Maximum number of documents to process (default: all documents)"]
    pub fn max_docs(mut self, max_docs: i64) -> Self {
        self.max_docs = Some(max_docs);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Should the effected indexes be refreshed?"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The throttle to set on this request in sub-requests per second. -1 means no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: i64) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "Control how long to keep the search context alive"]
    pub fn scroll(mut self, scroll: &'a str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "The number of slices this task should be divided into. Defaults to 1 meaning the task isn't sliced into subtasks."]
    pub fn slices(mut self, slices: i64) -> Self {
        self.slices = Some(slices);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Time each individual bulk request should wait for shards that are unavailable."]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the reindex operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Should the request should block until the reindex is complete."]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous request to the Reindex API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "max_docs", skip_serializing_if = "Option::is_none")]
                max_docs: Option<i64>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(
                    rename = "requests_per_second",
                    skip_serializing_if = "Option::is_none"
                )]
                requests_per_second: Option<i64>,
                #[serde(rename = "scroll", skip_serializing_if = "Option::is_none")]
                scroll: Option<&'a str>,
                #[serde(rename = "slices", skip_serializing_if = "Option::is_none")]
                slices: Option<i64>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<&'a str>,
                #[serde(
                    rename = "wait_for_completion",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                max_docs: self.max_docs,
                pretty: self.pretty,
                refresh: self.refresh,
                requests_per_second: self.requests_per_second,
                scroll: self.scroll,
                slices: self.slices,
                source: self.source,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Reindex Rethrottle API"]
pub enum ReindexRethrottleUrlParts<'a> {
    TaskId(&'a str),
}
impl<'a> ReindexRethrottleUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ReindexRethrottleUrlParts::TaskId(ref task_id) => {
                let mut p = String::with_capacity(22usize + task_id.len());
                p.push_str("/_reindex/");
                p.push_str(task_id.as_ref());
                p.push_str("/_rethrottle");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Reindex Rethrottle API"]
pub struct ReindexRethrottle<'a, B> {
    client: Elasticsearch,
    parts: ReindexRethrottleUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<&'a str>,
}
impl<'a, B> ReindexRethrottle<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: ReindexRethrottleUrlParts<'a>) -> Self {
        ReindexRethrottle {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            requests_per_second: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ReindexRethrottle<'a, T>
    where
        T: Serialize,
    {
        ReindexRethrottle {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            requests_per_second: self.requests_per_second,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    #[doc = "The throttle to set on this request in floating sub-requests per second. -1 means set no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: i64) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Reindex Rethrottle API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(
                    rename = "requests_per_second",
                    skip_serializing_if = "Option::is_none"
                )]
                requests_per_second: Option<i64>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                requests_per_second: self.requests_per_second,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Render Search Template API"]
pub enum RenderSearchTemplateUrlParts<'a> {
    None,
    Id(&'a str),
}
impl<'a> RenderSearchTemplateUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            RenderSearchTemplateUrlParts::None => "/_render/template".into(),
            RenderSearchTemplateUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(18usize + id.len());
                p.push_str("/_render/template/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Render Search Template API"]
pub struct RenderSearchTemplate<'a, B> {
    client: Elasticsearch,
    parts: RenderSearchTemplateUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> RenderSearchTemplate<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: RenderSearchTemplateUrlParts<'a>) -> Self {
        RenderSearchTemplate {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> RenderSearchTemplate<'a, T>
    where
        T: Serialize,
    {
        RenderSearchTemplate {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Render Search Template API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Scripts Painless Execute API"]
pub enum ScriptsPainlessExecuteUrlParts {
    None,
}
impl ScriptsPainlessExecuteUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ScriptsPainlessExecuteUrlParts::None => "/_scripts/painless/_execute".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Scripts Painless Execute API"]
pub struct ScriptsPainlessExecute<'a, B> {
    client: Elasticsearch,
    parts: ScriptsPainlessExecuteUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> ScriptsPainlessExecute<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        ScriptsPainlessExecute {
            client,
            parts: ScriptsPainlessExecuteUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ScriptsPainlessExecute<'a, T>
    where
        T: Serialize,
    {
        ScriptsPainlessExecute {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Scripts Painless Execute API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Scroll API"]
pub enum ScrollUrlParts<'a> {
    None,
    ScrollId(&'a str),
}
impl<'a> ScrollUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ScrollUrlParts::None => "/_search/scroll".into(),
            ScrollUrlParts::ScrollId(ref scroll_id) => {
                let mut p = String::with_capacity(16usize + scroll_id.len());
                p.push_str("/_search/scroll/");
                p.push_str(scroll_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Scroll API"]
pub struct Scroll<'a, B> {
    client: Elasticsearch,
    parts: ScrollUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    scroll: Option<&'a str>,
    scroll_id: Option<&'a str>,
    source: Option<&'a str>,
}
impl<'a, B> Scroll<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: ScrollUrlParts<'a>) -> Self {
        Scroll {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            rest_total_hits_as_int: None,
            scroll: None,
            scroll_id: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Scroll<'a, T>
    where
        T: Serialize,
    {
        Scroll {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            scroll: self.scroll,
            scroll_id: self.scroll_id,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: &'a str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "The scroll ID for scrolled search"]
    pub fn scroll_id(mut self, scroll_id: &'a str) -> Self {
        self.scroll_id = Some(scroll_id);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Scroll API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(
                    rename = "rest_total_hits_as_int",
                    skip_serializing_if = "Option::is_none"
                )]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "scroll", skip_serializing_if = "Option::is_none")]
                scroll: Option<&'a str>,
                #[serde(rename = "scroll_id", skip_serializing_if = "Option::is_none")]
                scroll_id: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                scroll: self.scroll,
                scroll_id: self.scroll_id,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Search API"]
pub enum SearchUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
    IndexType(&'a [&'a str], &'a [&'a str]),
}
impl<'a> SearchUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SearchUrlParts::None => "/_search".into(),
            SearchUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(9usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_search");
                p.into()
            }
            SearchUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(10usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_search");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Search API"]
pub struct Search<'a, B> {
    client: Elasticsearch,
    parts: SearchUrlParts<'a>,
    _source: Option<&'a [&'a str]>,
    _source_excludes: Option<&'a [&'a str]>,
    _source_includes: Option<&'a [&'a str]>,
    allow_no_indices: Option<bool>,
    allow_partial_search_results: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'a str>,
    batched_reduce_size: Option<i64>,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'a str>,
    docvalue_fields: Option<&'a [&'a str]>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    explain: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i64>,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    max_concurrent_shard_requests: Option<i64>,
    pre_filter_shard_size: Option<i64>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    q: Option<&'a str>,
    request_cache: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    routing: Option<&'a [&'a str]>,
    scroll: Option<&'a str>,
    search_type: Option<SearchType>,
    seq_no_primary_term: Option<bool>,
    size: Option<i64>,
    sort: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    stats: Option<&'a [&'a str]>,
    stored_fields: Option<&'a [&'a str]>,
    suggest_field: Option<&'a str>,
    suggest_mode: Option<SuggestMode>,
    suggest_size: Option<i64>,
    suggest_text: Option<&'a str>,
    terminate_after: Option<i64>,
    timeout: Option<&'a str>,
    track_scores: Option<bool>,
    track_total_hits: Option<bool>,
    typed_keys: Option<bool>,
    version: Option<bool>,
}
impl<'a, B> Search<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SearchUrlParts<'a>) -> Self {
        Search {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            allow_no_indices: None,
            allow_partial_search_results: None,
            analyze_wildcard: None,
            analyzer: None,
            batched_reduce_size: None,
            body: None,
            ccs_minimize_roundtrips: None,
            default_operator: None,
            df: None,
            docvalue_fields: None,
            error_trace: None,
            expand_wildcards: None,
            explain: None,
            filter_path: None,
            from: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            lenient: None,
            max_concurrent_shard_requests: None,
            pre_filter_shard_size: None,
            preference: None,
            pretty: None,
            q: None,
            request_cache: None,
            rest_total_hits_as_int: None,
            routing: None,
            scroll: None,
            search_type: None,
            seq_no_primary_term: None,
            size: None,
            sort: None,
            source: None,
            stats: None,
            stored_fields: None,
            suggest_field: None,
            suggest_mode: None,
            suggest_size: None,
            suggest_text: None,
            terminate_after: None,
            timeout: None,
            track_scores: None,
            track_total_hits: None,
            typed_keys: None,
            version: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: &'a [&'a str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'a [&'a str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'a [&'a str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Indicate if an error should be returned if there is a partial search failure or timeout"]
    pub fn allow_partial_search_results(mut self, allow_partial_search_results: bool) -> Self {
        self.allow_partial_search_results = Some(allow_partial_search_results);
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "The analyzer to use for the query string"]
    pub fn analyzer(mut self, analyzer: &'a str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The number of shard results that should be reduced at once on the coordinating node. This value should be used as a protection mechanism to reduce the memory overhead per search request if the potential number of shards in the request can be large."]
    pub fn batched_reduce_size(mut self, batched_reduce_size: i64) -> Self {
        self.batched_reduce_size = Some(batched_reduce_size);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Search<'a, T>
    where
        T: Serialize,
    {
        Search {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            allow_no_indices: self.allow_no_indices,
            allow_partial_search_results: self.allow_partial_search_results,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            batched_reduce_size: self.batched_reduce_size,
            ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
            default_operator: self.default_operator,
            df: self.df,
            docvalue_fields: self.docvalue_fields,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            explain: self.explain,
            filter_path: self.filter_path,
            from: self.from,
            human: self.human,
            ignore_throttled: self.ignore_throttled,
            ignore_unavailable: self.ignore_unavailable,
            lenient: self.lenient,
            max_concurrent_shard_requests: self.max_concurrent_shard_requests,
            pre_filter_shard_size: self.pre_filter_shard_size,
            preference: self.preference,
            pretty: self.pretty,
            q: self.q,
            request_cache: self.request_cache,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            routing: self.routing,
            scroll: self.scroll,
            search_type: self.search_type,
            seq_no_primary_term: self.seq_no_primary_term,
            size: self.size,
            sort: self.sort,
            source: self.source,
            stats: self.stats,
            stored_fields: self.stored_fields,
            suggest_field: self.suggest_field,
            suggest_mode: self.suggest_mode,
            suggest_size: self.suggest_size,
            suggest_text: self.suggest_text,
            terminate_after: self.terminate_after,
            timeout: self.timeout,
            track_scores: self.track_scores,
            track_total_hits: self.track_total_hits,
            typed_keys: self.typed_keys,
            version: self.version,
        }
    }
    #[doc = "Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution"]
    pub fn ccs_minimize_roundtrips(mut self, ccs_minimize_roundtrips: bool) -> Self {
        self.ccs_minimize_roundtrips = Some(ccs_minimize_roundtrips);
        self
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: &'a str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "A comma-separated list of fields to return as the docvalue representation of a field for each hit"]
    pub fn docvalue_fields(mut self, docvalue_fields: &'a [&'a str]) -> Self {
        self.docvalue_fields = Some(docvalue_fields);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "Specify whether to return detailed information about score computation as part of a hit"]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Starting offset (default: 0)"]
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether specified concrete, expanded or aliased indices should be ignored when throttled"]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "The number of concurrent shard requests per node this search executes concurrently. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests"]
    pub fn max_concurrent_shard_requests(mut self, max_concurrent_shard_requests: i64) -> Self {
        self.max_concurrent_shard_requests = Some(max_concurrent_shard_requests);
        self
    }
    #[doc = "A threshold that enforces a pre-filter roundtrip to prefilter search shards based on query rewriting if the\u{a0}number of shards the search request expands to exceeds the threshold. This filter roundtrip can limit the number of shards significantly if for instance a shard can not match any documents based on it's rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint."]
    pub fn pre_filter_shard_size(mut self, pre_filter_shard_size: i64) -> Self {
        self.pre_filter_shard_size = Some(pre_filter_shard_size);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: &'a str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "Specify if request cache should be used for this request or not, defaults to index level setting"]
    pub fn request_cache(mut self, request_cache: bool) -> Self {
        self.request_cache = Some(request_cache);
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: &'a [&'a str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: &'a str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "Specify whether to return sequence number and primary term of the last modification of each hit"]
    pub fn seq_no_primary_term(mut self, seq_no_primary_term: bool) -> Self {
        self.seq_no_primary_term = Some(seq_no_primary_term);
        self
    }
    #[doc = "Number of hits to return (default: 10)"]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "A comma-separated list of <field>:<direction> pairs"]
    pub fn sort(mut self, sort: &'a [&'a str]) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specific 'tag' of the request for logging and statistical purposes"]
    pub fn stats(mut self, stats: &'a [&'a str]) -> Self {
        self.stats = Some(stats);
        self
    }
    #[doc = "A comma-separated list of stored fields to return as part of a hit"]
    pub fn stored_fields(mut self, stored_fields: &'a [&'a str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Specify which field to use for suggestions"]
    pub fn suggest_field(mut self, suggest_field: &'a str) -> Self {
        self.suggest_field = Some(suggest_field);
        self
    }
    #[doc = "Specify suggest mode"]
    pub fn suggest_mode(mut self, suggest_mode: SuggestMode) -> Self {
        self.suggest_mode = Some(suggest_mode);
        self
    }
    #[doc = "How many suggestions to return in response"]
    pub fn suggest_size(mut self, suggest_size: i64) -> Self {
        self.suggest_size = Some(suggest_size);
        self
    }
    #[doc = "The source text for which the suggestions should be returned"]
    pub fn suggest_text(mut self, suggest_text: &'a str) -> Self {
        self.suggest_text = Some(suggest_text);
        self
    }
    #[doc = "The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early."]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Whether to calculate and return scores even if they are not used for sorting"]
    pub fn track_scores(mut self, track_scores: bool) -> Self {
        self.track_scores = Some(track_scores);
        self
    }
    #[doc = "Indicate if the number of documents that match the query should be tracked"]
    pub fn track_total_hits(mut self, track_total_hits: bool) -> Self {
        self.track_total_hits = Some(track_total_hits);
        self
    }
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "Specify whether to return document version as part of a hit"]
    pub fn version(mut self, version: bool) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Creates an asynchronous request to the Search API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<&'a [&'a str]>,
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(
                    rename = "allow_partial_search_results",
                    skip_serializing_if = "Option::is_none"
                )]
                allow_partial_search_results: Option<bool>,
                #[serde(rename = "analyze_wildcard", skip_serializing_if = "Option::is_none")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer", skip_serializing_if = "Option::is_none")]
                analyzer: Option<&'a str>,
                #[serde(
                    rename = "batched_reduce_size",
                    skip_serializing_if = "Option::is_none"
                )]
                batched_reduce_size: Option<i64>,
                #[serde(
                    rename = "ccs_minimize_roundtrips",
                    skip_serializing_if = "Option::is_none"
                )]
                ccs_minimize_roundtrips: Option<bool>,
                #[serde(rename = "default_operator", skip_serializing_if = "Option::is_none")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df", skip_serializing_if = "Option::is_none")]
                df: Option<&'a str>,
                #[serde(
                    rename = "docvalue_fields",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                docvalue_fields: Option<&'a [&'a str]>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "explain", skip_serializing_if = "Option::is_none")]
                explain: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i64>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_throttled", skip_serializing_if = "Option::is_none")]
                ignore_throttled: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient", skip_serializing_if = "Option::is_none")]
                lenient: Option<bool>,
                #[serde(
                    rename = "max_concurrent_shard_requests",
                    skip_serializing_if = "Option::is_none"
                )]
                max_concurrent_shard_requests: Option<i64>,
                #[serde(
                    rename = "pre_filter_shard_size",
                    skip_serializing_if = "Option::is_none"
                )]
                pre_filter_shard_size: Option<i64>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
                q: Option<&'a str>,
                #[serde(rename = "request_cache", skip_serializing_if = "Option::is_none")]
                request_cache: Option<bool>,
                #[serde(
                    rename = "rest_total_hits_as_int",
                    skip_serializing_if = "Option::is_none"
                )]
                rest_total_hits_as_int: Option<bool>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                routing: Option<&'a [&'a str]>,
                #[serde(rename = "scroll", skip_serializing_if = "Option::is_none")]
                scroll: Option<&'a str>,
                #[serde(rename = "search_type", skip_serializing_if = "Option::is_none")]
                search_type: Option<SearchType>,
                #[serde(
                    rename = "seq_no_primary_term",
                    skip_serializing_if = "Option::is_none"
                )]
                seq_no_primary_term: Option<bool>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i64>,
                #[serde(
                    rename = "sort",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                sort: Option<&'a [&'a str]>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(
                    rename = "stats",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stats: Option<&'a [&'a str]>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stored_fields: Option<&'a [&'a str]>,
                #[serde(rename = "suggest_field", skip_serializing_if = "Option::is_none")]
                suggest_field: Option<&'a str>,
                #[serde(rename = "suggest_mode", skip_serializing_if = "Option::is_none")]
                suggest_mode: Option<SuggestMode>,
                #[serde(rename = "suggest_size", skip_serializing_if = "Option::is_none")]
                suggest_size: Option<i64>,
                #[serde(rename = "suggest_text", skip_serializing_if = "Option::is_none")]
                suggest_text: Option<&'a str>,
                #[serde(rename = "terminate_after", skip_serializing_if = "Option::is_none")]
                terminate_after: Option<i64>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
                #[serde(rename = "track_scores", skip_serializing_if = "Option::is_none")]
                track_scores: Option<bool>,
                #[serde(rename = "track_total_hits", skip_serializing_if = "Option::is_none")]
                track_total_hits: Option<bool>,
                #[serde(rename = "typed_keys", skip_serializing_if = "Option::is_none")]
                typed_keys: Option<bool>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                allow_no_indices: self.allow_no_indices,
                allow_partial_search_results: self.allow_partial_search_results,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                batched_reduce_size: self.batched_reduce_size,
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
                default_operator: self.default_operator,
                df: self.df,
                docvalue_fields: self.docvalue_fields,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                explain: self.explain,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                max_concurrent_shard_requests: self.max_concurrent_shard_requests,
                pre_filter_shard_size: self.pre_filter_shard_size,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                request_cache: self.request_cache,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                routing: self.routing,
                scroll: self.scroll,
                search_type: self.search_type,
                seq_no_primary_term: self.seq_no_primary_term,
                size: self.size,
                sort: self.sort,
                source: self.source,
                stats: self.stats,
                stored_fields: self.stored_fields,
                suggest_field: self.suggest_field,
                suggest_mode: self.suggest_mode,
                suggest_size: self.suggest_size,
                suggest_text: self.suggest_text,
                terminate_after: self.terminate_after,
                timeout: self.timeout,
                track_scores: self.track_scores,
                track_total_hits: self.track_total_hits,
                typed_keys: self.typed_keys,
                version: self.version,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Search Shards API"]
pub enum SearchShardsUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> SearchShardsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SearchShardsUrlParts::None => "/_search_shards".into(),
            SearchShardsUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(16usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_search_shards");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Search Shards API"]
pub struct SearchShards<'a, B> {
    client: Elasticsearch,
    parts: SearchShardsUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
}
impl<'a, B> SearchShards<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SearchShardsUrlParts<'a>) -> Self {
        SearchShards {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            local: None,
            preference: None,
            pretty: None,
            routing: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SearchShards<'a, T>
    where
        T: Serialize,
    {
        SearchShards {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            local: self.local,
            preference: self.preference,
            pretty: self.pretty,
            routing: self.routing,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Search Shards API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
                local: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
                preference: self.preference,
                pretty: self.pretty,
                routing: self.routing,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Search Template API"]
pub enum SearchTemplateUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
    IndexType(&'a [&'a str], &'a [&'a str]),
}
impl<'a> SearchTemplateUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SearchTemplateUrlParts::None => "/_search/template".into(),
            SearchTemplateUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(18usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_search/template");
                p.into()
            }
            SearchTemplateUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(19usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_search/template");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Search Template API"]
pub struct SearchTemplate<'a, B> {
    client: Elasticsearch,
    parts: SearchTemplateUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    explain: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    profile: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    routing: Option<&'a [&'a str]>,
    scroll: Option<&'a str>,
    search_type: Option<SearchType>,
    source: Option<&'a str>,
    typed_keys: Option<bool>,
}
impl<'a, B> SearchTemplate<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SearchTemplateUrlParts<'a>) -> Self {
        SearchTemplate {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            explain: None,
            filter_path: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            preference: None,
            pretty: None,
            profile: None,
            rest_total_hits_as_int: None,
            routing: None,
            scroll: None,
            search_type: None,
            source: None,
            typed_keys: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SearchTemplate<'a, T>
    where
        T: Serialize,
    {
        SearchTemplate {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            explain: self.explain,
            filter_path: self.filter_path,
            human: self.human,
            ignore_throttled: self.ignore_throttled,
            ignore_unavailable: self.ignore_unavailable,
            preference: self.preference,
            pretty: self.pretty,
            profile: self.profile,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            routing: self.routing,
            scroll: self.scroll,
            search_type: self.search_type,
            source: self.source,
            typed_keys: self.typed_keys,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "Specify whether to return detailed information about score computation as part of a hit"]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether specified concrete, expanded or aliased indices should be ignored when throttled"]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specify whether to profile the query execution"]
    pub fn profile(mut self, profile: bool) -> Self {
        self.profile = Some(profile);
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: &'a [&'a str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: &'a str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "Creates an asynchronous request to the Search Template API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "explain", skip_serializing_if = "Option::is_none")]
                explain: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_throttled", skip_serializing_if = "Option::is_none")]
                ignore_throttled: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
                profile: Option<bool>,
                #[serde(
                    rename = "rest_total_hits_as_int",
                    skip_serializing_if = "Option::is_none"
                )]
                rest_total_hits_as_int: Option<bool>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                routing: Option<&'a [&'a str]>,
                #[serde(rename = "scroll", skip_serializing_if = "Option::is_none")]
                scroll: Option<&'a str>,
                #[serde(rename = "search_type", skip_serializing_if = "Option::is_none")]
                search_type: Option<SearchType>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "typed_keys", skip_serializing_if = "Option::is_none")]
                typed_keys: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                explain: self.explain,
                filter_path: self.filter_path,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                preference: self.preference,
                pretty: self.pretty,
                profile: self.profile,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                routing: self.routing,
                scroll: self.scroll,
                search_type: self.search_type,
                source: self.source,
                typed_keys: self.typed_keys,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Termvectors API"]
pub enum TermvectorsUrlParts<'a> {
    IndexId(&'a str, &'a str),
    Index(&'a str),
    IndexTypeId(&'a str, &'a str, &'a str),
    IndexType(&'a str, &'a str),
}
impl<'a> TermvectorsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            TermvectorsUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(15usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_termvectors/");
                p.push_str(id.as_ref());
                p.into()
            }
            TermvectorsUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(14usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_termvectors");
                p.into()
            }
            TermvectorsUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(16usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.push_str("/_termvectors");
                p.into()
            }
            TermvectorsUrlParts::IndexType(ref index, ref ty) => {
                let mut p = String::with_capacity(15usize + index.len() + ty.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/_termvectors");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Termvectors API"]
pub struct Termvectors<'a, B> {
    client: Elasticsearch,
    parts: TermvectorsUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    field_statistics: Option<bool>,
    fields: Option<&'a [&'a str]>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    offsets: Option<bool>,
    payloads: Option<bool>,
    positions: Option<bool>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
    term_statistics: Option<bool>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, B> Termvectors<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: TermvectorsUrlParts<'a>) -> Self {
        Termvectors {
            client,
            parts,
            body: None,
            error_trace: None,
            field_statistics: None,
            fields: None,
            filter_path: None,
            human: None,
            offsets: None,
            payloads: None,
            positions: None,
            preference: None,
            pretty: None,
            realtime: None,
            routing: None,
            source: None,
            term_statistics: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Termvectors<'a, T>
    where
        T: Serialize,
    {
        Termvectors {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            field_statistics: self.field_statistics,
            fields: self.fields,
            filter_path: self.filter_path,
            human: self.human,
            offsets: self.offsets,
            payloads: self.payloads,
            positions: self.positions,
            preference: self.preference,
            pretty: self.pretty,
            realtime: self.realtime,
            routing: self.routing,
            source: self.source,
            term_statistics: self.term_statistics,
            version: self.version,
            version_type: self.version_type,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned."]
    pub fn field_statistics(mut self, field_statistics: bool) -> Self {
        self.field_statistics = Some(field_statistics);
        self
    }
    #[doc = "A comma-separated list of fields to return."]
    pub fn fields(mut self, fields: &'a [&'a str]) -> Self {
        self.fields = Some(fields);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Specifies if term offsets should be returned."]
    pub fn offsets(mut self, offsets: bool) -> Self {
        self.offsets = Some(offsets);
        self
    }
    #[doc = "Specifies if term payloads should be returned."]
    pub fn payloads(mut self, payloads: bool) -> Self {
        self.payloads = Some(payloads);
        self
    }
    #[doc = "Specifies if term positions should be returned."]
    pub fn positions(mut self, positions: bool) -> Self {
        self.positions = Some(positions);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)."]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specifies if request is real-time as opposed to near-real-time (default: true)."]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Specific routing value."]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specifies if total term frequency and document frequency should be returned."]
    pub fn term_statistics(mut self, term_statistics: bool) -> Self {
        self.term_statistics = Some(term_statistics);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous request to the Termvectors API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "field_statistics", skip_serializing_if = "Option::is_none")]
                field_statistics: Option<bool>,
                #[serde(
                    rename = "fields",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                fields: Option<&'a [&'a str]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "offsets", skip_serializing_if = "Option::is_none")]
                offsets: Option<bool>,
                #[serde(rename = "payloads", skip_serializing_if = "Option::is_none")]
                payloads: Option<bool>,
                #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
                positions: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "term_statistics", skip_serializing_if = "Option::is_none")]
                term_statistics: Option<bool>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                field_statistics: self.field_statistics,
                fields: self.fields,
                filter_path: self.filter_path,
                human: self.human,
                offsets: self.offsets,
                payloads: self.payloads,
                positions: self.positions,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                routing: self.routing,
                source: self.source,
                term_statistics: self.term_statistics,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Update API"]
pub enum UpdateUrlParts<'a> {
    IndexId(&'a str, &'a str),
    IndexTypeId(&'a str, &'a str, &'a str),
}
impl<'a> UpdateUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            UpdateUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(10usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_update/");
                p.push_str(id.as_ref());
                p.into()
            }
            UpdateUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Update API"]
pub struct Update<'a, B> {
    client: Elasticsearch,
    parts: UpdateUrlParts<'a>,
    _source: Option<&'a [&'a str]>,
    _source_excludes: Option<&'a [&'a str]>,
    _source_includes: Option<&'a [&'a str]>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    lang: Option<&'a str>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    retry_on_conflict: Option<i64>,
    routing: Option<&'a str>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> Update<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: UpdateUrlParts<'a>) -> Self {
        Update {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            lang: None,
            pretty: None,
            refresh: None,
            retry_on_conflict: None,
            routing: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: &'a [&'a str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'a [&'a str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'a [&'a str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Update<'a, T>
    where
        T: Serialize,
    {
        Update {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            if_primary_term: self.if_primary_term,
            if_seq_no: self.if_seq_no,
            lang: self.lang,
            pretty: self.pretty,
            refresh: self.refresh,
            retry_on_conflict: self.retry_on_conflict,
            routing: self.routing,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "only perform the update operation if the last operation that has changed the document has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "only perform the update operation if the last operation that has changed the document has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
        self
    }
    #[doc = "The script language (default: painless)"]
    pub fn lang(mut self, lang: &'a str) -> Self {
        self.lang = Some(lang);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` then refresh the effected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specify how many times should the operation be retried when a conflict occurs (default: 0)"]
    pub fn retry_on_conflict(mut self, retry_on_conflict: i64) -> Self {
        self.retry_on_conflict = Some(retry_on_conflict);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the update operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous request to the Update API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<&'a [&'a str]>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "if_primary_term", skip_serializing_if = "Option::is_none")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no", skip_serializing_if = "Option::is_none")]
                if_seq_no: Option<i64>,
                #[serde(rename = "lang", skip_serializing_if = "Option::is_none")]
                lang: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<Refresh>,
                #[serde(rename = "retry_on_conflict", skip_serializing_if = "Option::is_none")]
                retry_on_conflict: Option<i64>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<&'a str>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                lang: self.lang,
                pretty: self.pretty,
                refresh: self.refresh,
                retry_on_conflict: self.retry_on_conflict,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Update By Query API"]
pub enum UpdateByQueryUrlParts<'a> {
    Index(&'a [&'a str]),
    IndexType(&'a [&'a str], &'a [&'a str]),
}
impl<'a> UpdateByQueryUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            UpdateByQueryUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(18usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_update_by_query");
                p.into()
            }
            UpdateByQueryUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(19usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_update_by_query");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Update By Query API"]
pub struct UpdateByQuery<'a, B> {
    client: Elasticsearch,
    parts: UpdateByQueryUrlParts<'a>,
    _source: Option<&'a [&'a str]>,
    _source_excludes: Option<&'a [&'a str]>,
    _source_includes: Option<&'a [&'a str]>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'a str>,
    body: Option<B>,
    conflicts: Option<Conflicts>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'a str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i64>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    max_docs: Option<i64>,
    pipeline: Option<&'a str>,
    preference: Option<&'a str>,
    pretty: Option<bool>,
    q: Option<&'a str>,
    refresh: Option<bool>,
    request_cache: Option<bool>,
    requests_per_second: Option<i64>,
    routing: Option<&'a [&'a str]>,
    scroll: Option<&'a str>,
    scroll_size: Option<i64>,
    search_timeout: Option<&'a str>,
    search_type: Option<SearchType>,
    size: Option<i64>,
    slices: Option<i64>,
    sort: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    stats: Option<&'a [&'a str]>,
    terminate_after: Option<i64>,
    timeout: Option<&'a str>,
    version: Option<bool>,
    version_type: Option<bool>,
    wait_for_active_shards: Option<&'a str>,
    wait_for_completion: Option<bool>,
}
impl<'a, B> UpdateByQuery<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: UpdateByQueryUrlParts<'a>) -> Self {
        UpdateByQuery {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            allow_no_indices: None,
            analyze_wildcard: None,
            analyzer: None,
            body: None,
            conflicts: None,
            default_operator: None,
            df: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            from: None,
            human: None,
            ignore_unavailable: None,
            lenient: None,
            max_docs: None,
            pipeline: None,
            preference: None,
            pretty: None,
            q: None,
            refresh: None,
            request_cache: None,
            requests_per_second: None,
            routing: None,
            scroll: None,
            scroll_size: None,
            search_timeout: None,
            search_type: None,
            size: None,
            slices: None,
            sort: None,
            source: None,
            stats: None,
            terminate_after: None,
            timeout: None,
            version: None,
            version_type: None,
            wait_for_active_shards: None,
            wait_for_completion: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: &'a [&'a str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'a [&'a str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'a [&'a str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "The analyzer to use for the query string"]
    pub fn analyzer(mut self, analyzer: &'a str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> UpdateByQuery<'a, T>
    where
        T: Serialize,
    {
        UpdateByQuery {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            allow_no_indices: self.allow_no_indices,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            conflicts: self.conflicts,
            default_operator: self.default_operator,
            df: self.df,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            from: self.from,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            lenient: self.lenient,
            max_docs: self.max_docs,
            pipeline: self.pipeline,
            preference: self.preference,
            pretty: self.pretty,
            q: self.q,
            refresh: self.refresh,
            request_cache: self.request_cache,
            requests_per_second: self.requests_per_second,
            routing: self.routing,
            scroll: self.scroll,
            scroll_size: self.scroll_size,
            search_timeout: self.search_timeout,
            search_type: self.search_type,
            size: self.size,
            slices: self.slices,
            sort: self.sort,
            source: self.source,
            stats: self.stats,
            terminate_after: self.terminate_after,
            timeout: self.timeout,
            version: self.version,
            version_type: self.version_type,
            wait_for_active_shards: self.wait_for_active_shards,
            wait_for_completion: self.wait_for_completion,
        }
    }
    #[doc = "What to do when the update by query hits version conflicts?"]
    pub fn conflicts(mut self, conflicts: Conflicts) -> Self {
        self.conflicts = Some(conflicts);
        self
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: &'a str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Starting offset (default: 0)"]
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Maximum number of documents to process (default: all documents)"]
    pub fn max_docs(mut self, max_docs: i64) -> Self {
        self.max_docs = Some(max_docs);
        self
    }
    #[doc = "Ingest pipeline to set on index requests made by this action. (default: none)"]
    pub fn pipeline(mut self, pipeline: &'a str) -> Self {
        self.pipeline = Some(pipeline);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'a str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: &'a str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "Should the effected indexes be refreshed?"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specify if request cache should be used for this request or not, defaults to index level setting"]
    pub fn request_cache(mut self, request_cache: bool) -> Self {
        self.request_cache = Some(request_cache);
        self
    }
    #[doc = "The throttle to set on this request in sub-requests per second. -1 means no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: i64) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: &'a [&'a str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: &'a str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "Size on the scroll request powering the update by query"]
    pub fn scroll_size(mut self, scroll_size: i64) -> Self {
        self.scroll_size = Some(scroll_size);
        self
    }
    #[doc = "Explicit timeout for each search request. Defaults to no timeout."]
    pub fn search_timeout(mut self, search_timeout: &'a str) -> Self {
        self.search_timeout = Some(search_timeout);
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "Deprecated, please use `max_docs` instead"]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The number of slices this task should be divided into. Defaults to 1 meaning the task isn't sliced into subtasks."]
    pub fn slices(mut self, slices: i64) -> Self {
        self.slices = Some(slices);
        self
    }
    #[doc = "A comma-separated list of <field>:<direction> pairs"]
    pub fn sort(mut self, sort: &'a [&'a str]) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specific 'tag' of the request for logging and statistical purposes"]
    pub fn stats(mut self, stats: &'a [&'a str]) -> Self {
        self.stats = Some(stats);
        self
    }
    #[doc = "The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early."]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Time each individual bulk request should wait for shards that are unavailable."]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Specify whether to return document version as part of a hit"]
    pub fn version(mut self, version: bool) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Should the document increment the version number (internal) on hit or not (reindex)"]
    pub fn version_type(mut self, version_type: bool) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the update by query operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Should the request should block until the update by query operation is complete."]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous request to the Update By Query API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<&'a [&'a str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<&'a [&'a str]>,
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard", skip_serializing_if = "Option::is_none")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer", skip_serializing_if = "Option::is_none")]
                analyzer: Option<&'a str>,
                #[serde(rename = "conflicts", skip_serializing_if = "Option::is_none")]
                conflicts: Option<Conflicts>,
                #[serde(rename = "default_operator", skip_serializing_if = "Option::is_none")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df", skip_serializing_if = "Option::is_none")]
                df: Option<&'a str>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i64>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient", skip_serializing_if = "Option::is_none")]
                lenient: Option<bool>,
                #[serde(rename = "max_docs", skip_serializing_if = "Option::is_none")]
                max_docs: Option<i64>,
                #[serde(rename = "pipeline", skip_serializing_if = "Option::is_none")]
                pipeline: Option<&'a str>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
                q: Option<&'a str>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "request_cache", skip_serializing_if = "Option::is_none")]
                request_cache: Option<bool>,
                #[serde(
                    rename = "requests_per_second",
                    skip_serializing_if = "Option::is_none"
                )]
                requests_per_second: Option<i64>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                routing: Option<&'a [&'a str]>,
                #[serde(rename = "scroll", skip_serializing_if = "Option::is_none")]
                scroll: Option<&'a str>,
                #[serde(rename = "scroll_size", skip_serializing_if = "Option::is_none")]
                scroll_size: Option<i64>,
                #[serde(rename = "search_timeout", skip_serializing_if = "Option::is_none")]
                search_timeout: Option<&'a str>,
                #[serde(rename = "search_type", skip_serializing_if = "Option::is_none")]
                search_type: Option<SearchType>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i64>,
                #[serde(rename = "slices", skip_serializing_if = "Option::is_none")]
                slices: Option<i64>,
                #[serde(
                    rename = "sort",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                sort: Option<&'a [&'a str]>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(
                    rename = "stats",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stats: Option<&'a [&'a str]>,
                #[serde(rename = "terminate_after", skip_serializing_if = "Option::is_none")]
                terminate_after: Option<i64>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<bool>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<bool>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<&'a str>,
                #[serde(
                    rename = "wait_for_completion",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                allow_no_indices: self.allow_no_indices,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                conflicts: self.conflicts,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                max_docs: self.max_docs,
                pipeline: self.pipeline,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                refresh: self.refresh,
                request_cache: self.request_cache,
                requests_per_second: self.requests_per_second,
                routing: self.routing,
                scroll: self.scroll,
                scroll_size: self.scroll_size,
                search_timeout: self.search_timeout,
                search_type: self.search_type,
                size: self.size,
                slices: self.slices,
                sort: self.sort,
                source: self.source,
                stats: self.stats,
                terminate_after: self.terminate_after,
                timeout: self.timeout,
                version: self.version,
                version_type: self.version_type,
                wait_for_active_shards: self.wait_for_active_shards,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Update By Query Rethrottle API"]
pub enum UpdateByQueryRethrottleUrlParts<'a> {
    TaskId(&'a str),
}
impl<'a> UpdateByQueryRethrottleUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            UpdateByQueryRethrottleUrlParts::TaskId(ref task_id) => {
                let mut p = String::with_capacity(30usize + task_id.len());
                p.push_str("/_update_by_query/");
                p.push_str(task_id.as_ref());
                p.push_str("/_rethrottle");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Update By Query Rethrottle API"]
pub struct UpdateByQueryRethrottle<'a, B> {
    client: Elasticsearch,
    parts: UpdateByQueryRethrottleUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<&'a str>,
}
impl<'a, B> UpdateByQueryRethrottle<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: UpdateByQueryRethrottleUrlParts<'a>) -> Self {
        UpdateByQueryRethrottle {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            requests_per_second: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> UpdateByQueryRethrottle<'a, T>
    where
        T: Serialize,
    {
        UpdateByQueryRethrottle {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            requests_per_second: self.requests_per_second,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    #[doc = "The throttle to set on this request in floating sub-requests per second. -1 means set no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: i64) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Update By Query Rethrottle API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(
                    rename = "requests_per_second",
                    skip_serializing_if = "Option::is_none"
                )]
                requests_per_second: Option<i64>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                requests_per_second: self.requests_per_second,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
impl Elasticsearch {
    #[doc = "Allows to perform multiple index/update/delete operations in a single request."]
    pub fn bulk<'a>(&self, parts: BulkUrlParts<'a>) -> Bulk<'a, ()> {
        Bulk::new(self.clone(), parts)
    }
    #[doc = "Explicitly clears the search context for a scroll."]
    pub fn clear_scroll<'a>(&self, parts: ClearScrollUrlParts<'a>) -> ClearScroll<'a, ()> {
        ClearScroll::new(self.clone(), parts)
    }
    #[doc = "Returns number of documents matching a query."]
    pub fn count<'a>(&self, parts: CountUrlParts<'a>) -> Count<'a, ()> {
        Count::new(self.clone(), parts)
    }
    #[doc = "Creates a new document in the index.\n\nReturns a 409 response when a document with a same ID already exists in the index."]
    pub fn create<'a>(&self, parts: CreateUrlParts<'a>) -> Create<'a, ()> {
        Create::new(self.clone(), parts)
    }
    #[doc = "Removes a document from the index."]
    pub fn delete<'a>(&self, parts: DeleteUrlParts<'a>) -> Delete<'a> {
        Delete::new(self.clone(), parts)
    }
    #[doc = "Deletes documents matching the provided query."]
    pub fn delete_by_query<'a>(&self, parts: DeleteByQueryUrlParts<'a>) -> DeleteByQuery<'a, ()> {
        DeleteByQuery::new(self.clone(), parts)
    }
    #[doc = "Changes the number of requests per second for a particular Delete By Query operation."]
    pub fn delete_by_query_rethrottle<'a>(
        &self,
        parts: DeleteByQueryRethrottleUrlParts<'a>,
    ) -> DeleteByQueryRethrottle<'a, ()> {
        DeleteByQueryRethrottle::new(self.clone(), parts)
    }
    #[doc = "Deletes a script."]
    pub fn delete_script<'a>(&self, parts: DeleteScriptUrlParts<'a>) -> DeleteScript<'a> {
        DeleteScript::new(self.clone(), parts)
    }
    #[doc = "Returns information about whether a document exists in an index."]
    pub fn exists<'a>(&self, parts: ExistsUrlParts<'a>) -> Exists<'a> {
        Exists::new(self.clone(), parts)
    }
    #[doc = "Returns information about whether a document source exists in an index."]
    pub fn exists_source<'a>(&self, parts: ExistsSourceUrlParts<'a>) -> ExistsSource<'a> {
        ExistsSource::new(self.clone(), parts)
    }
    #[doc = "Returns information about why a specific matches (or doesn't match) a query."]
    pub fn explain<'a>(&self, parts: ExplainUrlParts<'a>) -> Explain<'a, ()> {
        Explain::new(self.clone(), parts)
    }
    #[doc = "Returns the information about the capabilities of fields among multiple indices."]
    pub fn field_caps<'a>(&self, parts: FieldCapsUrlParts<'a>) -> FieldCaps<'a, ()> {
        FieldCaps::new(self.clone(), parts)
    }
    #[doc = "Returns a document."]
    pub fn get<'a>(&self, parts: GetUrlParts<'a>) -> Get<'a> {
        Get::new(self.clone(), parts)
    }
    #[doc = "Returns a script."]
    pub fn get_script<'a>(&self, parts: GetScriptUrlParts<'a>) -> GetScript<'a> {
        GetScript::new(self.clone(), parts)
    }
    #[doc = "Returns the source of a document."]
    pub fn get_source<'a>(&self, parts: GetSourceUrlParts<'a>) -> GetSource<'a> {
        GetSource::new(self.clone(), parts)
    }
    #[doc = "Creates or updates a document in an index."]
    pub fn index<'a>(&self, parts: IndexUrlParts<'a>) -> Index<'a, ()> {
        Index::new(self.clone(), parts)
    }
    #[doc = "Returns basic information about the cluster."]
    pub fn info<'a>(&self) -> Info<'a> {
        Info::new(self.clone())
    }
    #[doc = "Allows to get multiple documents in one request."]
    pub fn mget<'a>(&self, parts: MgetUrlParts<'a>) -> Mget<'a, ()> {
        Mget::new(self.clone(), parts)
    }
    #[doc = "Allows to execute several search operations in one request."]
    pub fn msearch<'a>(&self, parts: MsearchUrlParts<'a>) -> Msearch<'a, ()> {
        Msearch::new(self.clone(), parts)
    }
    #[doc = "Allows to execute several search template operations in one request."]
    pub fn msearch_template<'a>(
        &self,
        parts: MsearchTemplateUrlParts<'a>,
    ) -> MsearchTemplate<'a, ()> {
        MsearchTemplate::new(self.clone(), parts)
    }
    #[doc = "Returns multiple termvectors in one request."]
    pub fn mtermvectors<'a>(&self, parts: MtermvectorsUrlParts<'a>) -> Mtermvectors<'a, ()> {
        Mtermvectors::new(self.clone(), parts)
    }
    #[doc = "Returns whether the cluster is running."]
    pub fn ping<'a>(&self) -> Ping<'a> {
        Ping::new(self.clone())
    }
    #[doc = "Creates or updates a script."]
    pub fn put_script<'a>(&self, parts: PutScriptUrlParts<'a>) -> PutScript<'a, ()> {
        PutScript::new(self.clone(), parts)
    }
    #[doc = "Allows to evaluate the quality of ranked search results over a set of typical search queries"]
    pub fn rank_eval<'a>(&self, parts: RankEvalUrlParts<'a>) -> RankEval<'a, ()> {
        RankEval::new(self.clone(), parts)
    }
    #[doc = "Allows to copy documents from one index to another, optionally filtering the source\ndocuments by a query, changing the destination index settings, or fetching the\ndocuments from a remote cluster."]
    pub fn reindex<'a>(&self) -> Reindex<'a, ()> {
        Reindex::new(self.clone())
    }
    #[doc = "Changes the number of requests per second for a particular Reindex operation."]
    pub fn reindex_rethrottle<'a>(
        &self,
        parts: ReindexRethrottleUrlParts<'a>,
    ) -> ReindexRethrottle<'a, ()> {
        ReindexRethrottle::new(self.clone(), parts)
    }
    #[doc = "Allows to use the Mustache language to pre-render a search definition."]
    pub fn render_search_template<'a>(
        &self,
        parts: RenderSearchTemplateUrlParts<'a>,
    ) -> RenderSearchTemplate<'a, ()> {
        RenderSearchTemplate::new(self.clone(), parts)
    }
    #[doc = "Allows an arbitrary script to be executed and a result to be returned"]
    pub fn scripts_painless_execute<'a>(&self) -> ScriptsPainlessExecute<'a, ()> {
        ScriptsPainlessExecute::new(self.clone())
    }
    #[doc = "Allows to retrieve a large numbers of results from a single search request."]
    pub fn scroll<'a>(&self, parts: ScrollUrlParts<'a>) -> Scroll<'a, ()> {
        Scroll::new(self.clone(), parts)
    }
    #[doc = "Returns results matching a query."]
    pub fn search<'a>(&self, parts: SearchUrlParts<'a>) -> Search<'a, ()> {
        Search::new(self.clone(), parts)
    }
    #[doc = "Returns information about the indices and shards that a search request would be executed against."]
    pub fn search_shards<'a>(&self, parts: SearchShardsUrlParts<'a>) -> SearchShards<'a, ()> {
        SearchShards::new(self.clone(), parts)
    }
    #[doc = "Allows to use the Mustache language to pre-render a search definition."]
    pub fn search_template<'a>(&self, parts: SearchTemplateUrlParts<'a>) -> SearchTemplate<'a, ()> {
        SearchTemplate::new(self.clone(), parts)
    }
    #[doc = "Returns information and statistics about terms in the fields of a particular document."]
    pub fn termvectors<'a>(&self, parts: TermvectorsUrlParts<'a>) -> Termvectors<'a, ()> {
        Termvectors::new(self.clone(), parts)
    }
    #[doc = "Updates a document with a script or partial document."]
    pub fn update<'a>(&self, parts: UpdateUrlParts<'a>) -> Update<'a, ()> {
        Update::new(self.clone(), parts)
    }
    #[doc = "Performs an update on every document in the index without changing the source,\nfor example to pick up a mapping change."]
    pub fn update_by_query<'a>(&self, parts: UpdateByQueryUrlParts<'a>) -> UpdateByQuery<'a, ()> {
        UpdateByQuery::new(self.clone(), parts)
    }
    #[doc = "Changes the number of requests per second for a particular Update By Query operation."]
    pub fn update_by_query_rethrottle<'a>(
        &self,
        parts: UpdateByQueryRethrottleUrlParts<'a>,
    ) -> UpdateByQueryRethrottle<'a, ()> {
        UpdateByQueryRethrottle::new(self.clone(), parts)
    }
}