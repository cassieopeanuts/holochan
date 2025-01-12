use hdk::prelude::*;
use posts_integrity::*;

#[hdk_extern]
pub fn get_posts_by_thread(author: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(author, LinkTypes::PostsByThread)?.build())
}
