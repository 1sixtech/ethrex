window.BENCHMARK_DATA = {
  "lastUpdate": 1752776111206,
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
      }
    ]
  }
}