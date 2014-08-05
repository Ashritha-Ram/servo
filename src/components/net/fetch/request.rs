/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use url::Url;
use http::method::{Get, Method};
use http::headers::request::HeaderCollection;

/// A [request context](http://fetch.spec.whatwg.org/#concept-request-context)
pub enum Context {
    Audio, Beacon, CSPreport, Download, Embed, Eventsource,
    Favicon, Fetch, Font, Form, Frame, Hyperlink, IFrame, Image,
    ImageSet, Import, Internal, Location, Manifest, Object, Ping,
    Plugin, Prefetch, Script, ServiceWorker, SharedWorker, Subresource,
    Style, Track, Video, Worker, XMLHttpRequest, XSLT
}

/// A [request context frame type](http://fetch.spec.whatwg.org/#concept-request-context-frame-type)
pub enum ContextFrameType {
    Auxiliary,
    TopLevel,
    Nested,
    ContextNone
}

/// A [referer](http://fetch.spec.whatwg.org/#concept-request-referrer)
pub enum Referer {
    RefererNone,
    Client,
    RefererUrl(Url)
}

/// A [request mode](http://fetch.spec.whatwg.org/#concept-request-mode)
pub enum RequestMode {
    SameOrigin,
    NoCORS,
    CORSMode,
    ForcedPreflightMode
}

/// Request [credentials mode](http://fetch.spec.whatwg.org/#concept-request-credentials-mode)
pub enum CredentialsMode {
    Omit,
    CredentialsSameOrigin,
    Include
}

/// [Response tainting](http://fetch.spec.whatwg.org/#concept-request-response-tainting)
pub enum ResponseTainting {
    Basic,
    CORSTainting,
    Opaque
}

/// A [Request](http://fetch.spec.whatwg.org/#requests) as defined by the Fetch spec
pub struct Request {
    pub method: Method,
    pub url: Url,
    pub headers: HeaderCollection,
    pub unsafe_request: bool,
    pub body: Option<Vec<u8>>,
    pub preserve_content_codings: bool,
    // pub client: GlobalRef, // XXXManishearth copy over only the relevant fields of the global scope,
                              // not the entire scope to avoid the libscript dependency
    pub skip_service_worker: bool,
    pub context: Context,
    pub context_frame_type: ContextFrameType,
    pub origin: Option<Url>,
    pub force_origin_header: bool,
    pub same_origin_data: bool,
    pub referer: Referer,
    pub authentication: bool,
    pub sync: bool,
    pub mode: RequestMode,
    pub credentials_mode: CredentialsMode,
    pub use_url_credentials: bool,
    pub manual_redirect: bool,
    pub redirect_count: uint,
    pub response_tainting: ResponseTainting
}

impl Request {
    pub fn new(url: Url, context: Context) -> Request {
         Request {
            method: Get,
            url: url,
            headers: HeaderCollection::new(),
            unsafe_request: false,
            body: None,
            preserve_content_codings: false,
            skip_service_worker: false,
            context: context,
            context_frame_type: ContextNone,
            origin: None,
            force_origin_header: false,
            same_origin_data: false,
            referer: Client,
            authentication: false,
            sync: false,
            mode: NoCORS,
            credentials_mode: Omit,
            use_url_credentials: false,
            manual_redirect: false,
            redirect_count: 0,
            response_tainting: Basic
        }
    }
}