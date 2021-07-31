# git-hooks-dispatch

Run hooks only in a directory where files are changed. Useful for monorepo.



## Set up

Set up hooks you want as the following:

Hook file (e.g. `.git/hooks/pre-commit`)

```
#!/bin/sh
git-hooks-dispatch $(basename $0) "$@"
```

Make sure the file has permission to execute.

Then, in the child directories, you can set up hooks as they are in the project root directory.

For example:

`./sub-project1/git-hooks/pre-commit`

```
#!/bin/sh
npm run lint
```

`./sub-project2/git-hooks/pre-commit`

```
#!/bin/sh
mvn antrun:run@ktlint-format
```





### Manage hooks in your Git repository (recommended)

```
git config core.hooksPath git-hooks
```

