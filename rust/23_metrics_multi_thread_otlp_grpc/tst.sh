

for i in {1..40}; do
    if (( RANDOM % 4 == 0 )); then
        curl "http://localhost:8080/d6"
    else
        curl "http://localhost:8080/d8"
    fi
    sleep 0.1
done