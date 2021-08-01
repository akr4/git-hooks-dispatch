# git-hooks-dispatch

Run hooks only in a directory where files are changed. Useful for monorepo.

<a href="https://crates.io/crates/git-hooks-dispatch">![Crates.io](https://img.shields.io/crates/v/git-hooks-dispatch)</a> <a href="https://github.com/akr4/git-hooks-dispatch/actions">![test](https://github.com/akr4/git-hooks-dispatch/actions/workflows/test.yml/badge.svg)</a>

## Install

```
cargo install git-hooks-dispatch
```

## Setting up

Set up hooks you want as the following:

Hook file (e.g. `.git/hooks/pre-commit`)

```
#!/bin/sh
git-hooks-dispatch $(basename $0) -- "$@"
```

Make sure the file has permission to execute.

Then, in the child directories, you can set up hooks as they are in the project root directory.

For example:

`./sub-project1/git-hooks/pre-commit`

```
#!/bin/sh
npm run lint-staged
```

`./sub-project2/git-hooks/pre-commit`

```
#!/bin/sh
mvn antrun:run@ktlint-format
```

## Hooks are executed recursively

In the below example, if `./foo/bar/B` is changed, `pre-commit` hooks are executed recursively in the following order:

1. `./foo/bar/git-hooks/pre-commit`
2. `./foo/git-hooks/pre-commit`

```
.
├── .git
│  └── hooks
│     └── pre-commit
└── foo
   ├── A
   ├── git-hooks
   │  └── pre-commit
   └── bar
      ├── B
      └── git-hooks
         └── pre-commit
```

## Hooks dir name

`git-hooks-dispatch` searches a hook dir which name is `hooks-dir` or `.hooks-dir` by default. You can change it by `--hooks-dir` option.

### Manage hooks in your Git repository (recommended)

```
git config core.hooksPath git-hooks
```

## Print logs

Setting `RUST_LOG` environment variable turns logging on.

```
RUST_LOG=debug git commit ...
```
