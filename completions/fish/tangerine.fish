# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_tangerine_global_optspecs
	string join \n v/verbose q/quiet o/output= i/in-place h/help V/version
end

function __fish_tangerine_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_tangerine_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_tangerine_using_subcommand
	set -l cmd (__fish_tangerine_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c tangerine -n "__fish_tangerine_needs_command" -s o -l output -r
complete -c tangerine -n "__fish_tangerine_needs_command" -s v -l verbose -d 'Increase logging verbosity'
complete -c tangerine -n "__fish_tangerine_needs_command" -s q -l quiet -d 'Decrease logging verbosity'
complete -c tangerine -n "__fish_tangerine_needs_command" -s i -l in-place
complete -c tangerine -n "__fish_tangerine_needs_command" -s h -l help -d 'Print help'
complete -c tangerine -n "__fish_tangerine_needs_command" -s V -l version -d 'Print version'
complete -c tangerine -n "__fish_tangerine_needs_command" -a "completion"
complete -c tangerine -n "__fish_tangerine_needs_command" -a "gen-markdown"
complete -c tangerine -n "__fish_tangerine_needs_command" -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c tangerine -n "__fish_tangerine_using_subcommand completion" -s v -l verbose -d 'Increase logging verbosity'
complete -c tangerine -n "__fish_tangerine_using_subcommand completion" -s q -l quiet -d 'Decrease logging verbosity'
complete -c tangerine -n "__fish_tangerine_using_subcommand completion" -s h -l help -d 'Print help'
complete -c tangerine -n "__fish_tangerine_using_subcommand gen-markdown" -s v -l verbose -d 'Increase logging verbosity'
complete -c tangerine -n "__fish_tangerine_using_subcommand gen-markdown" -s q -l quiet -d 'Decrease logging verbosity'
complete -c tangerine -n "__fish_tangerine_using_subcommand gen-markdown" -s h -l help -d 'Print help'
complete -c tangerine -n "__fish_tangerine_using_subcommand help; and not __fish_seen_subcommand_from completion gen-markdown help" -f -a "completion"
complete -c tangerine -n "__fish_tangerine_using_subcommand help; and not __fish_seen_subcommand_from completion gen-markdown help" -f -a "gen-markdown"
complete -c tangerine -n "__fish_tangerine_using_subcommand help; and not __fish_seen_subcommand_from completion gen-markdown help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
