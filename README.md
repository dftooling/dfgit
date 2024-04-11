# dfgit

dfgit is a small blazingly fast program that allows for tracking changes in code on the [DiamondFire Minecraft server](https://mcdiamondfire.com) using git.




## Setup

You can use the template provided at [dfgit-tool/dfgit-template](https://github.com/dfgit-tool/dfgit-template) or use any repository. dfgit does not *require* special setup in the repository.



## Usage

### `dfgit clone <repo>` - Acts like both clone and pull
Clones a repo and inserts it into your current plot.

### `dfgit commit <repo> <message>` - Acts like both commit and push
Clones a repo, updates it with changes from your plot, and commits the changes.

### `--dir <target>`
Passing this argument will change the directory within the repo that dfgit uses. It overrides the `template_directory` option in `.dfgit`.




## Requirements

- You must have the [CodeClient](https://github.com/DFOnline/CodeClient) mod installed.
- You must have [git](https://git-scm.com) installed.

## `.dfgit` config file
The `.dfgit` file contains settings for dfgit, and should be found in the root directory of a repo, however dfgit does not require the existance of the file. It uses the yaml format.





### Example config file
```yaml
template_directory: templates
```

### Options

#### `template_directory`
Specifies the directory that dfgit will look for and create templates in (relative to the repo root). Defaults to `.`. Overriden by the `--dir` flag.




## More Info

dfgit stores templates in `.df` files. The files contain the raw json template data. Filenames are generated from the block type and event/function name. You should avoid manually creating `.df` files, as they will all be read but may not be written to if the name doesen't match. Different formatting may also cause issues.
