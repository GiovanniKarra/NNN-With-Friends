! test -f data.db && touch data.db

cat down.sql | sqlite3 data.db
cat up.sql | sqlite3 data.db