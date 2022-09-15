#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisualizeContractsRequest {
    #[prost(map="string, string", tag="1")]
    pub sources: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(message, optional, tag="15")]
    pub output_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisualizeStorageRequest {
    #[prost(map="string, string", tag="1")]
    pub sources: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="2")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub contract_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="15")]
    pub output_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The client should decide on what type they are interested in
/// and specify it through `request.output_mask` field. If omitted,
/// all types would be calculated and returned to the client.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisualizeResponse {
    #[prost(bytes="vec", tag="1")]
    pub png: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub svg: ::prost::alloc::vec::Vec<u8>,
}
pub mod solidity_visualizer_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::*;
    use super::solidity_visualizer_server::SolidityVisualizer;
    use tonic::IntoRequest;
    use std::sync::Arc;
    type VisualizeContractsJson = VisualizeContractsRequest;
    type VisualizeStorageJson = VisualizeStorageRequest;
    async fn call_visualize_contracts(
        service: ::actix_web::web::Data<dyn SolidityVisualizer + Sync + Send + 'static>,
        json: ::actix_web::web::Json<VisualizeContractsJson>,
    ) -> Result<::actix_web::web::Json<VisualizeResponse>, ::actix_web::Error> {
        let json = json.into_inner();
        let request = VisualizeContractsRequest {
            sources: json.sources,
            output_mask: json.output_mask,
        };
        Ok(
            ::actix_web::web::Json(
                service
                    .visualize_contracts(request.into_request())
                    .await
                    .map_err(::actix_prost::map_tonic_error)?
                    .into_inner(),
            ),
        )
    }
    async fn call_visualize_storage(
        service: ::actix_web::web::Data<dyn SolidityVisualizer + Sync + Send + 'static>,
        json: ::actix_web::web::Json<VisualizeStorageJson>,
    ) -> Result<::actix_web::web::Json<VisualizeResponse>, ::actix_web::Error> {
        let json = json.into_inner();
        let request = VisualizeStorageRequest {
            sources: json.sources,
            file_name: json.file_name,
            contract_name: json.contract_name,
            output_mask: json.output_mask,
        };
        Ok(
            ::actix_web::web::Json(
                service
                    .visualize_storage(request.into_request())
                    .await
                    .map_err(::actix_prost::map_tonic_error)?
                    .into_inner(),
            ),
        )
    }
    pub fn route_solidity_visualizer(
        config: &mut ::actix_web::web::ServiceConfig,
        service: Arc<dyn SolidityVisualizer + Send + Sync + 'static>,
    ) {
        config.app_data(::actix_web::web::Data::from(service));
        config
            .route(
                "/v1/solidity:visualizeContracts",
                ::actix_web::web::post().to(call_visualize_contracts),
            );
        config
            .route(
                "/v1/solidity:VisualizeStorage",
                ::actix_web::web::post().to(call_visualize_storage),
            );
    }
}
/// Generated client implementations.
pub mod solidity_visualizer_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SolidityVisualizerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SolidityVisualizerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SolidityVisualizerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SolidityVisualizerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SolidityVisualizerClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn visualize_contracts(
            &mut self,
            request: impl tonic::IntoRequest<super::VisualizeContractsRequest>,
        ) -> Result<tonic::Response<super::VisualizeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/blockscout.visualizer.v1.SolidityVisualizer/VisualizeContracts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn visualize_storage(
            &mut self,
            request: impl tonic::IntoRequest<super::VisualizeStorageRequest>,
        ) -> Result<tonic::Response<super::VisualizeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/blockscout.visualizer.v1.SolidityVisualizer/VisualizeStorage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod solidity_visualizer_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with SolidityVisualizerServer.
    #[async_trait]
    pub trait SolidityVisualizer: Send + Sync + 'static {
        async fn visualize_contracts(
            &self,
            request: tonic::Request<super::VisualizeContractsRequest>,
        ) -> Result<tonic::Response<super::VisualizeResponse>, tonic::Status>;
        async fn visualize_storage(
            &self,
            request: tonic::Request<super::VisualizeStorageRequest>,
        ) -> Result<tonic::Response<super::VisualizeResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SolidityVisualizerServer<T: SolidityVisualizer> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SolidityVisualizer> SolidityVisualizerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SolidityVisualizerServer<T>
    where
        T: SolidityVisualizer,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/blockscout.visualizer.v1.SolidityVisualizer/VisualizeContracts" => {
                    #[allow(non_camel_case_types)]
                    struct VisualizeContractsSvc<T: SolidityVisualizer>(pub Arc<T>);
                    impl<
                        T: SolidityVisualizer,
                    > tonic::server::UnaryService<super::VisualizeContractsRequest>
                    for VisualizeContractsSvc<T> {
                        type Response = super::VisualizeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VisualizeContractsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).visualize_contracts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VisualizeContractsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/blockscout.visualizer.v1.SolidityVisualizer/VisualizeStorage" => {
                    #[allow(non_camel_case_types)]
                    struct VisualizeStorageSvc<T: SolidityVisualizer>(pub Arc<T>);
                    impl<
                        T: SolidityVisualizer,
                    > tonic::server::UnaryService<super::VisualizeStorageRequest>
                    for VisualizeStorageSvc<T> {
                        type Response = super::VisualizeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VisualizeStorageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).visualize_storage(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VisualizeStorageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: SolidityVisualizer> Clone for SolidityVisualizerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SolidityVisualizer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SolidityVisualizer> tonic::server::NamedService
    for SolidityVisualizerServer<T> {
        const NAME: &'static str = "blockscout.visualizer.v1.SolidityVisualizer";
    }
}
