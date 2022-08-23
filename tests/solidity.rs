use actix_web::{
    test::{self, read_body, read_body_json, TestRequest},
    web, App,
};
use assert_str::assert_str_eq;
use std::{collections::BTreeMap, fs, path::PathBuf, str::from_utf8};

use sol_to_uml::{
    handlers::{sol_to_storage_handler, sol_to_uml_handler},
    types::{SolToStorageRequest, SolToStorageResponse, SolToUmlRequest, SolToUmlResponse},
};

const CONTRACTS_DIR: &'static str = "tests/contracts";
const SAMPLES_DIR: &'static str = "tests/samples";
const ROUTE: &'static str = "/solidity";

mod success_tests {
    use super::*;

    #[actix_web::test]
    async fn contract_with_lib_uml() {
        let route = format!("{}/uml", ROUTE);
        let app = test::init_service(
            App::new().service(web::resource(&route).route(web::post().to(sol_to_uml_handler))),
        )
        .await;

        let contract_path = format!("{}/contract_with_lib.sol", CONTRACTS_DIR);
        let uml_path = format!("{}/uml/contract_with_lib.svg", SAMPLES_DIR);
        let contract =
            fs::read_to_string(&contract_path).expect("Error while reading contract_with_lib.sol");
        let uml = fs::read_to_string(&uml_path).expect("Error while reading contract_with_lib.svg");

        let request = SolToUmlRequest {
            sources: BTreeMap::from([(PathBuf::from("contract_with_lib.sol"), contract)]),
        };
        let response = TestRequest::post()
            .uri(&route)
            .set_json(&request)
            .send_request(&app)
            .await;

        if !response.status().is_success() {
            let status = response.status();
            let body = read_body(response).await;
            let message = from_utf8(&body).expect("Read body as UTF-8");
            panic!(
                "Invalid status code (success expected). Status: {}. Message: {}",
                status, message
            )
        }

        let response: SolToUmlResponse = read_body_json(response).await;
        assert_str_eq!(uml, response.uml_diagram);
    }

    #[actix_web::test]
    async fn contract_with_lib_storage() {
        let route = format!("{}/storage", ROUTE);
        let app = test::init_service(
            App::new().service(web::resource(&route).route(web::post().to(sol_to_storage_handler))),
        )
        .await;

        let contract_path = format!("{}/contract_with_lib.sol", CONTRACTS_DIR);
        let storage_path = format!("{}/storage/contract_with_lib.svg", SAMPLES_DIR);
        let contract =
            fs::read_to_string(&contract_path).expect("Error while reading contract_with_lib.sol");
        let storage =
            fs::read_to_string(&storage_path).expect("Error while reading contract_with_lib.svg");

        let request = SolToStorageRequest {
            sources: BTreeMap::from([(PathBuf::from("contract_with_lib.sol"), contract)]),
            main_contract: String::from("SimpleStorage"),
            main_contract_filename: PathBuf::from("contract_with_lib.sol"),
        };
        let response = TestRequest::post()
            .uri(&route)
            .set_json(&request)
            .send_request(&app)
            .await;

        if !response.status().is_success() {
            let status = response.status();
            let body = read_body(response).await;
            let message = from_utf8(&body).expect("Read body as UTF-8");
            panic!(
                "Invalid status code (success expected). Status: {}. Message: {}",
                status, message
            )
        }

        let response: SolToStorageResponse = read_body_json(response).await;
        assert_str_eq!(storage, response.storage);
    }

    #[actix_web::test]
    async fn contract_with_lib_storage_alt_path() {
        let route = format!("{}/storage", ROUTE);
        let app = test::init_service(
            App::new().service(web::resource(&route).route(web::post().to(sol_to_storage_handler))),
        )
            .await;

        let contract_path = format!("{}/contract_with_lib.sol", CONTRACTS_DIR);
        let storage_path = format!("{}/storage/contract_with_lib.svg", SAMPLES_DIR);
        let contract =
            fs::read_to_string(&contract_path).expect("Error while reading contract_with_lib.sol");
        let storage =
            fs::read_to_string(&storage_path).expect("Error while reading contract_with_lib.svg");

        let request = SolToStorageRequest {
            sources: BTreeMap::from([(PathBuf::from("c/d/contract_with_lib.sol"), contract)]),
            main_contract: String::from("SimpleStorage"),
            main_contract_filename: PathBuf::from("contract_with_lib.sol"),
        };
        let response = TestRequest::post()
            .uri(&route)
            .set_json(&request)
            .send_request(&app)
            .await;

        if !response.status().is_success() {
            let status = response.status();
            let body = read_body(response).await;
            let message = from_utf8(&body).expect("Read body as UTF-8");
            panic!(
                "Invalid status code (success expected). Status: {}. Message: {}",
                status, message
            )
        }

        let response: SolToStorageResponse = read_body_json(response).await;
        assert_str_eq!(storage, response.storage);
    }
}

mod failure_tests {
    use super::*;

    #[actix_web::test]
    async fn wrong_path() {
        let route = format!("{}/uml", ROUTE);
        let app = test::init_service(
            App::new().service(web::resource(&route).route(web::post().to(sol_to_uml_handler))),
        )
        .await;

        let contract_path = format!("{}/contract_with_lib.sol", CONTRACTS_DIR);
        let contract =
            fs::read_to_string(&contract_path).expect("Error while reading contract_with_lib.sol");

        let request = SolToUmlRequest {
            sources: BTreeMap::from([(PathBuf::from("/usr/contract_with_lib.sol"), contract)]),
        };
        let response = TestRequest::post()
            .uri(&route)
            .set_json(&request)
            .send_request(&app)
            .await;

        assert!(
            response.status().is_client_error(),
            "Invalid status code (failed expected): {}",
            response.status()
        );

        let message = response.response().error().unwrap().to_string();
        assert!(
            message.contains("All paths should be relative"),
            "Invalid response message: {}",
            message
        );
    }

    #[actix_web::test]
    async fn storage_wrong_main_contract() {
        let route = format!("{}/storage", ROUTE);
        let app = test::init_service(
            App::new().service(web::resource(&route).route(web::post().to(sol_to_storage_handler))),
        )
        .await;

        let contract_path = format!("{}/contract_with_lib.sol", CONTRACTS_DIR);
        let contract =
            fs::read_to_string(&contract_path).expect("Error while reading contract_with_lib.sol");

        let request = SolToStorageRequest {
            sources: BTreeMap::from([(
                PathBuf::from("./contracts/contract_with_lib.sol"),
                contract,
            )]),
            main_contract: String::from("dsd"),
            main_contract_filename: PathBuf::from("contract_with_lib.sol"),
        };
        let response = TestRequest::post()
            .uri(&route)
            .set_json(&request)
            .send_request(&app)
            .await;

        assert!(
            response.status().is_client_error(),
            "Invalid status code (failed expected): {}",
            response.status()
        );

        let message = response.response().error().unwrap().to_string();
        assert!(
            message.contains("Failed to find contract with name"),
            "Invalid response message: {}",
            message
        );
    }
}
