use hdk::prelude::*;
use posts_integrity::*;

#[hdk_extern]
pub fn get_all_threads() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_threads");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllThreads)?.build(),
    )
}
