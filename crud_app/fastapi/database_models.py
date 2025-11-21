"""
SQLAlchemy ORM models for database tables.
These models define the database schema and table structure.
"""

from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy import Column, Integer, String, Float

# Base class for all ORM models
Base = declarative_base()

class Product(Base):
    """
    Product ORM model representing the 'product' table in the database.
    
    This class maps to the product table and defines the schema structure.
    """
    
    # Table name in the database
    __tablename__ = "product"

    # Primary key column with auto-increment and index
    id = Column(Integer, primary_key=True, index=True)
    
    # Product name column
    name = Column(String)
    
    # Product description column
    description = Column(String)
    
    # Product price column (stored as float)
    price = Column(Float)
    
    # Available quantity in stock
    quantity = Column(Integer)
