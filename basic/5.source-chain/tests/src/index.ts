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
  "test",
  async (s, t) => {
    const [alice] = await s.players([conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_common]] = await alice.installAgentsHapps(installation);

    // <add snacking log: >"april 2: lemon pie"
    let headerHash1 = await alice_common.cells[0].call(
      "exercise",
      "register_snacking",
      "april 2: lemon pie",
    );
    console.log("hash1");
    console.log(headerHash1);

    let headerHash2 = await alice_common.cells[0].call(
      "exercise",
      "register_snacking",
      "april 4: chocolates",
    );
    console.log("hash2");
    console.log(headerHash2);

    let headerHash3 = await alice_common.cells[0].call(
      "exercise",
      "register_snacking",
      "april 5: marshmallows",
    );
    console.log("hash3");
    console.log(headerHash3);

    // get snacking log based on headerhash
    let answer1 = await alice_common.cells[0].call(
      "exercise",
      "is_previous_header_hash",
      {
        current: headerHash2,
        previous: headerHash1
      }
    );
    t.equal(answer1, "IS previous header");

    // get snacking log based on headerhash
    let answer2 = await alice_common.cells[0].call(
      "exercise",
      "happened_before",
      {
        current: headerHash3,
        previous: headerHash1
      }
    );
    console.log(answer2);
    t.equal(answer2, "happened before");
  }
);

orchestrator.run();
