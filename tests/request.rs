use actix_web::{
    test::{self, read_body, read_body_json, TestRequest},
    web,
    App
};
use std::{
    collections::BTreeMap,
    fs,
    str::from_utf8,
    path::PathBuf,
};

use sol_to_uml::sol_to_uml_handler;
use sol_to_uml::types::{SolToUmlRequest, SolToUmlResponse};

const CONTRACTS_DIR: &'static str = "tests/contracts";
const UML_DIR: &'static str = "tests/uml";
const ROUTE: &'static str = "/sol2uml";

mod success_tests {
    use super::*;

    #[actix_web::test]
    async fn contract_with_lib() {
        let app = test::init_service(App::new().service(web::resource("/sol2uml").route(web::post().to(sol_to_uml_handler)))).await;

        let contract_path = format!("{}/contract_with_lib.sol", CONTRACTS_DIR);
        let uml_path = format!("{}/contract_with_lib.svg", UML_DIR);
        let contract = fs::read_to_string(&contract_path).expect("Error while reading contract_with_lib.sol");
        let uml = fs::read_to_string(&uml_path).expect("Error while reading contract_with_lib.svg");

        let request = SolToUmlRequest {
            sources: BTreeMap::from([(PathBuf::from("contract_with_lib.sol"), contract)])
        };
        let response = TestRequest::post()
            .uri(ROUTE)
            .set_json(&request)
            .send_request(&app)
            .await;

        if !response.status().is_success() {
            let status = response.status();
            let body = read_body(response).await;
            let message = from_utf8(&body).expect("Read body as UTF-8");
            panic!(
                "Invalid status code (success expected). Status: {}. Messsage: {}",
                status, message
            )
        }

        let response: SolToUmlResponse = read_body_json(response).await;
        assert_eq!(uml, response.uml_diagram);
    }
}

mod failure_tests {
    use super::*;

    #[actix_web::test]
    async fn wrong_path() {
        let app = test::init_service(App::new().service(web::resource("/sol2uml").route(web::post().to(sol_to_uml_handler)))).await;

        let contract_path = format!("{}/contract_with_lib.sol", CONTRACTS_DIR);
        let contract = fs::read_to_string(&contract_path).expect("Error while reading contract_with_lib.sol");

        let request = SolToUmlRequest {
            sources: BTreeMap::from([(PathBuf::from("/usr/contract_with_lib.sol"), contract)])
        };
        let response = TestRequest::post()
            .uri(ROUTE)
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
}
