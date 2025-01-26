use hdk::prelude::*;
use posts_integrity::*;

#[hdk_extern]
pub fn create_thread(mut thread: Thread) -> ExternResult<Record> {
    // 1) Set timestamp
    thread.timestamp = sys_time()?;

    // 2) Create the Thread entry
    let thread_hash = create_entry(&EntryTypes::Thread(thread.clone()))?;

    // 3) Link the thread to its author
    create_link(
        thread.author.clone(),
        thread_hash.clone(),
        LinkTypes::CreatorToThreads,
        (),
    )?;

    // 4) If the thread has an image, link
    if let Some(base) = thread.image_hash.clone() {
        create_link(base, thread_hash.clone(), LinkTypes::ThreadToThreads, ())?;
    }

    // 5) Link the thread to a board 
    if let Some(board_hash) = thread.board_hash.clone() {
        // board -> thread
        create_link(board_hash.clone(), thread_hash.clone(), LinkTypes::BoardToThreads, ())?;
        // thread -> board
        create_link(thread_hash.clone(), board_hash.clone(), LinkTypes::ThreadToBoards, ())?;
    }

    // 6) Link to path "all_threads"
    let path = Path::from("all_threads");
    create_link(
        path.path_entry_hash()?,
        thread_hash.clone(),
        LinkTypes::AllThreads,
        (),
    )?;

    // 7) Return the newly created record
    let record = get(thread_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created Thread".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn get_latest_thread(original_thread_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(original_thread_hash.clone(), LinkTypes::ThreadUpdates)?
            .build(),
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_thread_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(wasm_error!(WasmErrorInner::Guest(
                    "No action hash associated with link".to_string()
                )))?
        }
        None => original_thread_hash.clone(),
    };
    get(latest_thread_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_original_thread(original_thread_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_thread_hash, GetOptions::default())? else {
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
pub fn get_all_revisions_for_thread(original_thread_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let Some(original_record) = get_original_thread(original_thread_hash.clone())? else {
        return Ok(vec![]);
    };
    let links = get_links(
        GetLinksInputBuilder::try_new(original_thread_hash.clone(), LinkTypes::ThreadUpdates)?
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
pub struct UpdateThreadInput {
    pub original_thread_hash: ActionHash,
    pub previous_thread_hash: ActionHash,
    pub updated_thread: Thread,
}

#[hdk_extern]
pub fn update_thread(input: UpdateThreadInput) -> ExternResult<Record> {
    let updated_thread_hash =
        update_entry(input.previous_thread_hash.clone(), &input.updated_thread)?;
    create_link(
        input.original_thread_hash.clone(),
        updated_thread_hash.clone(),
        LinkTypes::ThreadUpdates,
        (),
    )?;
    let record = get(updated_thread_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly updated Thread".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_thread(original_thread_hash: ActionHash) -> ExternResult<ActionHash> {
    let details = get_details(original_thread_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("Thread not found".to_string())),
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
            "Thread record has no entry".to_string()
        )))?;
    let thread = <Thread>::try_from(entry)?;
    let links = get_links(
        GetLinksInputBuilder::try_new(thread.author.clone(), LinkTypes::CreatorToThreads)?.build(),
    )?;
    for link in links {
        if let Some(action_hash) = link.target.into_action_hash() {
            if action_hash == original_thread_hash {
                delete_link(link.create_link_hash)?;
            }
        }
    }
    if let Some(base_address) = thread.image_hash.clone() {
        let links = get_links(
            GetLinksInputBuilder::try_new(base_address, LinkTypes::ThreadToThreads)?.build(),
        )?;
        for link in links {
            if let Some(action_hash) = link.target.into_action_hash() {
                if action_hash == original_thread_hash {
                    delete_link(link.create_link_hash)?;
                }
            }
        }
    }
    let path = Path::from("all_threads");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllThreads)?.build(),
    )?;
    for link in links {
        if let Some(hash) = link.target.into_action_hash() {
            if hash == original_thread_hash {
                delete_link(link.create_link_hash)?;
            }
        }
    }
    delete_entry(original_thread_hash)
}

#[hdk_extern]
pub fn get_all_deletes_for_thread(
    original_thread_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_thread_hash, GetOptions::default())? else {
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
pub fn get_oldest_delete_for_thread(
    original_thread_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_thread(original_thread_hash)? else {
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
pub fn get_threads_for_creator(creator: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(creator, LinkTypes::CreatorToThreads)?.build())
}

#[hdk_extern]
pub fn get_deleted_threads_for_creator(
    creator: AgentPubKey,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        creator,
        LinkTypes::CreatorToThreads,
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
pub fn get_threads_for_thread(thread_hash: EntryHash) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(thread_hash, LinkTypes::ThreadToThreads)?.build())
}

#[hdk_extern]
pub fn get_deleted_threads_for_thread(
    thread_hash: EntryHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        thread_hash,
        LinkTypes::ThreadToThreads,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}
