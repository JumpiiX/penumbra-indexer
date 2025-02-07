-- Create blocks table with proper indexing and constraints
CREATE TABLE IF NOT EXISTS blocks (
    -- Block identifiers
                                      height BIGINT PRIMARY KEY,
                                      hash TEXT NOT NULL UNIQUE,

    -- Block metadata
                                      timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
                                      num_transactions INTEGER NOT NULL CHECK (num_transactions >= 0),

    -- Full block data
    data JSONB NOT NULL,

    -- Indexer metadata
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                             );

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_blocks_timestamp ON blocks(timestamp);
CREATE INDEX IF NOT EXISTS idx_blocks_created_at ON blocks(created_at);

-- Create index on hash for faster lookups
CREATE INDEX IF NOT EXISTS idx_blocks_hash ON blocks(hash);

-- Add a trigger to automatically update updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_blocks_updated_at
    BEFORE UPDATE ON blocks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Comments for documentation
COMMENT ON TABLE blocks IS 'Stores indexed blocks from Penumbra blockchain';
COMMENT ON COLUMN blocks.height IS 'Block height (unique identifier)';
COMMENT ON COLUMN blocks.hash IS 'Block hash (unique identifier)';
COMMENT ON COLUMN blocks.timestamp IS 'Block timestamp from the blockchain';
COMMENT ON COLUMN blocks.num_transactions IS 'Number of transactions in the block';
COMMENT ON COLUMN blocks.data IS 'Complete block data in JSON format';
COMMENT ON COLUMN blocks.created_at IS 'When this block was first indexed';
COMMENT ON COLUMN blocks.updated_at IS 'When this block was last updated';