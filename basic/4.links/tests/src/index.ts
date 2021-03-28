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
     * Create three entries with string content, make links between entries 1->2 and 1->3 
     * with appropriate tags, and then find all links associated with ‘my_tag’, which should 
     * return a List with 1 entry header target. Passing this target back into the zome yields the 
     * associated entry content ‘Entry2’.
     */

    let entry_1 = await alice_common.cells[0].call(
      "exercise",
      "make_entry_and_hash",
      {content: "Entry1"}
    );

    let entry_2 = await alice_common.cells[0].call(
      "exercise",
      "make_entry_and_hash",
      {content: "Entry2"}
    );

    let entry_3 = await alice_common.cells[0].call(
      "exercise",
      "make_entry_and_hash",
      {content: "Entry3"}
    );

    let link_1 = await alice_common.cells[0].call(
      "exercise",
      "make_links",
      {
        base: entry_1,
        target: entry_2,
        tag: 'my_tag',
      }
    )

    let link_2 = await alice_common.cells[0].call(
      "exercise",
      "make_links",
      {
        base: entry_1,
        target: entry_3,
        tag: 'another_tag',
      }
    )
  
    let link_info = await alice_common.cells[0].call(
      "exercise",
      "find_links",
      {
        base: entry_1,
        tag: 'my_tag'
      }
    )

    let result_target = await alice_common.cells[0].call(
      "exercise",
      "get_entry_from_input",
      {
        hash: link_info[0].target,
      }
    )

    t.ok(link_info.length === 1);
    t.ok(result_target === 'Entry2');

    /**
     * Test Set 2. 
     * Using the above entries, find all Links associated for the first entry without a tag filter.
     * Then, passing in the entry addresses found from both links back into the zome, we
     * gather the content of entries two and three, which are ‘Entry2’ and ‘Entry3’ respectively.
     */

    let link_info_2 = await alice_common.cells[0].call(
      "exercise",
      "find_links",
      {
        base: entry_1,
        tag: '',
      }
    )

    let result_target_2a = await alice_common.cells[0].call(
      "exercise",
      "get_entry_from_input",
      {
        hash: link_info_2[0].target,
      }
    )

    
    let result_target_2b = await alice_common.cells[0].call(
      "exercise",
      "get_entry_from_input",
      {
        hash: link_info_2[1].target,
      }
    )

    t.ok(link_info_2.length === 2);
    t.ok(result_target_2a === 'Entry2');
    t.ok(result_target_2b === 'Entry3');

  }
);

orchestrator.run();
