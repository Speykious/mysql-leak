#!/bin/bash

sqlargs="--user=root --password=$password --host=127.0.0.1 --port=$port --database=mysqlleak_demo"
infodir="$1"

if [ -z "$infodir" ]; then
    echo "usage: $0 <info dir>"
    exit 1
fi

mkdir -p "$infodir"

mysql $sqlargs -e 'SELECT COUNT(*) FROM information_schema.tables;' > "$infodir/count-infoschema-tables.txt"
mysql $sqlargs -e 'SHOW GLOBAL STATUS;'                             > "$infodir/global-status.txt"
mysql $sqlargs -e 'SHOW GLOBAL VARIABLES;'                          > "$infodir/global-variables.txt"
mysql $sqlargs -e 'SHOW FULL PROCESSLIST;'                          > "$infodir/full-processlist.txt"
mysql $sqlargs -e 'STATUS;'                                         > "$infodir/just-status.txt"
mysql $sqlargs -e 'SHOW ENGINE INNODB STATUS\G'                     > "$infodir/innodb-engine-status.txt"
