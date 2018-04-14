function bb 
  set -l result (/Users/nju33/.cargo/bin/bb $argv)

  if [ $status = 0 ]
    cd $result
  else
    echo err
  end
end

