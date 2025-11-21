"""
Pydantic models for request/response validation.
These models define the data structure for API requests and responses.
"""

from pydantic import BaseModel

class Product(BaseModel):
    """
    Product model for API request/response validation.
    
    Attributes:
        id: Unique identifier for the product
        name: Name of the product
        description: Detailed description of the product
        price: Price of the product (float)
        quantity: Available quantity in stock
    """
    id: int
    name: str
    description: str
    price: float
    quantity: int
