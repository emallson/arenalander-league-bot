# delete the tables we don't need. otherwise pgloader can create FKs that
# diesel doesn't agree with
set -e 
set +x

# dependency setup happens in dockerfile because this script is not run with root privileges

# download the data
curl https://www.mtgjson.com/files/AllPrintings.sqlite.gz > /tmp/AllPrintings.sqlite.gz
gunzip /tmp/AllPrintings.sqlite.gz

# prep the data for import
sqlite3 /tmp/AllPrintings.sqlite <<-EOSQL
    drop table set_translations;
    drop table foreign_data;
    drop table legalities;
    drop table rulings;
    drop table prices;
    drop table tokens;
    drop table sets;
EOSQL

# import
pg_ctl -o "-c listen_addresses='localhost'" -w restart
pgloader --cast 'type text to text drop typemod' sqlite:///tmp/AllPrintings.sqlite postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost/$POSTGRES_DB

# cleanup
rm /tmp/AllPrintings.sqlite