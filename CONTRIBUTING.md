# Contributing to librecraft

librecraft is an ambitious project, and contributions are always welcome!

If you want to work on the codebase, please keep the following in mind:
* Run `cargo fmt` on your code before committing to main.
* Run [`clippy`](https://github.com/rust-lang/rust-clippy) on your code before commiting to main and fix any warnings it gives. Clippy can detect common mistakes.
* Where possible and necessary, please write tests.
* Run cargo test before committing to ensure you have not broken anything (you could change something, and buh you broke vanilla code whatsoever)

## Proper base
When opening a PR, please make sure your branch targets the latest release branch, in this case it would be main. Also make sure your branch is even with the target branch, to avoid unnecessary surprises.

## Proper titles
When opening issues, make sure the title reflects the purpose of the issue or the pull request. Prefer past tense, and be brief. Further description belongs inside the issue or PR.

## Descriptive changes

We require the commits describe the change made. It can be a short description. If you fixed or resolved an open issue, please reference it by using the # notation.

Examples of good commit messages:

    Fixed a potential memory leak with cache entities. Fixes #142.
    Implemented new command extension. Resolves #169.
    Changed message cache behaviour. It's now global instead of per-channel.
    Fixed a potential NRE.

    Changed message cache behaviour:

    - Cache itself is now a ring buffer.

## Code style
You should follow all rust and clippy recommendations, but not necessary.

## Notes to your code
For notes to your code check the Checklist from [`pull_request_template.md`](.github/pull_request_template.md)

## Semantic versioning
When bumping version, you SHOULD follow this: https://semver.org/

## Commits
When committing you SHOULD follow this:
https://www.conventionalcommits.org/ru/v1.0.0-beta.2/

## Comments
You don't need to do ambigious comments, keep it simple.
They SHOULD start from capital letters, although some arguments are exception.
They SHOULD also be on other line, where code is missing.

## Naming Quantities
Variables intended to hold quantities should be written with the _count`/`_amount suffix instead of the num_ prefix. It is for differianting numeric types and amounts.

<table>
<tr>
<th>Countable nouns</th>
<th>Uncountable nouns</th>
<th>Not acceptable</th>
</tr>
<tr>
<td>

let block_count = ...;
</td>
<td>

let block_amount = ...;
</td>
<td>

let num_blocks = ...;
</td>
</tr>
</table>

# Original code (code from Minecraft)

> 🛑 Do not use any of code based of Minecraft's source: 
Please do not write code that is based on, or taken from Mojang's work, including but not limited to the vanilla server and client. Librecraft is afirstly a technical demo showcasing bevy's abilities, meaning that it is written from scratch without any involvement with proprietary code. By using code from Mojang, the project would become prone to legal issues.
This project tries to be on a grey side of legality, when we don't use any assets/code from minecraft, but by being a still valid minecraft clone/client. It will be forever free.
