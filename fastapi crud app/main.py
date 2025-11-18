"""
FastAPI application with CRUD operations for products.
This module defines the API endpoints and business logic for product management.
"""

from fastapi import FastAPI, Depends
from models import Product
from database import session, engine
import database_models
from sqlalchemy.orm import Session

# Initialize FastAPI application
app = FastAPI()

# Create all database tables if they don't exist
database_models.Base.metadata.create_all(bind=engine)

# Sample products data for initial database seeding
products = [
    Product(id=1, name="Laptop", description="14-inch display, 8GB RAM, 512GB SSD", price=75000, quantity=10),
    Product(id=2, name="Smartphone", description="6.5-inch AMOLED, 128GB storage", price=35000, quantity=25),
    Product(id=3, name="Headphones", description="Noise-cancelling over-ear headphones", price=8000, quantity=40),
    Product(id=4, name="Smartwatch", description="Water-resistant with heart rate monitor", price=12000, quantity=18),
]

def get_db():
    """
    Dependency function to get database session.
    
    Creates a database session and ensures it's properly closed after use.
    This is used as a dependency in route handlers via Depends().
    
    Yields:
        Session: SQLAlchemy database session
    """
    db = session()
    try:
        yield db
    finally:
        db.close()

def init_db():
    """
    Initialize database with sample products if the database is empty.
    
    Checks if any products exist in the database, and if not, 
    seeds it with the sample products data.
    """
    db = session()

    # Count existing products in the database
    count = db.query(database_models.Product).count

    # If database is empty, add sample products
    if count == 0:
        for product in products:
            db.add(database_models.Product(**product.model_dump()))
        db.commit()

# Initialize database on application startup
init_db()


@app.get("/")
def greet():
    """
    Root endpoint that returns a welcome message.
    
    Returns:
        str: Welcome message
    """
    return ("Welcome to fastapi server")

@app.get("/products")
def get_all_products(db: Session = Depends(get_db)):
    """
    Get all products from the database.
    
    Args:
        db: Database session (injected via dependency)
    
    Returns:
        List[Product]: List of all products in the database
    """
    db_products = db.query(database_models.Product).all()
    return db_products

@app.get("/product/{id}")
def get_product_by_id(id: int, db: Session = Depends(get_db)):
    """
    Get a specific product by its ID.
    
    Args:
        id: Product ID to search for
        db: Database session (injected via dependency)
    
    Returns:
        Product: Product object if found, otherwise error message
    """
    db_product = db.query(database_models.Product).filter(database_models.Product.id == id).first()
    if db_product:
        return db_product
    return "product not found"

@app.post("/product")
def add_product(product: Product, db: Session = Depends(get_db)):
    """
    Add a new product to the database.
    
    Args:
        product: Product data from request body (validated by Pydantic)
        db: Database session (injected via dependency)
    
    Returns:
        Product: The created product
    """
    db.add(database_models.Product(**product.model_dump()))
    db.commit()
    return product

@app.put("/product/{id}")
def update_product(id: int, product: Product, db: Session = Depends(get_db)):
    """
    Update an existing product by ID.
    
    Args:
        id: Product ID to update
        product: Updated product data from request body
        db: Database session (injected via dependency)
    
    Returns:
        str: Success or failure message
    """
    db_product = db.query(database_models.Product).filter(database_models.Product.id == id).first()
    if db_product: 
        # Update product fields
        db_product.name = product.name
        db_product.description = product.description
        db_product.price = product.price
        db_product.quantity = product.quantity
        db.commit()
        return "Product Update Successful"
    else: 
        return "Product Update Failed"

@app.delete("/product/{id}")
def delete_product(id: int, db: Session = Depends(get_db)):
    """
    Delete a product from the database by ID.
    
    Args:
        id: Product ID to delete
        db: Database session (injected via dependency)
    
    Returns:
        str: Success or failure message
    """
    db_product = db.query(database_models.Product).filter(database_models.Product.id == id).first()
    if db_product: 
        db.delete(db_product)
        db.commit()
        return "Product Deletion Successful"
    else: 
        return "Product Deletion Failed"