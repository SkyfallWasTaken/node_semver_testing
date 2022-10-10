# `node_semver` tester
This tests `node_semver` against the `npm` semver library.

## Commands
Run these **in order.**

These commands are made this way so that if one of the steps are interrupted, you can get going faster because the data from previous steps is still there.

If you're _sure_ you won't get interrupted, run:

```sh
$ chmod +x full_dump.sh
$ ./full_dump.sh
```

This will remove all working files, and run each step.

- **write-dump:** This dumps all npm package names to `dump.json`, as well as CouchDB revisions.
  - ⚠️ This takes a _long_ time, and the resulting file is _very_ large. If distributing said file, you basically have to compress said file as best as you can. However, you likely want to use...
- **extract-package-names:** This takes `dump.json` and simply outputs the names.
- **dump-versions-and-ranges:** This gets all the versions and ranges, and dumps them to `versions.txt` and `ranges.txt` respectively. 
- **test-versions-and-ranges:** Tests `node_semver` and npm semver parsing using `versions.txt` and `ranges.txt`.