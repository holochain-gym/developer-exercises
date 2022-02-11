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

const orchestrator = new Orchestrator();

orchestrator.registerScenario(
  "add and retrieve a book",
  async (s, t) => {
    const [alice] = await s.players([conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_common]] = await alice.installAgentsHapps(installation);

    const book = {
      title: "Sovereign Accountable Commons",
      content: "A Sovereign Accountable Commons (SAC) is akin to idea of a Decentralized Autonomous Organizations (DAO) on Ethereum but the underlying technology is fundamentally different, as SACs are built on Ceptr and Holochain. http://ceptr.org/projects/sovereign",
    };

    let entryHash: Buffer = await alice_common.cells[0].call(
      "exercise",
      "add_book",
      book
    );

    t.ok(entryHash, "test add book");
  }
);

orchestrator.run();
