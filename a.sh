#!/bin/sh

function bb () {
  result=$(./target/release/bb $*)
  status=$?

  if [ $status = 0 ]; then
    echo $result
    cd $result
  else
    echo err
  fi
}

bb $*