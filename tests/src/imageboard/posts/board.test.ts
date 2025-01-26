import { assert, test } from "vitest";

import {
  ActionHash,
  AppBundleSource,
  CreateLink,
  DeleteLink,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash,
  Link,
  NewEntryAction,
  Record,
  SignedActionHashed,
} from "@holochain/client";
import { CallableCell, dhtSync, runScenario } from "@holochain/tryorama";
import { decode } from "@msgpack/msgpack";

import { createBoard, sampleBoard } from "./common.js";

test("create Board", async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/holochan.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a Board
    const record: Record = await createBoard(alice.cells[0]);
    assert.ok(record);
  });
});

test("create and read Board", async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/holochan.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleBoard(alice.cells[0]);

    // Alice creates a Board
    const record: Record = await createBoard(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created Board
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_original_board",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);

    // Bob gets the Creators for the new Board
    let linksToCreators: Link[] = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_boards_for_creator",
      payload: sample.creator,
    });
    assert.equal(linksToCreators.length, 1);
    assert.deepEqual(linksToCreators[0].target, record.signed_action.hashed.hash);
  });
});

test("create and update Board", async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/holochan.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a Board
    const record: Record = await createBoard(alice.cells[0]);
    assert.ok(record);

    const originalActionHash = record.signed_action.hashed.hash;

    // Alice updates the Board
    let contentUpdate: any = await sampleBoard(alice.cells[0]);
    let updateInput = {
      original_board_hash: originalActionHash,
      previous_board_hash: originalActionHash,
      updated_board: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "update_board",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the updated Board
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_latest_board",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the Board again
    contentUpdate = await sampleBoard(alice.cells[0]);
    updateInput = {
      original_board_hash: originalActionHash,
      previous_board_hash: updatedRecord.signed_action.hashed.hash,
      updated_board: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "update_board",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the updated Board
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_latest_board",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);

    // Bob gets all the revisions for Board
    const revisions: Record[] = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_all_revisions_for_board",
      payload: originalActionHash,
    });
    assert.equal(revisions.length, 3);
    assert.deepEqual(contentUpdate, decode((revisions[2].entry as any).Present.entry) as any);
  });
});

test("create and delete Board", async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/holochan.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleBoard(alice.cells[0]);

    // Alice creates a Board
    const record: Record = await createBoard(alice.cells[0], sample);
    assert.ok(record);

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the Creators for the new Board
    let linksToCreators: Link[] = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_boards_for_creator",
      payload: sample.creator,
    });
    assert.equal(linksToCreators.length, 1);
    assert.deepEqual(linksToCreators[0].target, record.signed_action.hashed.hash);

    // Alice deletes the Board
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "delete_board",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the oldest delete for the Board
    const oldestDeleteForBoard: SignedActionHashed = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_oldest_delete_for_board",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(oldestDeleteForBoard);

    // Bob gets the deletions for the Board
    const deletesForBoard: SignedActionHashed[] = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_all_deletes_for_board",
      payload: record.signed_action.hashed.hash,
    });
    assert.equal(deletesForBoard.length, 1);

    // Bob gets the Creators for the Board again
    linksToCreators = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_boards_for_creator",
      payload: sample.creator,
    });
    assert.equal(linksToCreators.length, 0);

    // Bob gets the deleted Creators for the Board
    const deletedLinksToCreators: Array<[SignedActionHashed<CreateLink>, SignedActionHashed<DeleteLink>[]]> = await bob
      .cells[0].callZome({
        zome_name: "posts",
        fn_name: "get_deleted_boards_for_creator",
        payload: sample.creator,
      });
    assert.equal(deletedLinksToCreators.length, 1);
  });
});
