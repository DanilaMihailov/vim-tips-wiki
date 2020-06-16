function OpenRandomWikiTip()
    let n = system('/bin/bash -c "echo $RANDOM % 1678 + 1 | bc"')
    execute "help vtw-".n
endfunction

command! RandomVimTip call OpenRandomWikiTip()
