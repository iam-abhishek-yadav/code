"""
Database connection configuration.
This module sets up the SQLAlchemy engine and session factory for PostgreSQL database.
"""

from sqlalchemy.orm import sessionmaker
from sqlalchemy import create_engine

# PostgreSQL database connection URL
# Format: postgresql://username:password@host:port/database_name
db_url="postgresql://postgres:12345678@localhost:5432/fastapi"

# Create SQLAlchemy engine to manage database connections
engine = create_engine(db_url)

# Create session factory for database sessions
# autocommit=False: Changes must be explicitly committed
# autoflush=False: Changes are not automatically flushed to the database
session = sessionmaker(autocommit=False, autoflush=False, bind=engine)