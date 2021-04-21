import {
  Orchestrator,
  Config,
  InstallAgentsHapps,
  Player,
} from "@holochain/tryorama";
import path from "path";

const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const exercise = path.join(__dirname, "../../workdir/exercise.dna");

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [exercise],
  ],
  [
    // happ 0
    [exercise],
  ],
];

const sleep = (ms) =>
  new Promise((resolve) => setTimeout(() => resolve(null), ms));

const orchestrator = new Orchestrator();

orchestrator.registerScenario(
  "Make some links with tags and test retrieval",
  async (s, t) => {
    const [alice]: Player[] = await s.players([conductorConfig]);

    const [[alice_common], [bob_common]] = await alice.installAgentsHapps(
      installation
    );

    const alicePubKey = alice_common.cells[0].cellId[1];
    const bobPubKey = bob_common.cells[0].cellId[1];

    /**
     * Test Set 1.
     *   3 tests.
     *   Create two entries and generate links between the agent
     *     and each entry. Tags are the same for each entry.
     *   Find length of Vec<Post> when specifying the tags.
     */

    let entry_1 = await alice_common.cells[0].call("exercise", "create_post", {
      content: "This is entry 1",
    });

    let entry_2 = await alice_common.cells[0].call("exercise", "create_post", {
      content: "This is entry 2",
    });

    t.ok(entry_1);
    t.ok(entry_2);

    await sleep(100);

    let posts = await alice_common.cells[0].call(
      "exercise",
      "get_posts_for_agent",
      alicePubKey
    );

    t.equal(posts.length, 2);

    posts = await bob_common.cells[0].call(
      "exercise",
      "get_posts_for_agent",
      alicePubKey
    );

    t.equal(posts.length, 2);

    posts = await bob_common.cells[0].call(
      "exercise",
      "get_posts_for_agent",
      bobPubKey
    );

    t.equal(posts.length, 0);

    /**
     * Test Set 2.
     *   3 tests.
     *   Bob creates a new post.
     *   Get Alice posts returns 2 posts and get Bob posts returns 1.
     */

    let entry_3 = await bob_common.cells[0].call("exercise", "create_post", {
      content: "This is entry 2",
    });

    t.ok(entry_3);
    await sleep(100);

    let bob_posts = await alice_common.cells[0].call(
      "exercise",
      "get_posts_for_agent",
      bobPubKey
    );

    let alice_posts = await bob_common.cells[0].call(
      "exercise",
      "get_posts_for_agent",
      alicePubKey
    );

    t.equal(bob_posts.length, 1);
    t.equal(alice_posts.length, 2);
  }
);

orchestrator.run();
