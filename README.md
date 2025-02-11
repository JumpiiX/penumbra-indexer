# Penumbra Blockchain Indexer

A real-time blockchain indexer for the Penumbra network that collects raw block data and provides an API for querying the latest blocks.

## Overview

This application connects to a Penumbra node, continuously synchronizes the latest blocks, stores them in a PostgreSQL database, and provides a REST API for querying the data.

## Architecture

### Components

1. **Block Synchronizer**
    - Connects to Penumbra RPC node
    - Fetches latest blocks in real-time
    - Handles reconnection and error recovery
    - Manages block data validation

2. **Database Layer**
    - PostgreSQL database for block storage
    - Maintains latest 10 blocks
    - Automatic cleanup of old blocks
    - Optimized queries for block retrieval

3. **API Server**
    - RESTful endpoints for data access
    - JSON response format
    - CORS support for frontend integration
    - Error handling and status codes

## Technical Stack

- **Language**: Rust
- **Database**: PostgreSQL 15
- **Dependencies**:
    - `tokio` - Async runtime
    - `axum` - Web framework
    - `sqlx` - Database ORM
    - `reqwest` - HTTP client
    - `serde` - Serialization
    - `chrono` - DateTime handling

## Database Schema

```sql
CREATE TABLE blocks (
    height BIGINT PRIMARY KEY,
    time TIMESTAMP WITH TIME ZONE NOT NULL,
    hash TEXT NOT NULL,
    proposer_address TEXT NOT NULL,
    tx_count INTEGER NOT NULL,
    previous_block_hash TEXT,
    data JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

## API Endpoints

### GET /api/blocks
Returns the latest 10 blocks from the chain.

**Response Format**:
```json
{
    "blocks": [
        {
            "height": number,
            "time": string (ISO datetime),
            "hash": string,
            "proposer_address": string,
            "tx_count": number,
            "previous_block_hash": string,
            "data": object,
            "created_at": string (ISO datetime)
        }
    ],
    "total_count": number
}
```

## Setup & Deployment

### Prerequisites
- Docker
- Docker Compose

### Environment Variables
```env
DATABASE_URL=postgres://indexer:indexer@db/indexer
RPC_URL=http://grpc.penumbra.silentvalidator.com:26657
API_PORT=3000
```

### Running the Application

1. Clone the repository:
```bash
git clone https://github.com/JumpiiX/penumbra-indexer
cd penumbra-indexer
```

2. Start the application:
```bash
docker compose up --build
```

The API will be available at `http://localhost:3000/api/blocks`

### Docker Components

1. **Database Container**
    - PostgreSQL 15
    - Persistent volume for data storage
    - Health checks configured

2. **Indexer Container**
    - Rust application
    - Automatic reconnection handling
    - Real-time block synchronization

## Project Structure

```
src/
├── main.rs           # Application entry point
├── client.rs         # Penumbra RPC client
├── db/
│   └── mod.rs       # Database operations
├── api/
│   ├── mod.rs       # API setup
│   └── routes.rs    # API endpoints
└── models/
    └── mod.rs       # Data structures
```

## Error Handling

The application implements comprehensive error handling:
- Database connection errors
- RPC node connectivity issues
- Invalid block data
- API error responses

## Monitoring

The application provides logging for:
- Block synchronization status
- Database operations
- API requests
- Error conditions

## Future Improvements

1. Add block search functionality
2. Implement transaction indexing
3. Add validator statistics
4. Create metrics endpoint
5. Add WebSocket support for real-time updates
