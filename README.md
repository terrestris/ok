# ok

Secretly inspired by http://www.secretgeek.net/ok

Also implements a hierarchical structure as outlined in the
comments. So if you have a directory called `$HOME/checkouts/project/module`
with a `.ok` file in both the `project` and the `module` directory,
you'll get a combined list of commands in the `module` directory.

Note that the tool searches for `.ok.fish` files if given an argument
(fish and sh/bash/zsh aren't that compatible). 

## Install

Clone the project, make sure you've got rust installed (rustup is
recommended) run e.g. `cargo build deb` and install the resulting
package.

Then setup an appropriate shell script supplement. For the fish shell:

```fish
function ok
    if test -z $argv[1]
        command ok fish
    else
        set res (command ok fish| grep \^$argv[1] - | sed 's/^[[:digit:]]* - //')
        eval $res
    end
end

function cd
    builtin cd $argv
    ok
end
```

For the bash:
```bash
function ok {
  if (test -z $1) then
    /usr/bin/ok
  else
    $(/usr/bin/ok | grep ^$1 | sed 's/^[[:digit:]]* - //')
  fi
}

function cd {
  builtin cd "$*"
  ok
}
```
