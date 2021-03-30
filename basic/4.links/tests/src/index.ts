import { Orchestrator, Config, InstallAgentsHapps } from "@holochain/tryorama";
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

orchestrator.registerScenario(
  "Make some links with tags and test retrieval",
  async (s, t) => {
    const [alice] = await s.players([conductorConfig]);

    const [[alice_common]] = await alice.installAgentsHapps(installation);

    /**
     * Test Set 1. 
     *   3 tests.
     *   Create two entries and generate links between the agent
     *     and each entry. Tags are the same for each entry.
     *   Find length of Vec<Post> when specifying the tags.
     */

    let entry_1 = await alice_common.cells[0].call(
      "exercise",
      "create_post",
      {
        content: "This is entry 1",
        tag: 'tag 1'
      }
    );

    let entry_2 = await alice_common.cells[0].call(
      "exercise",
      "create_post",
      {
        content: "This is entry 2",
        tag: 'tag 1'
      }
    );

    t.ok(entry_1)
    t.ok(entry_2)

    let posts = await alice_common.cells[0].call(
      "exercise",
      "get_posts_for_agent",
      'tag 1'
    );

    t.ok(posts.length == 2);

    /**
     * Test Set 2. 
     *   3 tests.
     *   Create a new entry with a different tag.
     *   Find length of Vec<Post> when specifying the 2 different tags. 
     */

    let entry_3 = await alice_common.cells[0].call(
      "exercise",
      "create_post",
      {
        content: "This is entry 2",
        tag: 'tag 2'
      }
    );

    t.ok(entry_3);

    let posts_2a = await alice_common.cells[0].call(
      "exercise",
      "get_posts_for_agent",
      'tag 1'
    );

    let posts_2b = await alice_common.cells[0].call(
      "exercise",
      "get_posts_for_agent",
      'tag 2'
    );

    t.ok(posts_2a.length == 2);
    t.ok(posts_2b.length == 1);

    /**
     * Test Set 3. 
     *   2 tests.
     *   Create a new entry with no tag.
     *   Find length of Vec<Post> when specifying no tag.
     *     When no tag is specified, all posts should be returned (4). 
     */

     let entry_4 = await alice_common.cells[0].call(
      "exercise",
      "create_post",
      {
        content: "This is entry 2",
        tag: ''
      }
    );

    t.ok(entry_4);

    let posts_3 = await alice_common.cells[0].call(
      "exercise",
      "get_posts_for_agent",
      ''
    );

    t.ok(posts_3.length == 4);

  }
);

orchestrator.run();
