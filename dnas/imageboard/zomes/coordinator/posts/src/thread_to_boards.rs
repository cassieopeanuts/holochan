use hdk::prelude::*;
use posts_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddBoardForThreadInput {
    pub base_thread_hash: ActionHash,
    pub target_board_hash: ActionHash,
}

#[hdk_extern]
pub fn add_board_for_thread(input: AddBoardForThreadInput) -> ExternResult<()> {
    create_link(
        input.base_thread_hash.clone(),
        input.target_board_hash.clone(),
        LinkTypes::ThreadToBoards,
        (),
    )?;
    create_link(
        input.target_board_hash,
        input.base_thread_hash,
        LinkTypes::BoardToThreads,
        (),
    )?;
    Ok(())
}

#[hdk_extern]
pub fn get_boards_for_thread(thread_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(thread_hash, LinkTypes::ThreadToBoards)?.build())
}

#[hdk_extern]
pub fn get_deleted_boards_for_thread(
    thread_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        thread_hash,
        LinkTypes::ThreadToBoards,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_threads_for_board(board_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(board_hash, LinkTypes::BoardToThreads)?.build())
}

#[hdk_extern]
pub fn get_deleted_threads_for_board(
    board_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        board_hash,
        LinkTypes::BoardToThreads,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveBoardForThreadInput {
    pub base_thread_hash: ActionHash,
    pub target_board_hash: ActionHash,
}

#[hdk_extern]
pub fn delete_board_for_thread(input: RemoveBoardForThreadInput) -> ExternResult<()> {
    let links = get_links(
        GetLinksInputBuilder::try_new(input.base_thread_hash.clone(), LinkTypes::ThreadToBoards)?
            .build(),
    )?;
    for link in links {
        if link
            .target
            .clone()
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?
            == input.target_board_hash.clone().into_hash().into()
        {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        GetLinksInputBuilder::try_new(input.target_board_hash.clone(), LinkTypes::BoardToThreads)?
            .build(),
    )?;
    for link in links {
        if link
            .target
            .clone()
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?
            == input.base_thread_hash.clone().into_hash().into()
        {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
