set -l rock_cmds install i remove r info if install-info iif search s update u cleanup c --help -h

complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a install -d "Install package"
complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a i -d "Alias: Install package"

complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a remove -d "Remove package & useless dependencies"
complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a r -d "Alias: Remove package & useless dependencies"

complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a info -d "Retrieve info about package"
complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a if -d "Alias: Retrieve info about package"

complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a install-info -d "Display info about installed package"
complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a iif -d "Alias: Display info about installed package"

complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a search -d "Search for package"
complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a s -d "Alias: Search for package"

complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a update -d "Update all packages"
complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a u -d "Alias: Update all packages"

complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a cleanup -d "Remove unused packages"
complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a c -d "Alias: Remove unused packages"

complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a --help -d "Display usage"
complete -f -c rock -n "not __fish_seen_subcommand_from $rock_cmds" -a -h -d "Alias: Display usage"