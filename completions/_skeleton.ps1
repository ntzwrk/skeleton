
@('skeleton', './skeleton') | %{
    Register-ArgumentCompleter -Native -CommandName $_ -ScriptBlock {
        param($wordToComplete, $commandAst, $cursorPosition)

        $command = '_skeleton'
        $commandAst.CommandElements |
            Select-Object -Skip 1 |
            %{
                switch ($_.ToString()) {

                    'new' {
                        $command += '_new'
                        break
                    }

                    'init' {
                        $command += '_init'
                        break
                    }

                    'help' {
                        $command += '_help'
                        break
                    }

                    default { 
                        break
                    }
                }
            }

        $completions = @()

        switch ($command) {

            '_skeleton' {
                $completions = @('new', 'init', 'help', '-h', '-V', '-l', '--help', '--version', '--lang')
            }

            '_skeleton_new' {
                $completions = @('-h', '-V', '--help', '--version')
            }

            '_skeleton_init' {
                $completions = @('-h', '-V', '--help', '--version')
            }

            '_skeleton_help' {
                $completions = @('-h', '-V', '--help', '--version')
            }

        }

        $completions |
            ?{ $_ -like "$wordToComplete*" } |
            Sort-Object |
            %{ New-Object System.Management.Automation.CompletionResult $_, $_, 'ParameterValue', $_ }
    }
}
