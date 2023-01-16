cargo build --release
rsync -arv target/release/sync-multithreaded dlencina@35.184.38.124:~
rsync -arv target/release/async-multithreaded dlencina@35.184.38.124:~
rsync -arv target/release/sync-singlethreaded dlencina@35.184.38.124:~
rsync -arv target/release/async-singlethreaded dlencina@35.184.38.124:~
