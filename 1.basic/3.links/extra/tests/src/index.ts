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

    /**
     * Test Set 1
     * */

    // create author
    let entry_hash_author = await alice_common.cells[0].call(
      "exercise", 
      "create_author", 
      "Elisabeth Sawin",
    );

    // create post
    let entry_hash_post = await alice_common.cells[0].call(
      "exercise", 
      "create_post", 
      "The Power of Multisolving for People and Climate",  //this is actually a TEDx talk https://www.youtube.com/watch?v=prF8trTallQ
    );

    t.ok(entry_hash_author);
    t.ok(entry_hash_post);

    // create link between author and post
    let link_header_hash = await alice_common.cells[0].call("exercise", "link_author_to_post", //LinkInput
      {
        author_entry_hash : entry_hash_author,
        post_entry_hash : entry_hash_post,
      }
    );
    t.ok(link_header_hash);

    // get link
    let link_header = await alice_common.cells[0].call("exercise", "get_link_header", link_header_hash);
    // check link
    t.ok(link_header);
    t.same(entry_hash_author, link_header.base_address);
    t.same(entry_hash_post, link_header.target_address);
    t.equal("is_author", link_header.tag.toString());
    t.ok(link_header.tag); 

    await sleep(100);

    // create comment and link to post 
    let link_header2 = await alice_common.cells[0].call(
      "exercise", 
      "comment_on_post", 
      {
        post_entry_hash: entry_hash_post,
        comment: "my first comment",
      });

    // check link
    t.ok(link_header2);
    t.ok(link_header2.base_address);
    t.ok(link_header2.target_address);
    t.equal("is_comment_on", link_header2.tag.toString());
    t.ok(link_header.tag); 
  

  }
);

orchestrator.run();
