function __fish_using_command
    set cmd (commandline -opc)
    if [ (count $cmd) -eq (count $argv) ]
        for i in (seq (count $argv))
            if [ $cmd[$i] != $argv[$i] ]
                return 1
            end
        end
        return 0
    end
    return 1
end

complete -c skeleton -n "__fish_using_command skeleton" -s l -l lang -d "Set language configuration"
complete -c skeleton -n "__fish_using_command skeleton" -s h -l help -d "Prints help information"
complete -c skeleton -n "__fish_using_command skeleton" -s V -l version -d "Prints version information"
complete -c skeleton -n "__fish_using_command skeleton" -f -a "new" -d "create new project"
complete -c skeleton -n "__fish_using_command skeleton" -f -a "init" -d "initialize existing project"
complete -c skeleton -n "__fish_using_command skeleton" -f -a "help" -d "Prints this message or the help of the given subcommand(s)"
complete -c skeleton -n "__fish_using_command skeleton new" -s h -l help -d "Prints help information"
complete -c skeleton -n "__fish_using_command skeleton new" -s V -l version -d "Prints version information"
complete -c skeleton -n "__fish_using_command skeleton init" -s h -l help -d "Prints help information"
complete -c skeleton -n "__fish_using_command skeleton init" -s V -l version -d "Prints version information"
complete -c skeleton -n "__fish_using_command skeleton help" -s h -l help -d "Prints help information"
complete -c skeleton -n "__fish_using_command skeleton help" -s V -l version -d "Prints version information"
