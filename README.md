# gcl - git clone

I really like the project path structure from the "old" [go workspaces](https://golang.org/doc/gopath_code#Workspaces). For example, the command `go get github.com/golang/protobuf/` is cloning the package into $GOPATH/src/github.com/golang/protobuf.

I use this structure for every project I clone, so I had to navigate into the given base folder and create all underlying folders which was quite a pain. That's why I created this tiny tool which helps to keep the structure without the need for multiple commands.

Example: `gcl https://github.com/<user>/<project>`
You can either set a basepath via the gcl environment variable or the curent working directory will be used.

gcl is creating a `<basePath>/github.com/<user>/<project>` folder structure were the project will be cloned into.

I also love to have subfolders for each language. That's why I implemented the `-p <name>` parameter which is creating a new <name> subfolder.

Example: `gcl -p rust https://github.com/<user>/<project>`
-> `<basePath>/rust/github.com/<user>/<project>`

# Requirements:
- git (I could also use https://github.com/rust-lang/git2-rs which would resolve this)
- set gcl environment variable if you don't want to use the current working directory

# Installation:
- clone the repo
- cargo install gcl
