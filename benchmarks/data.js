window.BENCHMARK_DATA = {
  "lastUpdate": 1754999075963,
  "repoUrl": "https://github.com/1sixtech/ethrex",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "128638963+santiago-MV@users.noreply.github.com",
            "name": "santiago-MV",
            "username": "santiago-MV"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "229b791477f203dde019f36d5d62be182139d63a",
          "message": "refactor(l1): make ethrex-only github actions faster (#3648)\n\n**Motivation**\n\nRunning the ethrex_only github actions job seems to be slower than those\nthat use other execution clients as well\n\n**Description**\n\nThere were 2 main reasons why this job was slower compared to the others\n- The ethrex_only job includes the EOA and BLOB transactions assertoor\nplaybooks, which are the ones being run in the other two github jobs\n- The slot time of 12 sec was making the test take to long\n\nThe slot time was modified and now the tests take 10 minutes instead of\nthe original 18\n\n<!-- Link to issues: Resolves #111, Resolves #222 -->\n\nCloses #3628",
          "timestamp": "2025-07-17T16:21:11Z",
          "tree_id": "41a3b691e05e3ff267354c99a2f50f9b45e9edc7",
          "url": "https://github.com/1sixtech/ethrex/commit/229b791477f203dde019f36d5d62be182139d63a"
        },
        "date": 1752776109427,
        "tool": "cargo",
        "benches": [
          {
            "name": "Block import/Block import ERC20 transfers",
            "value": 210267068815,
            "range": "± 898799110",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "99273364+fmoletta@users.noreply.github.com",
            "name": "fmoletta",
            "username": "fmoletta"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9a771ea8f3b895153a5f122caab87a1e7e6f7967",
          "message": "feat(l1): add `--input_dir` and `--output_dir` options to archive sync (#3962)\n\n**Motivation**\nAllow writing dump state to files while archive syncing and using them\nfor archive syncing without the need of an active archive node .\nThis PR adds the following flags:\n* `--ipc_path`: replaces the previously required `archive_node_ipc`\nrequired unnamed arg\n* `--output_dir`: outputs the state data received during the sync to the\ngiven directory\n* `--input_dir`: allows fetching state data from a previous archive sync\nexecution instead of an archive node\n* `--no_sync`: skips state rebuild, only usable with `--output_dir` to\nspeed up state data writing when syncing the current node is not needed\n<!-- Why does this pull request exist? What are its goals? -->\n\n**Description**\n* Adds new CLI flags `--ipc_path`, `--input_dir`, `--output_dir`,\n`--no_sync`\n* Abstracts archive sync main behaviour into structs to accommodate new\nfeatures:\n* DumpReader: Allows reading state data from either an ipc connection or\na directory\n* DumpProcessor: Processes state data by either using it to rebuild the\nstate and/or writing it to a file\n<!-- A clear and concise general description of the changes this PR\nintroduces -->\n\n<!-- Link to issues: Resolves #111, Resolves #222 -->\n\nCloses #issue_number",
          "timestamp": "2025-08-11T22:18:28Z",
          "tree_id": "d2f1dc95e5f8e2a9903a6f710cad03a2b2910b22",
          "url": "https://github.com/1sixtech/ethrex/commit/9a771ea8f3b895153a5f122caab87a1e7e6f7967"
        },
        "date": 1754999073810,
        "tool": "cargo",
        "benches": [
          {
            "name": "Block import/Block import ERC20 transfers",
            "value": 162379673663,
            "range": "± 363751116",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}