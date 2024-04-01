use serde_json::json;
use ShardNFTs::NFTLeaf;

#[tokio::test]
async fn test_contract_initialization() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;
    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    let authorized_account = sandbox.root_account().unwrap();

    let outcome = authorized_account
        .call(contract.id(), "new")
        .args_json(json!({ "authorized_account": authorized_account.id() }))
        .transact()
        .await?;

    assert!(outcome.is_success());

    let authorized_account_view = contract
        .view("authorized_account")
        .args_json(json!({}))
        .await?;
    assert_eq!(
        authorized_account_view.json::<String>()?,
        authorized_account.id().to_string()
    );

    Ok(())
}

#[tokio::test]
async fn test_update_merkle_tree() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;
    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    let authorized_account = sandbox.root_account().unwrap();

    let outcome = authorized_account
        .call(contract.id(), "new")
        .args_json(json!({ "authorized_account": authorized_account.id() }))
        .transact()
        .await?;

    assert!(outcome.is_success());

    let leaf = json!({
        "nft_id": "1",
        "owner": authorized_account.id(),
        "metadata": "some metadata"
    });

    let proof = vec![vec![1, 2, 3], vec![4, 5, 6]];

    let outcome = authorized_account
        .call(contract.id(), "update_merkle_tree")
        .args_json(json!({ "leaf": leaf, "proof": proof }))
        .transact()
        .await?;

    assert!(outcome.is_success());

    let get_leaf_outcome = contract
        .view("get_leaf")
        .args_json(json!({ "nft_id": "1" }))
        .await?;

    println!("get_leaf_outcome: {:?}", get_leaf_outcome);

    assert!(get_leaf_outcome.json::<Option<NFTLeaf>>()?.is_some());

    Ok(())
}
