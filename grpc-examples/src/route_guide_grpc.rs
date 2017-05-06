// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait RouteGuide {
    fn GetFeature(&self, o: ::grpc::GrpcRequestOptions, p: super::route_guide::Point) -> ::grpc::result::GrpcResult<super::route_guide::Feature>;

    fn ListFeatures(&self, o: ::grpc::GrpcRequestOptions, p: super::route_guide::Rectangle) -> ::grpc::iter::GrpcIterator<super::route_guide::Feature>;

    fn RecordRoute(&self, o: ::grpc::GrpcRequestOptions, p: ::grpc::iter::GrpcIterator<super::route_guide::Point>) -> ::grpc::result::GrpcResult<super::route_guide::RouteSummary>;

    fn RouteChat(&self, o: ::grpc::GrpcRequestOptions, p: ::grpc::iter::GrpcIterator<super::route_guide::RouteNote>) -> ::grpc::iter::GrpcIterator<super::route_guide::RouteNote>;
}

pub trait RouteGuideAsync {
    fn GetFeature(&self, o: ::grpc::GrpcRequestOptions, p: super::route_guide::Point) -> ::grpc::GrpcSingleResponse<super::route_guide::Feature>;

    fn ListFeatures(&self, o: ::grpc::GrpcRequestOptions, p: super::route_guide::Rectangle) -> ::grpc::GrpcStreamingResponse<super::route_guide::Feature>;

    fn RecordRoute(&self, o: ::grpc::GrpcRequestOptions, p: ::grpc::futures_grpc::GrpcStreamSend<super::route_guide::Point>) -> ::grpc::GrpcSingleResponse<super::route_guide::RouteSummary>;

    fn RouteChat(&self, o: ::grpc::GrpcRequestOptions, p: ::grpc::futures_grpc::GrpcStreamSend<super::route_guide::RouteNote>) -> ::grpc::GrpcStreamingResponse<super::route_guide::RouteNote>;
}

// sync client

pub struct RouteGuideClient {
    async_client: RouteGuideAsyncClient,
}

impl RouteGuideClient {
    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::client::GrpcClientConf) -> ::grpc::result::GrpcResult<Self> {
        RouteGuideAsyncClient::new(host, port, tls, conf).map(|c| {
            RouteGuideClient {
                async_client: c,
            }
        })
    }
}

impl RouteGuide for RouteGuideClient {
    fn GetFeature(&self, o: ::grpc::GrpcRequestOptions, p: super::route_guide::Point) -> ::grpc::result::GrpcResult<super::route_guide::Feature> {
        ::grpc::GrpcSingleResponse::wait_drop_metadata(self.async_client.GetFeature(o, p))
    }

    fn ListFeatures(&self, o: ::grpc::GrpcRequestOptions, p: super::route_guide::Rectangle) -> ::grpc::iter::GrpcIterator<super::route_guide::Feature> {
        ::grpc::rt::stream_to_iter(self.async_client.ListFeatures(o, p))
    }

    fn RecordRoute(&self, o: ::grpc::GrpcRequestOptions, p: ::grpc::iter::GrpcIterator<super::route_guide::Point>) -> ::grpc::result::GrpcResult<super::route_guide::RouteSummary> {
        let p = ::futures::stream::Stream::boxed(::futures::stream::iter(::std::iter::IntoIterator::into_iter(p)));
        ::grpc::GrpcSingleResponse::wait_drop_metadata(self.async_client.RecordRoute(o, p))
    }

    fn RouteChat(&self, o: ::grpc::GrpcRequestOptions, p: ::grpc::iter::GrpcIterator<super::route_guide::RouteNote>) -> ::grpc::iter::GrpcIterator<super::route_guide::RouteNote> {
        let p = ::futures::stream::Stream::boxed(::futures::stream::iter(::std::iter::IntoIterator::into_iter(p)));
        ::grpc::rt::stream_to_iter(self.async_client.RouteChat(o, p))
    }
}

// async client

pub struct RouteGuideAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_GetFeature: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::route_guide::Point, super::route_guide::Feature>>,
    method_ListFeatures: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::route_guide::Rectangle, super::route_guide::Feature>>,
    method_RecordRoute: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::route_guide::Point, super::route_guide::RouteSummary>>,
    method_RouteChat: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::route_guide::RouteNote, super::route_guide::RouteNote>>,
}

impl RouteGuideAsyncClient {
    pub fn with_client(grpc_client: ::grpc::client::GrpcClient) -> Self {
        RouteGuideAsyncClient {
            grpc_client: grpc_client,
            method_GetFeature: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/proto.RouteGuide/GetFeature".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
            method_ListFeatures: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/proto.RouteGuide/ListFeatures".to_string(),
                streaming: ::grpc::method::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
            method_RecordRoute: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/proto.RouteGuide/RecordRoute".to_string(),
                streaming: ::grpc::method::GrpcStreaming::ClientStreaming,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
            method_RouteChat: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/proto.RouteGuide/RouteChat".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::client::GrpcClientConf) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls, conf).map(|c| {
            RouteGuideAsyncClient::with_client(c)
        })
    }
}

impl RouteGuideAsync for RouteGuideAsyncClient {
    fn GetFeature(&self, o: ::grpc::GrpcRequestOptions, p: super::route_guide::Point) -> ::grpc::GrpcSingleResponse<super::route_guide::Feature> {
        self.grpc_client.call_unary(o, p, self.method_GetFeature.clone())
    }

    fn ListFeatures(&self, o: ::grpc::GrpcRequestOptions, p: super::route_guide::Rectangle) -> ::grpc::GrpcStreamingResponse<super::route_guide::Feature> {
        self.grpc_client.call_server_streaming(o, p, self.method_ListFeatures.clone())
    }

    fn RecordRoute(&self, o: ::grpc::GrpcRequestOptions, p: ::grpc::futures_grpc::GrpcStreamSend<super::route_guide::Point>) -> ::grpc::GrpcSingleResponse<super::route_guide::RouteSummary> {
        self.grpc_client.call_client_streaming(o, p, self.method_RecordRoute.clone())
    }

    fn RouteChat(&self, o: ::grpc::GrpcRequestOptions, p: ::grpc::futures_grpc::GrpcStreamSend<super::route_guide::RouteNote>) -> ::grpc::GrpcStreamingResponse<super::route_guide::RouteNote> {
        self.grpc_client.call_bidi(o, p, self.method_RouteChat.clone())
    }
}

// sync server

pub struct RouteGuideServer {
    async_server: RouteGuideAsyncServer,
}

impl ::std::ops::Deref for RouteGuideServer {
    type Target = RouteGuideAsyncServer;

    fn deref(&self) -> &Self::Target {
        &self.async_server
    }
}

struct RouteGuideServerHandlerToAsync {
    handler: ::std::sync::Arc<RouteGuide + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl RouteGuideAsync for RouteGuideServerHandlerToAsync {
    fn GetFeature(&self, o: ::grpc::GrpcRequestOptions, p: super::route_guide::Point) -> ::grpc::GrpcSingleResponse<super::route_guide::Feature> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetFeature(o, p)
        })
    }

    fn ListFeatures(&self, o: ::grpc::GrpcRequestOptions, p: super::route_guide::Rectangle) -> ::grpc::GrpcStreamingResponse<super::route_guide::Feature> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_server_streaming(&self.cpupool, p, move |p| {
            h.ListFeatures(o, p)
        })
    }

    fn RecordRoute(&self, o: ::grpc::GrpcRequestOptions, p: ::grpc::futures_grpc::GrpcStreamSend<super::route_guide::Point>) -> ::grpc::GrpcSingleResponse<super::route_guide::RouteSummary> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_client_streaming(&self.cpupool, p, move |p| {
            h.RecordRoute(o, p)
        })
    }

    fn RouteChat(&self, o: ::grpc::GrpcRequestOptions, p: ::grpc::futures_grpc::GrpcStreamSend<super::route_guide::RouteNote>) -> ::grpc::GrpcStreamingResponse<super::route_guide::RouteNote> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_bidi(&self.cpupool, p, move |p| {
            h.RouteChat(o, p)
        })
    }
}

impl RouteGuideServer {
    pub fn new_plain<A : ::std::net::ToSocketAddrs, H : RouteGuide + Send + Sync + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H) -> Self {
        let h = RouteGuideServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        RouteGuideServer {
            async_server: RouteGuideAsyncServer::new(addr, conf, h),
        }
    }
}

// async server

pub struct RouteGuideAsyncServer {
    pub grpc_server: ::grpc::server::GrpcServer,
}

impl ::std::ops::Deref for RouteGuideAsyncServer {
    type Target = ::grpc::server::GrpcServer;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl RouteGuideAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : RouteGuideAsync + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H) -> Self {
        let service_definition = RouteGuideAsyncServer::new_service_def(h);
        RouteGuideAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new_plain(addr, conf, service_definition),
        }
    }

    pub fn new_service_def<H : RouteGuideAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/proto.RouteGuide/GetFeature".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.GetFeature(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/proto.RouteGuide/ListFeatures".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerServerStreaming::new(move |o, p| handler_copy.ListFeatures(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/proto.RouteGuide/RecordRoute".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::ClientStreaming,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerClientStreaming::new(move |o, p| handler_copy.RecordRoute(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/proto.RouteGuide/RouteChat".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerBidi::new(move |o, p| handler_copy.RouteChat(o, p))
                    },
                ),
            ],
        )
    }
}
