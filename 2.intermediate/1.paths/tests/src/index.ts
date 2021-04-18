import { Orchestrator, Config, InstallAgentsHapps } from "@holochain/tryorama";
import path from "path";

const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const pathsExercise = path.join(__dirname, "../../workdir/exercise.dna");

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [pathsExercise],
  ],
];

const sleep = (ms) =>
  new Promise((resolve) => setTimeout(() => resolve(null), ms));

const orchestrator = new Orchestrator();

orchestrator.registerScenario(
  "create posts and retrieve them",
  async (s, t) => {
    const [alice] = await s.players([conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_common]] = await alice.installAgentsHapps(installation);

    let entryHash = await alice_common.cells[0].call(
      "exercise",
      "create_post",
      {
        content: "good morning",
        tags: ["nature", "giraffe"],
      }
    );
    t.ok(entryHash);
    await alice_common.cells[0].call("exercise", "create_post", {
      content: "Inception",
      tags: ["movie"],
    });
    t.ok(entryHash);
    await sleep(500);

    const all_tags = await alice_common.cells[0].call(
      "exercise",
      "get_all_tags",
      null
    );
    t.deepEqual(all_tags, ["movie", "nature", "giraffe"]);

    const naturePosts = await alice_common.cells[0].call(
      "exercise",
      "get_posts_by_tag",
      "nature"
    );
    t.deepEqual(naturePosts, ["good morning"]);

    const moviePosts = await alice_common.cells[0].call(
      "exercise",
      "get_posts_by_tag",
      "movie"
    );
    t.deepEqual(moviePosts, ["Inception"]);

    let postsByTime = await alice_common.cells[0].call(
      "exercise",
      "get_posts_by_time",
      {
        year: 0,
        month: 0,
        day: 0,
        hour: null,
      }
    );
    t.deepEqual(postsByTime, []);

    const date = new Date();
    postsByTime = await alice_common.cells[0].call(
      "exercise",
      "get_posts_by_time",
      {
        year: date.getUTCFullYear(),
        month: date.getMonth() + 1,
        day: date.getUTCDate(),
        hour: null,
      }
    );
    t.deepEqual(postsByTime, ["good morning", "Inception"]);

    postsByTime = await alice_common.cells[0].call(
      "exercise",
      "get_posts_by_time",
      {
        year: date.getUTCFullYear(),
        month: date.getMonth() + 1,
        day: date.getUTCDate(),
        hour: date.getUTCHours() - 1,
      }
    );
    t.deepEqual(postsByTime, []);

    postsByTime = await alice_common.cells[0].call(
      "exercise",
      "get_posts_by_time",
      {
        year: date.getUTCFullYear(),
        month: date.getMonth() + 1,
        day: date.getUTCDate(),
        hour: date.getUTCHours(),
      }
    );
    t.deepEqual(postsByTime, ["good morning", "Inception"]);
  }
);

orchestrator.run();
