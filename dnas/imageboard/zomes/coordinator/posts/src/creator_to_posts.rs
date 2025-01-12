use hdk::prelude::*;
use posts_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddPostForCreatorInput {
    pub base_creator: AgentPubKey,
    pub target_post_hash: ActionHash,
}

#[hdk_extern]
pub fn add_post_for_creator(input: AddPostForCreatorInput) -> ExternResult<()> {
    create_link(
        input.base_creator.clone(),
        input.target_post_hash.clone(),
        LinkTypes::CreatorToPosts,
        (),
    )?;
    create_link(
        input.target_post_hash,
        input.base_creator,
        LinkTypes::PostToCreators,
        (),
    )?;
    Ok(())
}

#[hdk_extern]
pub fn get_posts_for_creator(creator: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(creator, LinkTypes::CreatorToPosts)?.build())
}

#[hdk_extern]
pub fn get_deleted_posts_for_creator(
    creator: AgentPubKey,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        creator,
        LinkTypes::CreatorToPosts,
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
pub fn get_creators_for_post(post_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(post_hash, LinkTypes::PostToCreators)?.build())
}

#[hdk_extern]
pub fn get_deleted_creators_for_post(
    post_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        post_hash,
        LinkTypes::PostToCreators,
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
pub struct RemovePostForCreatorInput {
    pub base_creator: AgentPubKey,
    pub target_post_hash: ActionHash,
}

#[hdk_extern]
pub fn delete_post_for_creator(input: RemovePostForCreatorInput) -> ExternResult<()> {
    let links = get_links(
        GetLinksInputBuilder::try_new(input.base_creator.clone(), LinkTypes::CreatorToPosts)?
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
        GetLinksInputBuilder::try_new(input.target_post_hash.clone(), LinkTypes::PostToCreators)?
            .build(),
    )?;
    for link in links {
        if AgentPubKey::from(link.target.clone().into_entry_hash().ok_or(wasm_error!(
            WasmErrorInner::Guest("No entry_hash associated with link".to_string())
        ))?) == input.base_creator.clone().into_hash().into()
        {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
