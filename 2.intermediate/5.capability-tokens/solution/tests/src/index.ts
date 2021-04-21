import { Orchestrator, Config, InstallAgentsHapps } from "@holochain/tryorama";
import path from "path";

const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const capExercise = path.join(__dirname, "../../workdir/exercise.dna");

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [capExercise],
  ],
];

const sleep = (ms) =>
  new Promise((resolve) => setTimeout(() => resolve(null), ms));

const orchestrator = new Orchestrator();

orchestrator.registerScenario(
  "grant cap access and retrieve those created grants",
  async (s, t) => {
    const [alice, bob] = await s.players([conductorConfig, conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_common]] = await alice.installAgentsHapps(installation);
    const [[bob_common]] = await bob.installAgentsHapps(installation);

    await s.shareAllNodes([alice, bob])

    let capGrant = await alice_common.cells[0].call(
      "exercise",
      "create_transferable_cap_access",
      {
        function: "test_access",
        agent: bob_common.agent,
      }
    );
    console.log(capGrant);
    t.ok(capGrant);

    console.log("Getting cap tokens for bob");
    let capAccess = await bob_common.cells[0].call(
      "exercise",
      "get_cap_tokens",
      alice_common.agent
    );
    console.log(capAccess);
    t.ok(capAccess);
    t.deepEqual(capAccess.length, 1);

    let capAssignedGrant = await bob_common.cells[0].call(
      "exercise",
      "create_assigned_cap_access",
      {
        function: "test_access",
        agent: alice_common.agent,
      }
    );
    console.log(capGrant);
    t.ok(capAssignedGrant);

    console.log("Getting cap tokens for alice");
    let capAccess2 = await alice_common.cells[0].call(
      "exercise",
      "get_cap_tokens",
      bob_common.agent
    );
    console.log(capAccess2);
    t.ok(capAccess2);
    t.deepEqual(capAccess2.length, 1);
  }
);

orchestrator.run();
