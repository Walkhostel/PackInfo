ABOUT
        Packinfo how info about all installed pacman packeges in the system with their name, description, depends and require by like this:
        ├╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │name  ┆ desc  ┆ depends  ┆ require by   │
        ├╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤

INSTALLATION
        From Cargo (crates.io):
                cargo install packinfo
        From source:
                git clone https://github.com/твойник/packinfo
                cd packinfo
                cargo install --path

HOW TO USE
        "packinfo" for full list
        "packinfo [package name]" for info about specific package
