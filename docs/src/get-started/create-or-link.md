# Create or Link Project

There are mainly two ways to get started -- either create a new project or link to an existing android project.

## Create

You may create or initialize a project using 

```sh
$ android create
```

And then answer a few prompts:

- Destination path
- Project name
- Package identifier

> The template project that this is based upon lives [here](https://github.com/SyedAhkam/android-cli-template). Open for contributions.

After creating the project, you can start editing the source and run the project.

```sh
$ cd <dest>
$ android run
```

## Link an existing project

If you already have an android project bootstrapped, you will need to link it.

```sh
$ android link
```

> You might need to dig into your own source code to figure out the appropriate prompt values. Especially the package identifier and main activity class name.

All this essentially does is create a [.android](../reference/dot-android.md) file which is then relied upon by the CLI for future operations.

