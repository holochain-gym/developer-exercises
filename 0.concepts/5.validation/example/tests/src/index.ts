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

    // Alice add a correct estimate for an item
    let entry_hash1 = await alice_common.cells[0].call("exercise", "add_estimate", {
      item: "work-item-xyz",
      value: 13,
    });

    t.ok(entry_hash1);

    await sleep(100);

    // Alice adds a incorrect estimate for an item
    try{
      await alice_common.cells[0].call("exercise", "add_estimate", {
        item: "work-item-xyz",
        value: 6,
      });
      t.fail();
    } catch(e){
      t.ok("error thrown")
    }
    
   // TODO find out if there is a way to show validation is done by peers as well
  
  
  }
);

orchestrator.run();
