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
  "register snacking, get snacking log by header hash, by entry hash, getting header hash by content",
  async (s, t) => {
    const [alice] = await s.players([conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_common]] = await alice.installAgentsHapps(installation);

    // <add snacking log: >"april 2: lemon pie"
    let headerAndEntryHash = await alice_common.cells[0].call(
      "exercise",
      "register_snacking",
      "april 2: lemon pie",
    );
    let entryHash = headerAndEntryHash.entry_hash;
    let headerHash = headerAndEntryHash.header_hash;

    // get snacking log based on headerhash
    let snackinglog1 = await alice_common.cells[0].call(
      "exercise",
      "get_by_header_hash",
       headerHash 
    );
    t.equal(snackinglog1, "april 2: lemon pie");

    // get snacking log based on entryhash
    let snackinglog2 = await alice_common.cells[0].call(
      "exercise",
      "get_by_entry_hash",
        entryHash 
    );
    t.equal(snackinglog2, "april 2: lemon pie");

    // get header_hash for snacking_log based on content
    let headerHash2 = await alice_common.cells[0].call(
      "exercise",
      "get_header_hash_by_content",
      "april 2: lemon pie"
    );
    t.same(headerHash, headerHash2);   // or t.deepEqual()    --> compares on content of array, not on identity
  }
);

orchestrator.run();
