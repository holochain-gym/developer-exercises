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
];

const sleep = (ms) =>
  new Promise((resolve) => setTimeout(() => resolve(null), ms));

const orchestrator = new Orchestrator();

orchestrator.registerScenario("create and access posts", async (s, t) => {
  const [alice]: Player[] = await s.players([conductorConfig]);

  const [[alice_common]] = await alice.installAgentsHapps(installation);

  await sleep(2000);

  const [[bob_common]] = await alice.installAgentsHapps(installation);

  /**
   * Test Set 1.
   *   3 tests.
   *   Create two entries.
   *   Find length of Vec<Post> when accessed by two different users.
   */

  let entry_1 = await alice_common.cells[0].call("exercise", "create_post", {
    content: "This is entry 1",
  });

  let entry_2 = await alice_common.cells[0].call("exercise", "create_post", {
    content: "This is entry 2",
  });

  t.ok(entry_1);
  t.ok(entry_2);

  await sleep(500);

  let posts = await alice_common.cells[0].call(
    "exercise",
    "get_all_posts",
    undefined
  );

  t.equal(posts.length, 2);

  posts = await bob_common.cells[0].call(
    "exercise",
    "get_all_posts",
    undefined
  );

  t.equal(posts.length, 2);
});

orchestrator.run();
