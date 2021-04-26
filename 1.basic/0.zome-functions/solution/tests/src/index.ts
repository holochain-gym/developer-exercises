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
  "zome-functions",
  async (s, t) => {
    const [alice] = await s.players([conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_common]] = await alice.installAgentsHapps(installation);

    // Hello world
    let text_hello_world = await alice_common.cells[0].call(
      "exercise",
      "hello_world",
      null
    );
    t.equal(text_hello_world, "Hello world");

    // Say my name
    let text_name = await alice_common.cells[0].call(
      "exercise",
      "say_my_name",
      {
        first_name: "Srinivasa",
        last_name: "Ramanujan",
      }
    );
    console.log(text_name);
    t.equal(text_name, "Your name is Srinivasa Ramanujan");

    // Agent info
    let agent_info = await alice_common.cells[0].call(
      "exercise",
      "get_agent_id",
      null
    );
    t.ok(agent_info);
    console.log(agent_info);
    
  }
);

orchestrator.run();
