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

    // Alice creates an entry with her username and password in - A BAD IDEA
    let entry_hash1 = await alice_common.cells[0].call("exercise", "add_password", {
      username: "Alice",
      code: "1234",
    });

    t.ok(entry_hash1);

    await sleep(100);

    // Alice gets her own password with with the entry/header hash she has
    let password1 = await alice_common.cells[0].call(
      "exercise",
      "get_password",
      entry_hash1
    );

    t.equal(password1.code, "1234");
    
    // if Bob some how knows the hash, he can get the entry
    let password2 = await bob_common.cells[0].call(
      "exercise",
      "get_password",
      entry_hash1
    );

    t.equal(password2.code, "1234");

    // even if Bob does not know the hash, he can GUESS the entry
    let password3 = await bob_common.cells[0].call(
      "exercise",
      "get_hash_by_content",
      {
        username: "Alice",
        code: "1234",
      }
    );

    t.equal(password3.code, "1234");

    // The DHT that is created by your holochain app is a PUBLIC space. 
    // You can limit who has access to this public space
    // but when someone has entered this space
    // it is harder to keep things secret
    // This is by design: holochain is COLLABORATION CENTRIC
    // It is hard to work together if want to keep everything secret
  }
);

orchestrator.run();
