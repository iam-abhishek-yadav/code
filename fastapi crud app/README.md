## FastAPI Product Management API

A RESTful API built with FastAPI for managing products with full CRUD (Create, Read, Update, Delete) operations. The application uses PostgreSQL as the database and SQLAlchemy as the ORM.

### Features

- Create, Read, Update, and Delete products
- PostgreSQL database integration
- Automatic database table creation
- Database seeding with sample products
- Request/response validation with Pydantic
- Interactive API documentation (Swagger UI)

### Prerequisites

Before running the application, make sure you have the following installed:

- **Python 3.8+** (Python 3.12 recommended)
- **PostgreSQL** (version 12 or higher)
- **pip** (Python package manager)

### Installation

#### 1. Clone the repository (if applicable)

```bash
cd /home/abhishek/github/code/fastApi
```

#### 2. Create a virtual environment (recommended)

```bash
python3 -m venv venv
```

#### 3. Activate the virtual environment

**On Linux/macOS:**
```bash
source venv/bin/activate
```

**On Windows:**
```bash
venv\Scripts\activate
```

#### 4. Install dependencies

Install all required packages from `requirements.txt`:

```bash
pip install -r requirements.txt
```

Alternatively, install packages individually:
```bash
pip install fastapi uvicorn sqlalchemy psycopg2-binary pydantic
```

### Database Setup

#### 1. Install and start PostgreSQL

Make sure PostgreSQL is installed and running on your system.

**On Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install postgresql postgresql-contrib
sudo systemctl start postgresql
```

**On macOS (using Homebrew):**
```bash
brew install postgresql
brew services start postgresql
```

#### 2. Create the database

Connect to PostgreSQL and create the database:

```bash
sudo -u postgres psql
```

Then in the PostgreSQL prompt:
```sql
CREATE DATABASE fastapi;
\q
```

#### 3. Configure database connection

Update the database connection string in `database.py` if your PostgreSQL credentials differ:

```python
db_url="postgresql://username:password@localhost:5432/fastapi"
```

**Default configuration:**
- Username: `postgres`
- Password: `12345678`
- Host: `localhost`
- Port: `5432`
- Database: `fastapi`

### Running the Application

#### Method 1: Using Uvicorn directly

```bash
uvicorn main:app --reload
```

The `--reload` flag enables auto-reload on code changes (useful for development).

#### Method 2: Using Uvicorn with custom host and port

```bash
uvicorn main:app --host 0.0.0.0 --port 8000 --reload
```

#### Method 3: Using Python module

```bash
python -m uvicorn main:app --reload
```

### Accessing the Application

Once the server is running, you can access:

- **API Base URL:** `http://localhost:8000`
- **Interactive API Documentation (Swagger UI):** `http://localhost:8000/docs`


### Dependencies

- **FastAPI**: Modern, fast web framework for building APIs
- **Uvicorn**: ASGI server for running FastAPI
- **SQLAlchemy**: SQL toolkit and ORM
- **psycopg2**: PostgreSQL adapter for Python
- **Pydantic**: Data validation using Python type annotations