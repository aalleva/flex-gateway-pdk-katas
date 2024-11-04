// Copyright 2023 Salesforce, Inc. All rights reserved.

mod common;

use pdk_test::{pdk_test, TestComposite};
use pdk_test::port::Port;
use pdk_test::services::httpmock::{HttpMockConfig, HttpMock};

// Flex port for the internal test network
const _FLEX_PORT: Port = 8081;

// This integration test shows how to build a test to compose a local-flex instance
// with a MockServer backend
#[pdk_test]
async fn pdk_basic_logging_filter() -> anyhow::Result<()> {

    // Configure an HttpMock service
    let backend_config = HttpMockConfig::builder()
        .hostname("backend")
        .port(80)
        .build();

    // Register HTTPMock service and start the docker network
    let composite = TestComposite::builder()
        .with_service(backend_config)
        .build()
        .await?;

    // Get the httpmock handle
    let httpmock: HttpMock = composite.service()?;

    // Connect the mock server
    let mock_server = httpmock::MockServer::connect_async(httpmock.socket()).await;

    // Configure the endpoint mocks
    mock_server.mock(|when, then| {
        when.path_contains("/ping");
        then.status(200).body("pong");
    });

    let base_url = mock_server.base_url();

    // Hit the endpoint
    let response = reqwest::get(format!("{base_url}/ping")).await?;

    // Assert the response
    assert_eq!(response.status(), 200);
    assert_eq!(response.text().await?, "pong");

    Ok(())
}
