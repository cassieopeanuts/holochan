import { assert, test } from "vitest";

import {
  ActionHash,
  AppBundleSource,
  CreateLink,
  DeleteLink,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash,
  hashFrom32AndType,
  Link,
  NewEntryAction,
  Record,
  SignedActionHashed,
} from "@holochain/client";
import { CallableCell, dhtSync, runScenario } from "@holochain/tryorama";
import { decode } from "@msgpack/msgpack";

import { createThread } from "./common.js";
import { createBoard } from "./common.js";

test("link a Thread to a Board", async () => {
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

    const baseRecord = await createThread(alice.cells[0]);
    const baseAddress = baseRecord.signed_action.hashed.hash;
    const targetRecord = await createBoard(alice.cells[0]);
    const targetAddress = targetRecord.signed_action.hashed.hash;

    // Bob gets the links, should be empty
    let linksOutput: Link[] = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_boards_for_thread",
      payload: baseAddress,
    });
    assert.equal(linksOutput.length, 0);

    // Alice creates a link from Thread to Board
    await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "add_board_for_thread",
      payload: {
        base_thread_hash: baseAddress,
        target_board_hash: targetAddress,
      },
    });

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the links again
    linksOutput = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_boards_for_thread",
      payload: baseAddress,
    });
    assert.equal(linksOutput.length, 1);
    assert.deepEqual(targetAddress, linksOutput[0].target);

    // Bob gets the links in the inverse direction
    linksOutput = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_threads_for_board",
      payload: targetAddress,
    });
    assert.equal(linksOutput.length, 1);
    assert.deepEqual(baseAddress, linksOutput[0].target);

    await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "delete_board_for_thread",
      payload: {
        base_thread_hash: baseAddress,
        target_board_hash: targetAddress,
      },
    });

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the links again
    linksOutput = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_boards_for_thread",
      payload: baseAddress,
    });
    assert.equal(linksOutput.length, 0);

    // Bob gets the deleted links
    let deletedLinksOutput: Array<[SignedActionHashed<CreateLink>, SignedActionHashed<DeleteLink>[]]> = await bob
      .cells[0].callZome({
        zome_name: "posts",
        fn_name: "get_deleted_boards_for_thread",
        payload: baseAddress,
      });
    assert.equal(deletedLinksOutput.length, 1);

    // Bob gets the links in the inverse direction
    linksOutput = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_threads_for_board",
      payload: targetAddress,
    });
    assert.equal(linksOutput.length, 0);

    // Bob gets the deleted links in the inverse direction
    deletedLinksOutput = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_deleted_threads_for_board",
      payload: targetAddress,
    });
    assert.equal(deletedLinksOutput.length, 1);
  });
});
