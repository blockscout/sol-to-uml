use actix_web::{
    test::{self, read_body, read_body_json, TestRequest},
    web::{self, Json},
    App,
};
use assert_str::assert_str_eq;
use std::{borrow::BorrowMut, collections::BTreeMap, fs, path::PathBuf, str::from_utf8};
use walkdir::WalkDir;

use sol_to_uml::{
    handlers::{sol_to_storage_handler, sol_to_uml_handler},
    types::{SolToStorageRequest, SolToStorageResponse, SolToUmlRequest, SolToUmlResponse},
};

const CONTRACTS_DIR: &'static str = "tests/contracts";
const SAMPLES_DIR: &'static str = "tests/samples";
const ROUTE: &'static str = "/solidity";

fn fill_sources_map(sources: &mut BTreeMap<PathBuf, String>, project_path: &PathBuf) {
    if project_path.is_dir() {
        for entry in WalkDir::new(project_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let relative_path = entry
                .path()
                .strip_prefix(project_path)
                .expect("Failed to strip prefix")
                .to_path_buf();
            if entry.path().is_file() {
                let content = fs::read_to_string(entry.path()).unwrap();
                sources.insert(relative_path, content);
            }
        }
    } else {
        let content = fs::read_to_string(project_path).unwrap();
        sources.insert(project_path.clone(), content);
    }
}

mod success_simple_tests {
    use super::*;

    #[actix_web::test]
    async fn uml_simple_contract() {
        let route = format!("{}/uml", ROUTE);
        let app = test::init_service(
            App::new().service(web::resource(&route).route(web::post().to(sol_to_uml_handler))),
        )
        .await;

        let contract_path = format!("{}/SimpleContract.sol", CONTRACTS_DIR);
        let uml_path = format!("{}/uml/simple_contract.svg", SAMPLES_DIR);
        let contract =
            fs::read_to_string(&contract_path).expect("Error while reading SimpleContract.sol");
        let uml = fs::read_to_string(&uml_path).expect("Error while reading simple_contract.svg");

        let request = SolToUmlRequest {
            sources: BTreeMap::from([(PathBuf::from("SimpleContract.sol"), contract)]),
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
    async fn storage_simple_contract() {
        let route = format!("{}/storage", ROUTE);
        let app = test::init_service(
            App::new().service(web::resource(&route).route(web::post().to(sol_to_storage_handler))),
        )
        .await;

        let contract_path = format!("{}/SimpleContract.sol", CONTRACTS_DIR);
        let storage_path = format!("{}/storage/simple_contract.svg", SAMPLES_DIR);
        let contract =
            fs::read_to_string(&contract_path).expect("Error while reading SimpleContract.sol");
        let storage =
            fs::read_to_string(&storage_path).expect("Error while reading simple_contract.svg");

        let request = SolToStorageRequest {
            sources: BTreeMap::from([(PathBuf::from("SimpleContract.sol"), contract)]),
            main_contract: String::from("SimpleStorage"),
            main_contract_filename: PathBuf::from("SimpleContract.sol"),
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
    async fn storage_simple_contract_alt_path() {
        let route = format!("{}/storage", ROUTE);
        let app = test::init_service(
            App::new().service(web::resource(&route).route(web::post().to(sol_to_storage_handler))),
        )
        .await;

        let contract_path = format!("{}/SimpleContract.sol", CONTRACTS_DIR);
        let storage_path = format!("{}/storage/simple_contract.svg", SAMPLES_DIR);
        let contract =
            fs::read_to_string(&contract_path).expect("Error while reading SimpleContract.sol");
        let storage =
            fs::read_to_string(&storage_path).expect("Error while reading simple_contract.svg");

        let request = SolToStorageRequest {
            sources: BTreeMap::from([(PathBuf::from("c/d/SimpleContract.sol"), contract)]),
            main_contract: String::from("SimpleStorage"),
            main_contract_filename: PathBuf::from("c/d/SimpleContract.sol"),
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

mod success_advanced_tests {
    use super::*;

    #[actix_web::test]
    async fn uml_large_project() {
        let mut request = SolToUmlRequest {
            sources: BTreeMap::new(),
        };

        let project_path = PathBuf::from(format!("{}/large_project_many_methods", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let uml_path = format!("{}/uml/large_project_many_methods.svg", SAMPLES_DIR);
        let uml = fs::read_to_string(&uml_path)
            .expect("Error while reading large_project_many_methods.svg");

        let result = sol_to_uml_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(uml, res.uml_diagram);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }

    #[actix_web::test]
    async fn storage_large_project() {
        let mut request = SolToStorageRequest {
            sources: BTreeMap::new(),
            main_contract: String::from("MyToken"),
            main_contract_filename: PathBuf::from("Token.sol"),
        };

        let project_path = PathBuf::from(format!("{}/large_project_many_methods", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let storage_path = format!("{}/storage/large_project_many_methods.svg", SAMPLES_DIR);
        let storage = fs::read_to_string(&storage_path)
            .expect("Error while reading large_project_many_methods.svg");

        let result = sol_to_storage_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(storage, res.storage);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }

    #[actix_web::test]
    async fn uml_many_libraries() {
        let mut request = SolToUmlRequest {
            sources: BTreeMap::new(),
        };

        let project_path = PathBuf::from(format!("{}/many_libraries", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let uml_path = format!("{}/uml/many_libraries.svg", SAMPLES_DIR);
        let uml = fs::read_to_string(&uml_path).expect("Error while reading many_libraries.svg");

        let result = sol_to_uml_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(uml, res.uml_diagram);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }

    #[actix_web::test]
    async fn uml_same_contract_names() {
        let mut request = SolToUmlRequest {
            sources: BTreeMap::new(),
        };

        let project_path = PathBuf::from(format!("{}/same_contract_names", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let uml_path = format!("{}/uml/same_contract_names.svg", SAMPLES_DIR);
        let uml =
            fs::read_to_string(&uml_path).expect("Error while reading same_contract_names.svg");

        let result = sol_to_uml_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(uml, res.uml_diagram);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }

    #[actix_web::test]
    async fn storage_same_contract_names() {
        let mut request = SolToStorageRequest {
            sources: BTreeMap::new(),
            main_contract: String::from("A"),
            main_contract_filename: PathBuf::from("Main.sol"),
        };

        let project_path = PathBuf::from(format!("{}/same_contract_names", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let storage_path = format!("{}/storage/same_contract_names.svg", SAMPLES_DIR);
        let storage =
            fs::read_to_string(&storage_path).expect("Error while reading same_contract_names.svg");

        let result = sol_to_storage_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(storage, res.storage);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }

    #[actix_web::test]
    async fn storage_same_filenames_different_contracts() {
        let mut request = SolToStorageRequest {
            sources: BTreeMap::new(),
            main_contract: String::from("A"),
            main_contract_filename: PathBuf::from("SameName.sol"),
        };

        let project_path = PathBuf::from(format!(
            "{}/same_filenames_different_contracts",
            CONTRACTS_DIR
        ));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let storage_path = format!(
            "{}/storage/same_filenames_different_contracts.svg",
            SAMPLES_DIR
        );
        let storage = fs::read_to_string(&storage_path)
            .expect("Error while reading same_filenames_different_contracts.svg");

        let result = sol_to_storage_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(storage, res.storage);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }
}

mod success_known_issues {
    use super::*;

    #[actix_web::test]
    async fn uml_contract_with_compile_error() {
        // sol2uml ignores not syntax compile errors
        let mut request = SolToUmlRequest {
            sources: BTreeMap::new(),
        };

        let project_path = PathBuf::from(format!("{}/ContractCompileError.sol", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let uml_path = format!("{}/uml/contract_compile_error.svg", SAMPLES_DIR);
        let uml =
            fs::read_to_string(&uml_path).expect("Error while reading contract_compile_error.svg");

        let result = sol_to_uml_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(uml, res.uml_diagram);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }

    #[actix_web::test]
    async fn storage_contract_with_compile_error() {
        // sol2uml ignores not syntax compile errors also in storage mod
        let mut request = SolToStorageRequest {
            sources: BTreeMap::new(),
            main_contract: String::from("Main"),
            main_contract_filename: PathBuf::from("ContractCompileError.sol"),
        };

        let project_path = PathBuf::from(format!("{}/ContractCompileError.sol", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let storage_path = format!("{}/storage/contract_compile_error.svg", SAMPLES_DIR);
        let storage = fs::read_to_string(&storage_path)
            .expect("Error while reading contract_compile_error.svg");

        let result = sol_to_storage_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(storage, res.storage);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }

    #[actix_web::test]
    async fn uml_import_missing_contract() {
        // sol2uml just doesn`t show missing contract on uml diagram
        let mut request = SolToUmlRequest {
            sources: BTreeMap::new(),
        };

        let project_path = PathBuf::from(format!("{}/ImportMissingContract.sol", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let uml_path = format!("{}/uml/import_missing_contract.svg", SAMPLES_DIR);
        let uml =
            fs::read_to_string(&uml_path).expect("Error while reading import_missing_contract.svg");

        let result = sol_to_uml_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(uml, res.uml_diagram);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }

    #[actix_web::test]
    async fn storage_import_missing_contract() {
        // sol2uml ignores missing contract if it doesn`t affect storage
        let mut request = SolToStorageRequest {
            sources: BTreeMap::new(),
            main_contract: String::from("Main"),
            main_contract_filename: PathBuf::from("ImportMissingContract.sol"),
        };

        let project_path = PathBuf::from(format!("{}/ImportMissingContract.sol", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let storage_path = format!("{}/storage/import_missing_contract.svg", SAMPLES_DIR);
        let storage = fs::read_to_string(&storage_path)
            .expect("Error while reading import_missing_contract.svg");

        let result = sol_to_storage_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(storage, res.storage);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }

    #[actix_web::test]
    async fn uml_import_missing_inherited_contract() {
        // sol2uml just doesn`t show missing contract on uml, even if some of
        // existing contracts is inherited from it
        let mut request = SolToUmlRequest {
            sources: BTreeMap::new(),
        };

        let project_path = PathBuf::from(format!(
            "{}/ImportMissingInheritedContract.sol",
            CONTRACTS_DIR
        ));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let uml_path = format!("{}/uml/import_missing_inherited_contract.svg", SAMPLES_DIR);
        let uml = fs::read_to_string(&uml_path)
            .expect("Error while reading import_missing_inherited_contract.svg");

        let result = sol_to_uml_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(uml, res.uml_diagram);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }

    #[actix_web::test]
    async fn uml_import_missing_library() {
        // sol2uml just doesn`t show missing library on uml
        let mut request = SolToUmlRequest {
            sources: BTreeMap::new(),
        };

        let project_path = PathBuf::from(format!("{}/ImportMissingLibrary.sol", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let uml_path = format!("{}/uml/import_missing_library.svg", SAMPLES_DIR);
        let uml =
            fs::read_to_string(&uml_path).expect("Error while reading import_missing_library.svg");

        let result = sol_to_uml_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(uml, res.uml_diagram);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }

    #[actix_web::test]
    async fn uml_long_names() {
        let mut request = SolToUmlRequest {
            sources: BTreeMap::new(),
        };

        let project_path = PathBuf::from(format!("{}/LongNames.sol", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let uml_path = format!("{}/uml/long_names.svg", SAMPLES_DIR);
        let uml = fs::read_to_string(&uml_path).expect("Error while reading long_names.svg");

        let result = sol_to_uml_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(uml, res.uml_diagram);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }

    #[actix_web::test]
    async fn storage_long_names() {
        let mut request = SolToStorageRequest {
            sources: BTreeMap::new(),
            main_contract: String::from("Main"),
            main_contract_filename: PathBuf::from("LongNames.sol"),
        };

        let project_path = PathBuf::from(format!("{}/LongNames.sol", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let storage_path = format!("{}/storage/long_names.svg", SAMPLES_DIR);
        let storage =
            fs::read_to_string(&storage_path).expect("Error while reading long_names.svg");

        let result = sol_to_storage_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(storage, res.storage);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }

    #[actix_web::test]
    async fn storage_same_filenames() {
        // when contracts with the same name have the same filename, then
        // storage will be returned for the contract with the smallest filename in sort order
        let mut request = SolToStorageRequest {
            sources: BTreeMap::new(),
            main_contract: String::from("A"),
            main_contract_filename: PathBuf::from("main_dir/SameName.sol"),
        };

        let project_path = PathBuf::from(format!("{}/same_filenames", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let storage_path = format!("{}/storage/same_filenames.svg", SAMPLES_DIR);
        let storage =
            fs::read_to_string(&storage_path).expect("Error while reading same_filenames.svg");

        let result = sol_to_storage_handler(Json(request)).await;
        match result {
            Ok(res) => {
                assert_str_eq!(storage, res.storage);
            }
            Err(err) => {
                panic!("Invalid response. Error: {}", err)
            }
        };
    }
}

mod failure_tests {
    use super::*;

    #[actix_web::test]
    async fn uml_wrong_path() {
        // also will fail for storage
        let route = format!("{}/uml", ROUTE);
        let app = test::init_service(
            App::new().service(web::resource(&route).route(web::post().to(sol_to_uml_handler))),
        )
        .await;

        let contract_path = format!("{}/SimpleContract.sol", CONTRACTS_DIR);
        let contract =
            fs::read_to_string(&contract_path).expect("Error while reading SimpleContract.sol");

        let request = SolToUmlRequest {
            sources: BTreeMap::from([(PathBuf::from("/usr/SimpleContract.sol"), contract)]),
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

        let contract_path = format!("{}/SimpleContract.sol", CONTRACTS_DIR);
        let contract =
            fs::read_to_string(&contract_path).expect("Error while reading SimpleContract.sol");

        let request = SolToStorageRequest {
            sources: BTreeMap::from([(PathBuf::from("./contracts/SimpleContract.sol"), contract)]),
            main_contract: String::from("dsd"),
            main_contract_filename: PathBuf::from("SimpleContract.sol"),
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

    #[actix_web::test]
    async fn uml_library_with_syntax_error() {
        // also will fail for storage
        let mut request = SolToUmlRequest {
            sources: BTreeMap::new(),
        };

        let project_path = PathBuf::from(format!("{}/library_syntax_error", CONTRACTS_DIR));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let result = sol_to_uml_handler(Json(request)).await;
        match result {
            Ok(res) => {
                panic!(
                    "Invalid response, error expected. Response: {}",
                    res.uml_diagram
                );
            }
            Err(err) => {
                if !err.to_string().contains("Failed to parse solidity code") {
                    panic!("Invalid response, wrong error type. {}", err);
                }
            }
        };
    }

    #[actix_web::test]
    async fn storage_import_missing_inherited_contract() {
        // sol2uml returns error if main contract is inherited from missing contract
        // cause it affects main contract storage
        let mut request = SolToStorageRequest {
            sources: BTreeMap::new(),
            main_contract: String::from("Main"),
            main_contract_filename: PathBuf::from("ImportMissingInheritedContract.sol"),
        };

        let project_path = PathBuf::from(format!(
            "{}/ImportMissingInheritedContract.sol",
            CONTRACTS_DIR
        ));
        fill_sources_map(request.sources.borrow_mut(), &project_path);

        let result = sol_to_storage_handler(Json(request)).await;
        match result {
            Ok(res) => {
                panic!(
                    "Invalid response, error expected. Response: {}",
                    res.storage
                );
            }
            Err(err) => {
                if !err
                    .to_string()
                    .contains("Failed to find inherited contract")
                {
                    panic!("Invalid response, wrong error type. {}", err);
                }
            }
        };
    }
}
