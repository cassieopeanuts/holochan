pub mod thread_to_boards;
pub use thread_to_boards::*;
pub mod board;
pub use board::*;
pub mod creator_to_posts;
pub use creator_to_posts::*;
pub mod thread_to_posts;
pub use thread_to_posts::*;
pub mod post;
pub use post::*;
pub mod thread;
use hdi::prelude::*;

pub use thread::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Thread(Thread),
    Post(Post),
    Board(Board),
}

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    CreatorToThreads,
    ThreadToThreads,
    ThreadUpdates,
    AllThreads,
    PostToPosts,
    PostUpdates,
    AllPosts,
    PostsByThread,
    ThreadToPosts,
    PostToThreads,
    CreatorToPosts,
    PostToCreators,
    CreatorToBoards,
    BoardUpdates,
    AllBoards,
    ThreadToBoards,
    BoardToThreads,
}

// Validation you perform during the genesis process. Nobody else on the network performs it, only you.
// There *is no* access to network calls in this callback
#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

// Validation the network performs when you try to join, you can't perform this validation yourself as you are not a member yet.
// There *is* access to network calls in this function
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

// This is the unified validation callback for all entries and link types in this integrity zome
// Below is a match template for all of the variants of `DHT Ops` and entry and link types
// Holochain has already performed the following validation for you:
// - The action signature matches on the hash of its content and is signed by its author
// - The previous action exists, has a lower timestamp than the new action, and incremented sequence number
// - The previous action author is the same as the new action author
// - The timestamp of each action is after the DNA's origin time
// - AgentActivity authorities check that the agent hasn't forked their chain
// - The entry hash in the action matches the entry content
// - The entry type in the action matches the entry content
// - The entry size doesn't exceed the maximum entry size (currently 4MB)
// - Private entry types are not included in the Op content, and public entry types are
// - If the `Op` is an update or a delete, the original action exists and is a `Create` or `Update` action
// - If the `Op` is an update, the original entry exists and is of the same type as the new one
// - If the `Op` is a delete link, the original action exists and is a `CreateLink` action
// - Link tags don't exceed the maximum tag size (currently 1KB)
// - Countersigned entries include an action from each required signer
// You can read more about validation here: https://docs.rs/hdi/latest/hdi/index.html#data-validation
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => match store_entry {
            OpEntry::CreateEntry { app_entry, action } => match app_entry {
                EntryTypes::Thread(thread) => {
                    validate_create_thread(EntryCreationAction::Create(action), thread)
                }
                EntryTypes::Post(post) => {
                    validate_create_post(EntryCreationAction::Create(action), post)
                }
                EntryTypes::Board(board) => {
                    validate_create_board(EntryCreationAction::Create(action), board)
                }
            },
            OpEntry::UpdateEntry {
                app_entry, action, ..
            } => match app_entry {
                EntryTypes::Thread(thread) => {
                    validate_create_thread(EntryCreationAction::Update(action), thread)
                }
                EntryTypes::Post(post) => {
                    validate_create_post(EntryCreationAction::Update(action), post)
                }
                EntryTypes::Board(board) => {
                    validate_create_board(EntryCreationAction::Update(action), board)
                }
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterUpdate(update_entry) => match update_entry {
            OpUpdate::Entry { app_entry, action } => {
                let original_action = must_get_action(action.clone().original_action_address)?
                    .action()
                    .to_owned();
                let original_create_action = match EntryCreationAction::try_from(original_action) {
                    Ok(action) => action,
                    Err(e) => {
                        return Ok(ValidateCallbackResult::Invalid(format!(
                            "Expected to get EntryCreationAction from Action: {e:?}"
                        )));
                    }
                };
                match app_entry {
                    EntryTypes::Board(board) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_board = match Board::try_from(original_app_entry) {
                            Ok(entry) => entry,
                            Err(e) => {
                                return Ok(ValidateCallbackResult::Invalid(format!(
                                    "Expected to get Board from Record: {e:?}"
                                )));
                            }
                        };
                        validate_update_board(action, board, original_create_action, original_board)
                    }
                    EntryTypes::Post(post) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_post = match Post::try_from(original_app_entry) {
                            Ok(entry) => entry,
                            Err(e) => {
                                return Ok(ValidateCallbackResult::Invalid(format!(
                                    "Expected to get Post from Record: {e:?}"
                                )));
                            }
                        };
                        validate_update_post(action, post, original_create_action, original_post)
                    }
                    EntryTypes::Thread(thread) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_thread = match Thread::try_from(original_app_entry) {
                            Ok(entry) => entry,
                            Err(e) => {
                                return Ok(ValidateCallbackResult::Invalid(format!(
                                    "Expected to get Thread from Record: {e:?}"
                                )));
                            }
                        };
                        validate_update_thread(
                            action,
                            thread,
                            original_create_action,
                            original_thread,
                        )
                    }
                }
            }
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterDelete(delete_entry) => {
            let original_action_hash = delete_entry.clone().action.deletes_address;
            let original_record = must_get_valid_record(original_action_hash)?;
            let original_record_action = original_record.action().clone();
            let original_action = match EntryCreationAction::try_from(original_record_action) {
                Ok(action) => action,
                Err(e) => {
                    return Ok(ValidateCallbackResult::Invalid(format!(
                        "Expected to get EntryCreationAction from Action: {e:?}"
                    )));
                }
            };
            let app_entry_type = match original_action.entry_type() {
                EntryType::App(app_entry_type) => app_entry_type,
                _ => {
                    return Ok(ValidateCallbackResult::Valid);
                }
            };
            let entry = match original_record.entry().as_option() {
                Some(entry) => entry,
                None => {
                    return Ok(ValidateCallbackResult::Invalid(
                        "Original record for a delete must contain an entry".to_string(),
                    ));
                }
            };
            let original_app_entry = match EntryTypes::deserialize_from_type(
                app_entry_type.zome_index,
                app_entry_type.entry_index,
                entry,
            )? {
                Some(app_entry) => app_entry,
                None => {
                    return Ok(ValidateCallbackResult::Invalid(
                        "Original app entry must be one of the defined entry types for this zome"
                            .to_string(),
                    ));
                }
            };
            match original_app_entry {
                EntryTypes::Board(original_board) => validate_delete_board(
                    delete_entry.clone().action,
                    original_action,
                    original_board,
                ),
                EntryTypes::Post(original_post) => validate_delete_post(
                    delete_entry.clone().action,
                    original_action,
                    original_post,
                ),
                EntryTypes::Thread(original_thread) => validate_delete_thread(
                    delete_entry.clone().action,
                    original_action,
                    original_thread,
                ),
            }
        }
        FlatOp::RegisterCreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            action,
        } => match link_type {
            LinkTypes::CreatorToThreads => {
                validate_create_link_creator_to_threads(action, base_address, target_address, tag)
            }
            LinkTypes::ThreadToThreads => {
                validate_create_link_thread_to_threads(action, base_address, target_address, tag)
            }
            LinkTypes::ThreadUpdates => {
                validate_create_link_thread_updates(action, base_address, target_address, tag)
            }
            LinkTypes::AllThreads => {
                validate_create_link_all_threads(action, base_address, target_address, tag)
            }
            LinkTypes::PostToPosts => {
                validate_create_link_post_to_posts(action, base_address, target_address, tag)
            }
            LinkTypes::PostUpdates => {
                validate_create_link_post_updates(action, base_address, target_address, tag)
            }
            LinkTypes::AllPosts => {
                validate_create_link_all_posts(action, base_address, target_address, tag)
            }
            LinkTypes::PostsByThread => {
                validate_create_link_posts_by_thread(action, base_address, target_address, tag)
            }
            LinkTypes::ThreadToPosts => {
                validate_create_link_thread_to_posts(action, base_address, target_address, tag)
            }
            LinkTypes::PostToThreads => {
                validate_create_link_post_to_threads(action, base_address, target_address, tag)
            }
            LinkTypes::CreatorToPosts => {
                validate_create_link_creator_to_posts(action, base_address, target_address, tag)
            }
            LinkTypes::PostToCreators => {
                validate_create_link_post_to_creators(action, base_address, target_address, tag)
            }
            LinkTypes::CreatorToBoards => {
                validate_create_link_creator_to_boards(action, base_address, target_address, tag)
            }
            LinkTypes::BoardUpdates => {
                validate_create_link_board_updates(action, base_address, target_address, tag)
            }
            LinkTypes::AllBoards => {
                validate_create_link_all_boards(action, base_address, target_address, tag)
            }
            LinkTypes::ThreadToBoards => {
                validate_create_link_thread_to_boards(action, base_address, target_address, tag)
            }
            LinkTypes::BoardToThreads => {
                validate_create_link_board_to_threads(action, base_address, target_address, tag)
            }
        },
        FlatOp::RegisterDeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        } => match link_type {
            LinkTypes::CreatorToThreads => validate_delete_link_creator_to_threads(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ThreadToThreads => validate_delete_link_thread_to_threads(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ThreadUpdates => validate_delete_link_thread_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllThreads => validate_delete_link_all_threads(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::PostToPosts => validate_delete_link_post_to_posts(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::PostUpdates => validate_delete_link_post_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllPosts => validate_delete_link_all_posts(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::PostsByThread => validate_delete_link_posts_by_thread(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ThreadToPosts => validate_delete_link_thread_to_posts(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::PostToThreads => validate_delete_link_post_to_threads(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::CreatorToPosts => validate_delete_link_creator_to_posts(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::PostToCreators => validate_delete_link_post_to_creators(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::CreatorToBoards => validate_delete_link_creator_to_boards(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::BoardUpdates => validate_delete_link_board_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllBoards => validate_delete_link_all_boards(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ThreadToBoards => validate_delete_link_thread_to_boards(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::BoardToThreads => validate_delete_link_board_to_threads(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
        },
        FlatOp::StoreRecord(store_record) => {
            match store_record {
                // Complementary validation to the `StoreEntry` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `StoreEntry`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `StoreEntry` validation failed
                OpRecord::CreateEntry { app_entry, action } => match app_entry {
                    EntryTypes::Thread(thread) => {
                        validate_create_thread(EntryCreationAction::Create(action), thread)
                    }
                    EntryTypes::Post(post) => {
                        validate_create_post(EntryCreationAction::Create(action), post)
                    }
                    EntryTypes::Board(board) => {
                        validate_create_board(EntryCreationAction::Create(action), board)
                    }
                },
                // Complementary validation to the `RegisterUpdate` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `StoreEntry` and in `RegisterUpdate`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the other validations failed
                OpRecord::UpdateEntry {
                    original_action_hash,
                    app_entry,
                    action,
                    ..
                } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(ValidateCallbackResult::Invalid(
                                "Original action for an update must be a Create or Update action"
                                    .to_string(),
                            ));
                        }
                    };
                    match app_entry {
                        EntryTypes::Thread(thread) => {
                            let result = validate_create_thread(
                                EntryCreationAction::Update(action.clone()),
                                thread.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_thread: Option<Thread> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_thread = match original_thread {
                                    Some(thread) => thread,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_thread(
                                    action,
                                    thread,
                                    original_action,
                                    original_thread,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::Post(post) => {
                            let result = validate_create_post(
                                EntryCreationAction::Update(action.clone()),
                                post.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_post: Option<Post> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_post = match original_post {
                                    Some(post) => post,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_post(action, post, original_action, original_post)
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::Board(board) => {
                            let result = validate_create_board(
                                EntryCreationAction::Update(action.clone()),
                                board.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_board: Option<Board> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_board = match original_board {
                                    Some(board) => board,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_board(
                                    action,
                                    board,
                                    original_action,
                                    original_board,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                    }
                }
                // Complementary validation to the `RegisterDelete` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `RegisterDelete`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `RegisterDelete` validation failed
                OpRecord::DeleteEntry {
                    original_action_hash,
                    action,
                    ..
                } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(ValidateCallbackResult::Invalid(
                                "Original action for a delete must be a Create or Update action"
                                    .to_string(),
                            ));
                        }
                    };
                    let app_entry_type = match original_action.entry_type() {
                        EntryType::App(app_entry_type) => app_entry_type,
                        _ => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    let entry = match original_record.entry().as_option() {
                        Some(entry) => entry,
                        None => {
                            return Ok(ValidateCallbackResult::Invalid(
                                "Original record for a delete must contain an entry".to_string(),
                            ));
                        }
                    };
                    let original_app_entry = match EntryTypes::deserialize_from_type(
                        app_entry_type.zome_index,
                        app_entry_type.entry_index,
                        entry,
                    )? {
                        Some(app_entry) => app_entry,
                        None => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match original_app_entry {
                        EntryTypes::Thread(original_thread) => {
                            validate_delete_thread(action, original_action, original_thread)
                        }
                        EntryTypes::Post(original_post) => {
                            validate_delete_post(action, original_action, original_post)
                        }
                        EntryTypes::Board(original_board) => {
                            validate_delete_board(action, original_action, original_board)
                        }
                    }
                }
                // Complementary validation to the `RegisterCreateLink` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `RegisterCreateLink`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `RegisterCreateLink` validation failed
                OpRecord::CreateLink {
                    base_address,
                    target_address,
                    tag,
                    link_type,
                    action,
                } => match link_type {
                    LinkTypes::CreatorToThreads => validate_create_link_creator_to_threads(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::ThreadToThreads => validate_create_link_thread_to_threads(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::ThreadUpdates => validate_create_link_thread_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::AllThreads => {
                        validate_create_link_all_threads(action, base_address, target_address, tag)
                    }
                    LinkTypes::PostToPosts => validate_create_link_post_to_posts(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::PostUpdates => {
                        validate_create_link_post_updates(action, base_address, target_address, tag)
                    }
                    LinkTypes::AllPosts => {
                        validate_create_link_all_posts(action, base_address, target_address, tag)
                    }
                    LinkTypes::PostsByThread => validate_create_link_posts_by_thread(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::ThreadToPosts => validate_create_link_thread_to_posts(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::PostToThreads => validate_create_link_post_to_threads(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::CreatorToPosts => validate_create_link_creator_to_posts(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::PostToCreators => validate_create_link_post_to_creators(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::CreatorToBoards => validate_create_link_creator_to_boards(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::BoardUpdates => validate_create_link_board_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::AllBoards => {
                        validate_create_link_all_boards(action, base_address, target_address, tag)
                    }
                    LinkTypes::ThreadToBoards => validate_create_link_thread_to_boards(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::BoardToThreads => validate_create_link_board_to_threads(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                },
                // Complementary validation to the `RegisterDeleteLink` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `RegisterDeleteLink`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `RegisterDeleteLink` validation failed
                OpRecord::DeleteLink {
                    original_action_hash,
                    base_address,
                    action,
                } => {
                    let record = must_get_valid_record(original_action_hash)?;
                    let create_link = match record.action() {
                        Action::CreateLink(create_link) => create_link.clone(),
                        _ => {
                            return Ok(ValidateCallbackResult::Invalid(
                                "The action that a DeleteLink deletes must be a CreateLink"
                                    .to_string(),
                            ));
                        }
                    };
                    let link_type = match LinkTypes::from_type(
                        create_link.zome_index,
                        create_link.link_type,
                    )? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    match link_type {
                        LinkTypes::CreatorToThreads => validate_delete_link_creator_to_threads(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ThreadToThreads => validate_delete_link_thread_to_threads(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ThreadUpdates => validate_delete_link_thread_updates(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::AllThreads => validate_delete_link_all_threads(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::PostToPosts => validate_delete_link_post_to_posts(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::PostUpdates => validate_delete_link_post_updates(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::AllPosts => validate_delete_link_all_posts(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::PostsByThread => validate_delete_link_posts_by_thread(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ThreadToPosts => validate_delete_link_thread_to_posts(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::PostToThreads => validate_delete_link_post_to_threads(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::CreatorToPosts => validate_delete_link_creator_to_posts(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::PostToCreators => validate_delete_link_post_to_creators(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::CreatorToBoards => validate_delete_link_creator_to_boards(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::BoardUpdates => validate_delete_link_board_updates(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::AllBoards => validate_delete_link_all_boards(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ThreadToBoards => validate_delete_link_thread_to_boards(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::BoardToThreads => validate_delete_link_board_to_threads(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                    }
                }
                OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterAgentActivity(agent_activity) => match agent_activity {
            OpActivity::CreateAgent { agent, action } => {
                let previous_action = must_get_action(action.prev_action)?;
                match previous_action.action() {
                        Action::AgentValidationPkg(
                            AgentValidationPkg { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
            }
            _ => Ok(ValidateCallbackResult::Valid),
        },
    }
}
