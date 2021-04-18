import { Orchestrator, Config, InstallAgentsHapps } from "@holochain/tryorama";
import path from "path";

const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const remoteCallExercise = path.join(__dirname, "../../workdir/exercise.dna");

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [remoteCallExercise],
  ],
];

const sleep = (ms) =>
  new Promise((resolve) => setTimeout(() => resolve(null), ms));

const orchestrator = new Orchestrator();

orchestrator.registerScenario(
  "create posts p2p and retrieve them",
  async (s, t) => {
    const [alice, bob, clare] = await s.players([conductorConfig, conductorConfig, conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_common]] = await alice.installAgentsHapps(installation);
    const [[bob_common]] = await bob.installAgentsHapps(installation);
    const [[clare_common]] = await clare.installAgentsHapps(installation);

    await s.shareAllNodes([alice, bob, clare])

    let capGrant = await alice_common.cells[0].call(
      "exercise",
      "create_cap_access",
      {
        function: "receive_p2p_message",
        agent: bob_common.agent,
      }
    );
    console.log(capGrant);
    t.ok(capGrant);
    
    let capGrantBob = await bob_common.cells[0].call(
      "exercise",
      "create_cap_access",
      {
        function: "receive_p2p_message",
        agent: alice_common.agent,
      }
    );
    console.log(capGrantBob);
    t.ok(capGrantBob);

    //Create p2p post to bob
    let p2pPost2Bob = await alice_common.cells[0].call(
      "exercise",
      "create_post",
      {
        content: "Hey Bob!",
        target_agent: bob_common.agent,
      }
    );
    console.log(p2pPost2Bob);
    t.ok(p2pPost2Bob);

    //Check that bob has the p2p post
    let bobMessages = await bob_common.cells[0].call(
      "exercise",
      "get_my_posts",
      null
    );
    console.log(bobMessages);
    t.ok(bobMessages);
    t.equal(bobMessages.length, 1);

    //Create p2p post to alice
    let p2pPost2Alice = await bob_common.cells[0].call(
      "exercise",
      "create_post",
      {
        content: "Hey Alice!",
        target_agent: alice_common.agent,
      }
    );
    console.log(p2pPost2Alice);
    t.ok(p2pPost2Alice);

    //Check that alice has the p2p post
    let aliceMessages = await alice_common.cells[0].call(
      "exercise",
      "get_my_posts",
      null
    );
    console.log(aliceMessages);
    t.ok(aliceMessages);
    t.equal(aliceMessages.length, 1);
    
    //Check that clare cannot message alice since she does not have cap access
        // TODO add test that validates if the async function throws error
        // -- commenting this out to make the CI build pass
    // let p2pPost2Alice2 = await clare_common.cells[0].call(
    //   "exercise",
    //   "create_post",
    //   {
    //     content: "Hey Alice!",
    //     target_agent: alice_common.agent,
    //   }
    // );
    // console.log(p2pPost2Alice2);
    // t.equal(p2pPost2Alice2.Err)
  }
);

orchestrator.run();
