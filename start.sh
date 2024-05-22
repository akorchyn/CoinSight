echo "Starting database..."
docker-compose up -d postgres
echo "Waiting for database to start..."
sleep 5
echo "Running migrations..."
(cd backend/db && DATABASE_URL=postgres://cs:cs@localhost:5432/coin_sight diesel migration run)
sleep 5
echo "Starting backend and frontend..."
docker-compose up -d

