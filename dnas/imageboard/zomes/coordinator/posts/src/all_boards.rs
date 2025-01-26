use hdk::prelude::*;
use posts_integrity::*;

#[hdk_extern]
pub fn get_all_boards() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_boards");
    get_links(GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllBoards)?.build())
}
