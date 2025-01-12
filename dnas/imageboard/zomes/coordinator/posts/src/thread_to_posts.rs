use hdk::prelude::*;
use posts_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddPostForThreadInput {
    pub base_thread_hash: ActionHash,
    pub target_post_hash: ActionHash,
}

#[hdk_extern]
pub fn add_post_for_thread(input: AddPostForThreadInput) -> ExternResult<()> {
    create_link(
        input.base_thread_hash.clone(),
        input.target_post_hash.clone(),
        LinkTypes::ThreadToPosts,
        (),
    )?;
    create_link(
        input.target_post_hash,
        input.base_thread_hash,
        LinkTypes::PostToThreads,
        (),
    )?;
    Ok(())
}

#[hdk_extern]
pub fn get_posts_for_thread(thread_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(thread_hash, LinkTypes::ThreadToPosts)?.build())
}

#[hdk_extern]
pub fn get_deleted_posts_for_thread(
    thread_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        thread_hash,
        LinkTypes::ThreadToPosts,
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
pub fn get_threads_for_post(post_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(post_hash, LinkTypes::PostToThreads)?.build())
}

#[hdk_extern]
pub fn get_deleted_threads_for_post(
    post_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        post_hash,
        LinkTypes::PostToThreads,
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
pub struct RemovePostForThreadInput {
    pub base_thread_hash: ActionHash,
    pub target_post_hash: ActionHash,
}

#[hdk_extern]
pub fn delete_post_for_thread(input: RemovePostForThreadInput) -> ExternResult<()> {
    let links = get_links(
        GetLinksInputBuilder::try_new(input.base_thread_hash.clone(), LinkTypes::ThreadToPosts)?
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
            == input.target_post_hash.clone().into_hash().into()
        {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        GetLinksInputBuilder::try_new(input.target_post_hash.clone(), LinkTypes::PostToThreads)?
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
