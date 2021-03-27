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
  "add, get and update label",
  async (s, t) => {
    const [alice] = await s.players([conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_common]] = await alice.installAgentsHapps(installation);

    // add and get label #1 pistacchios
    let headerHash1 = await alice_common.cells[0].call(
      "exercise",
      "add_label",
      "#1 pistacchios",
    );
    let label1 = await alice_common.cells[0].call(
      "exercise",
      "get_label",
       headerHash1 
    );
    t.equal(label1, "#1 pistacchios");

    // add and get label #2 brazil nuts
    let headerHash2 = await alice_common.cells[0].call(
      "exercise",
      "add_label",
      "#2 brazil nuts",
    );
    let label2 = await alice_common.cells[0].call(
      "exercise",
      "get_label",
       headerHash2 
    );
    t.equal(label2, "#2 brazil nuts");

    // add and get label #3 peanuts
    let headerHash3 = await alice_common.cells[0].call(
      "exercise",
      "add_label",
      "#3 peanuts",
    );
    let label3 = await alice_common.cells[0].call(
      "exercise",
      "get_label",
       headerHash3
    );
    t.equal(label3, "#3 peanuts");

    // update label #3 peanuts to #3 cashews
    let headerHash3update = await alice_common.cells[0].call(
      "exercise",
      "update_label",
      {
        label: "#3 cashews",
        header_hash: headerHash3,  //headerhash of the entry to update
      },
    );
    let label3update = await alice_common.cells[0].call(
      "exercise",
      "get_label",
      headerHash3update
    );
    t.equal(label3update, "#3 cashews");

    // You can still get the original entry
    let label3again = await alice_common.cells[0].call(
      "exercise",
      "get_label",
       headerHash3
    );
    t.equal(label3again, "#3 peanuts");
  }
);

orchestrator.run();
