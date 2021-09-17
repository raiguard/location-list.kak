decl str loli_cmd

face global loli_highlight "default+r"

# For performance reasons, kakoune does not provide all variables to all shell expansions
# Therefore, we must write the actual commands to $kak_command_fifo so kakoune can see the
# variable names directly, instead of using indirection

def -hidden -params 1 lgnew %{
    nop %sh{
        echo "eval %sh{ \$kak_opt_loli_cmd -t \$kak_timestamp new -t \$kak_timestamp \"\$kak_quoted_opt_$1\" }" > $kak_command_fifo
    }
}

def -hidden -params 1 lcnew %{
    nop %sh{
        echo "eval %sh{ \$kak_opt_loli_cmd -c $kak_client new -t \$kak_timestamp \"\$kak_quoted_opt_$1\" }" > $kak_command_fifo
    }
}

# Delete store file when the session ends
hook global KakEnd .* %{
    nop %sh{ $kak_opt_loli_cmd clean }
}
