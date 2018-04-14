function bb () {
  result=$(./target/release/bb $*)
  status=$?

  if [ $status = 0 ]; then
    cd $result
  else
    echo err
  fi
}