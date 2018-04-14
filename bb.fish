function bb 
  set -l result (./target/release/bb $argv)

  if [ $status = 0 ]
    cd $result
  else
    echo err
  end
end

bb $argv