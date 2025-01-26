use hdk::prelude::*;
use posts_integrity::*;

#[hdk_extern]
pub fn create_board(board: Board) -> ExternResult<Record> {
    let board_hash = create_entry(&EntryTypes::Board(board.clone()))?;
    create_link(
        board.creator.clone(),
        board_hash.clone(),
        LinkTypes::CreatorToBoards,
        (),
    )?;
    let record = get(board_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created Board".to_string())
    ))?;
    let path = Path::from("all_boards");
    create_link(
        path.path_entry_hash()?,
        board_hash.clone(),
        LinkTypes::AllBoards,
        (),
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_latest_board(original_board_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(original_board_hash.clone(), LinkTypes::BoardUpdates)?
            .build(),
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_board_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(wasm_error!(WasmErrorInner::Guest(
                    "No action hash associated with link".to_string()
                )))?
        }
        None => original_board_hash.clone(),
    };
    get(latest_board_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_original_board(original_board_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_board_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed get details response".to_string()
        ))),
    }
}

#[hdk_extern]
pub fn get_all_revisions_for_board(original_board_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let Some(original_record) = get_original_board(original_board_hash.clone())? else {
        return Ok(vec![]);
    };
    let links = get_links(
        GetLinksInputBuilder::try_new(original_board_hash.clone(), LinkTypes::BoardUpdates)?
            .build(),
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| {
            Ok(GetInput::new(
                link.target
                    .into_action_hash()
                    .ok_or(wasm_error!(WasmErrorInner::Guest(
                        "No action hash associated with link".to_string()
                    )))?
                    .into(),
                GetOptions::default(),
            ))
        })
        .collect::<ExternResult<Vec<GetInput>>>()?;
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let mut records: Vec<Record> = records.into_iter().flatten().collect();
    records.insert(0, original_record);
    Ok(records)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateBoardInput {
    pub original_board_hash: ActionHash,
    pub previous_board_hash: ActionHash,
    pub updated_board: Board,
}

#[hdk_extern]
pub fn update_board(input: UpdateBoardInput) -> ExternResult<Record> {
    let updated_board_hash = update_entry(input.previous_board_hash.clone(), &input.updated_board)?;
    create_link(
        input.original_board_hash.clone(),
        updated_board_hash.clone(),
        LinkTypes::BoardUpdates,
        (),
    )?;
    let record = get(updated_board_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly updated Board".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_board(original_board_hash: ActionHash) -> ExternResult<ActionHash> {
    let details = get_details(original_board_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("Board not found".to_string())),
    )?;
    let record = match details {
        Details::Record(details) => Ok(details.record),
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed get details response".to_string()
        ))),
    }?;
    let entry = record
        .entry()
        .as_option()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Board record has no entry".to_string()
        )))?;
    let board = <Board>::try_from(entry)?;
    let links = get_links(
        GetLinksInputBuilder::try_new(board.creator.clone(), LinkTypes::CreatorToBoards)?.build(),
    )?;
    for link in links {
        if let Some(action_hash) = link.target.into_action_hash() {
            if action_hash == original_board_hash {
                delete_link(link.create_link_hash)?;
            }
        }
    }
    let path = Path::from("all_boards");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllBoards)?.build(),
    )?;
    for link in links {
        if let Some(hash) = link.target.into_action_hash() {
            if hash == original_board_hash {
                delete_link(link.create_link_hash)?;
            }
        }
    }
    delete_entry(original_board_hash)
}

#[hdk_extern]
pub fn get_all_deletes_for_board(
    original_board_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_board_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed details".into()
        ))),
        Details::Record(record_details) => Ok(Some(record_details.deletes)),
    }
}

#[hdk_extern]
pub fn get_oldest_delete_for_board(
    original_board_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_board(original_board_hash)? else {
        return Ok(None);
    };
    deletes.sort_by(|delete_a, delete_b| {
        delete_a
            .action()
            .timestamp()
            .cmp(&delete_b.action().timestamp())
    });
    Ok(deletes.first().cloned())
}

#[hdk_extern]
pub fn get_boards_for_creator(creator: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(creator, LinkTypes::CreatorToBoards)?.build())
}

#[hdk_extern]
pub fn get_deleted_boards_for_creator(
    creator: AgentPubKey,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        creator,
        LinkTypes::CreatorToBoards,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}
