#!/bin/bash

watchexec \
  --project-origin .\
  -c -r -p -e rs\
  --shell=bash -- '\
  FILE_PATH="$WATCHEXEC_COMMON_PATH/$WATCHEXEC_WRITTEN_PATH" \
  && PARENT_DIR="$(dirname "$FILE_PATH")" \
  && cd $PARENT_DIR \
  && cargo -Zscript $FILE_PATH'
