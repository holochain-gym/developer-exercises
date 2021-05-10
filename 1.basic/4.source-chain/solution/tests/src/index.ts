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
  // agent 1
  [
    // happ 0
    [exercise],
  ],
];

const sleep = (ms) =>
  new Promise((resolve) => setTimeout(() => resolve(null), ms));

const millisToTimestamp = (ms) => [Math.floor(ms / 1000), ms % 1000];

const orchestrator = new Orchestrator();

orchestrator.registerScenario(
  "register snackings, and execute different queries on the source chain",
  async (s, t) => {
    const [alice] = await s.players([conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_common], [bob_common]] = await alice.installAgentsHapps(
      installation
    );

    let elements = await alice_common.cells[0].call(
      "exercise",
      "query_all_elements",
      null
    );
    t.equal(elements.length, 4);

    elements = await bob_common.cells[0].call(
      "exercise",
      "query_all_elements",
      null
    );
    t.equal(elements.length, 4);

    await sleep(1000);
    
    const start = millisToTimestamp(Date.now());

    await sleep(1000);

    // <add snacking log: >"april 2: lemon pie"
    await alice_common.cells[0].call(
      "exercise",
      "register_snacking",
      "april 2: lemon pie"
    );

    // <add snacking log: >"april 2: lemon pie"
    await alice_common.cells[0].call(
      "exercise",
      "register_snacking",
      "april 2: lemon pie"
    );

    elements = await alice_common.cells[0].call(
      "exercise",
      "query_all_elements",
      null
    );
    t.equal(elements.length, 6);

    elements = await bob_common.cells[0].call("exercise", "query_all_elements", null);
    t.equal(elements.length, 4);

    elements = await alice_common.cells[0].call(
      "exercise",
      "query_snackings",
      null
    );
    t.equal(elements.length, 2);
    t.ok(elements[0].entry.Present)

    elements = await bob_common.cells[0].call(
      "exercise",
      "query_snackings",
      null
    );
    t.equal(elements.length, 0);

    await bob_common.cells[0].call(
      "exercise",
      "register_snacking",
      "april 2: lemon pie"
    );

    elements = await bob_common.cells[0].call(
      "exercise",
      "query_snackings",
      null
    );
    t.equal(elements.length, 1);

    await sleep(1000);
    
    const middle = millisToTimestamp(Date.now());

    await sleep(1000);

    await bob_common.cells[0].call(
      "exercise",
      "register_snacking",
      "april 2: lemon pie"
    );
    await bob_common.cells[0].call(
      "exercise",
      "register_snacking",
      "april 2: lemon pie"
    );

    await sleep(1000);
    
    const end = millisToTimestamp(Date.now());
    
    await sleep(1000);
    await bob_common.cells[0].call(
      "exercise",
      "register_snacking",
      "april 2: lemon pie"
    );

    elements = await bob_common.cells[0].call("exercise", "query_by_time", {
      start_time: start,
      end_time: middle,
    });
    t.equal(elements.length, 1);
    elements = await bob_common.cells[0].call("exercise", "query_by_time", {
      start_time: start,
      end_time: end,
    });
    t.equal(elements.length, 3);
    elements = await bob_common.cells[0].call("exercise", "query_by_time", {
      start_time: middle,
      end_time: end,
    });
    t.equal(elements.length, 2);
  }
);

orchestrator.run();
