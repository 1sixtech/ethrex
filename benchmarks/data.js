window.BENCHMARK_DATA = {
  "lastUpdate": 1757520291835,
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
      },
      {
        "commit": {
          "author": {
            "email": "72628438+avilagaston9@users.noreply.github.com",
            "name": "Avila Gastón",
            "username": "avilagaston9"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "da96b3878ff45cb8a1789f0a52124ec624fa4e38",
          "message": "refactor(l2): remove WrappedTransaction (#3984)\n\n**Motivation**\n\n`WrappedTransaction` is an abstraction that is not really necessary. We\ncan use `GenericTransaction` instead, which serves a similar purpose but\nis easier to manipulate, as we don't have to match Enum variants to\naccess its attributes.\n\n**Description**\n\n- Removes `WrappedTransaction`.\n- Updates all `WrappedTransaction` related functions (send,bump) to use\n`GenericTransactions` instead.\n- Implements mapper functions to convert a `GenericTransaction` to a\nspecific transaction type.\n- Replaces `build_eipxxx`/`send_eipxxx` functions with generic ones:\n`build_generic_tx(type)`/`send_generic_tx()`.\n\nCloses #3787\n\n---------\n\nCo-authored-by: Manuel Iñaki Bilbao <manuel.bilbao@lambdaclass.com>",
          "timestamp": "2025-08-19T23:26:47Z",
          "tree_id": "ccbe706b9e892b782a083e1f3d6a4e1fc9c5ef6d",
          "url": "https://github.com/1sixtech/ethrex/commit/da96b3878ff45cb8a1789f0a52124ec624fa4e38"
        },
        "date": 1755676056415,
        "tool": "cargo",
        "benches": [
          {
            "name": "Block import/Block import ERC20 transfers",
            "value": 162810195723,
            "range": "± 434733353",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "git@edgl.dev",
            "name": "Edgar",
            "username": "edg-l"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8c312705f4295279dad3297c1f0630fbd66685de",
          "message": "perf(levm): improve load_range by returning a Bytes directly, avoding a vec (#4098)\n\n**Motivation**\n\nAll load range usage leads to creating a Bytes, but to do so we allocate\na intermediate vec, this pr avoids this.\n\nimprove load_range by returning a Bytes directly, avoding a vec\n\n---------\n\nCo-authored-by: Mario Rugiero <mrugiero@gmail.com>",
          "timestamp": "2025-08-25T09:02:40Z",
          "tree_id": "b43426d32c02a260b64897801aef39417d83da9c",
          "url": "https://github.com/1sixtech/ethrex/commit/8c312705f4295279dad3297c1f0630fbd66685de"
        },
        "date": 1756128078357,
        "tool": "cargo",
        "benches": [
          {
            "name": "Block import/Block import ERC20 transfers",
            "value": 160198407314,
            "range": "± 283138683",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "67517699+ilitteri@users.noreply.github.com",
            "name": "Ivan Litteri",
            "username": "ilitteri"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5c0681db9dc344138192c793b88fadf7db1fcb46",
          "message": "refactor(l2): prover crate (#3800)\n\n**Description**\n\nRefactors the prover crate\n\n---------\n\nCo-authored-by: Copilot <175728472+Copilot@users.noreply.github.com>",
          "timestamp": "2025-09-03T22:13:33Z",
          "tree_id": "380745ee7fafc41c3fff02fd68ce87929d31bed8",
          "url": "https://github.com/1sixtech/ethrex/commit/5c0681db9dc344138192c793b88fadf7db1fcb46"
        },
        "date": 1756957468368,
        "tool": "cargo",
        "benches": [
          {
            "name": "Block import/Block import ERC20 transfers",
            "value": 168594276787,
            "range": "± 1090177929",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49622509+jrchatruc@users.noreply.github.com",
            "name": "Javier Rodríguez Chatruc",
            "username": "jrchatruc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "34627b89752faf652f3b60cfb75fdc22a112c2e1",
          "message": "chore(l1): lower snap sync chunk file size and move it to a constant variable (#4328)\n\n**Motivation**\n\n<!-- Why does this pull request exist? What are its goals? -->\n\n**Description**\n\n<!-- A clear and concise general description of the changes this PR\nintroduces -->\n\n<!-- Link to issues: Resolves #111, Resolves #222 -->\n\nCloses #issue_number",
          "timestamp": "2025-09-04T22:03:59Z",
          "tree_id": "a60844b65cb1cc6a919437e1dec4dcc18e6cf95d",
          "url": "https://github.com/1sixtech/ethrex/commit/34627b89752faf652f3b60cfb75fdc22a112c2e1"
        },
        "date": 1757054479164,
        "tool": "cargo",
        "benches": [
          {
            "name": "Block import/Block import ERC20 transfers",
            "value": 168172323433,
            "range": "± 436855219",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "iovoid@users.noreply.github.com",
            "name": "Lucas Fiegl",
            "username": "iovoid"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c356adc73e2ae50a465cdb1f0090d3d3e1b86323",
          "message": "feat(l1): only track state root of big accounts (#4262)\n\n**Motivation**\n\nWhen inserting storages ranges during snap sync, we keep track of the\nstorage root to insert to the same tree as we go from one chunk to\nanother. We don't need to keep track if the state root is already\ncomplete.\n\n**Description**\n\nThis PR only updates maybe_big_account_storage_state_roots if the\naccount is, in fact, big (couldn't be fetched in a single batch).",
          "timestamp": "2025-09-09T13:44:16Z",
          "tree_id": "14a74fb69b7486c74720f42d563ab7f3fbe2c560",
          "url": "https://github.com/1sixtech/ethrex/commit/c356adc73e2ae50a465cdb1f0090d3d3e1b86323"
        },
        "date": 1757430752391,
        "tool": "cargo",
        "benches": [
          {
            "name": "Block import/Block import ERC20 transfers",
            "value": 168549883501,
            "range": "± 1107646178",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "mrugiero@gmail.com",
            "name": "Mario Rugiero",
            "username": "Oppen"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3565e63abdfeb7fdbb7bfd6b40943b0c891cc6eb",
          "message": "fix(l1): make account and storage ranges handlers async (#4401)\n\n**Motivation**\n\nThese handlers are currently sync and can block the runtime for quite a\nlong time. This is not the ideal solution for this; we still need to do\nthings better so the handler itself doesn't take a long time to fetch\nthe requested nodes. We'll tackle this on a subsequent PR.\n\n**Description**\n\n<!-- A clear and concise general description of the changes this PR\nintroduces -->\n\n<!-- Link to issues: Resolves #111, Resolves #222 -->\n\nCloses #issue_number\n\n---------\n\nCo-authored-by: Javier Chatruc <jrchatruc@gmail.com>",
          "timestamp": "2025-09-09T22:05:09Z",
          "tree_id": "aa0487027bcf30b9695271fa583b480e4745ee83",
          "url": "https://github.com/1sixtech/ethrex/commit/3565e63abdfeb7fdbb7bfd6b40943b0c891cc6eb"
        },
        "date": 1757520289916,
        "tool": "cargo",
        "benches": [
          {
            "name": "Block import/Block import ERC20 transfers",
            "value": 171899298702,
            "range": "± 1505010048",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}