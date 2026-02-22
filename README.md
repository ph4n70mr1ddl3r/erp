# ERP System

A full-featured Enterprise Resource Planning system built with Rust and React.

## Features

### Backend (Rust + Axum)
- **Finance Module**: Chart of accounts, journal entries, fiscal years, double-entry validation
- **Inventory Module**: Products, warehouses, stock movements, stock tracking
- **Sales Module**: Customers, sales orders, order confirmation workflow
- **Purchasing Module**: Vendors, purchase orders, approval workflow
- **Manufacturing Module**: Bills of materials, work orders, production workflow
- **HR Module**: Employees, attendance, payroll
- **Authentication**: JWT-based auth with role-based access control

### Frontend (React + TypeScript)
- Modern responsive UI with Tailwind CSS
- Login/Register with JWT
- Dashboard with statistics
- Full CRUD for all modules
- Workflow actions (confirm, approve, post)

## Quick Start

### Option 1: Docker (Recommended)

```bash
# Build and run everything
docker-compose up --build

# Access the application at http://localhost
```

For production, set environment variables:
```bash
export JWT_SECRET=your-secure-secret-key
docker-compose up -d
```

### Option 2: Manual Setup

### Prerequisites
- Rust 1.70+
- Node.js 18+
- SQLite 3

### Backend

```bash
# Set environment variables
export DATABASE_URL=sqlite:erp.db?mode=rwc
export JWT_SECRET=your-secret-key-change-in-production
export SERVER_HOST=127.0.0.1
export SERVER_PORT=3000

# Build and run
cargo build --release
./target/release/erp-server
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

Access the application at http://localhost:5173

## API Endpoints

### Authentication
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/auth/register` | Register new user |
| POST | `/auth/login` | Login, returns JWT |
| GET | `/auth/me` | Get current user (requires auth) |

### Finance
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET/POST | `/api/v1/finance/accounts` | List/create accounts |
| GET/PUT/DELETE | `/api/v1/finance/accounts/:id` | CRUD account |
| GET/POST | `/api/v1/finance/journal-entries` | List/create entries |
| POST | `/api/v1/finance/journal-entries/:id/post` | Post entry |
| GET/POST | `/api/v1/finance/fiscal-years` | List/create fiscal years |

### Inventory
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET/POST | `/api/v1/inventory/products` | List/create products |
| GET/PUT/DELETE | `/api/v1/inventory/products/:id` | CRUD product |
| GET/POST | `/api/v1/inventory/warehouses` | List/create warehouses |
| POST | `/api/v1/inventory/stock-movements` | Record stock movement |
| GET | `/api/v1/inventory/stock/:product_id` | Get stock levels |

### Sales
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET/POST | `/api/v1/sales/customers` | List/create customers |
| GET/POST | `/api/v1/sales/orders` | List/create orders |
| POST | `/api/v1/sales/orders/:id/confirm` | Confirm order |

### Purchasing
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET/POST | `/api/v1/purchasing/vendors` | List/create vendors |
| GET/POST | `/api/v1/purchasing/orders` | List/create POs |
| POST | `/api/v1/purchasing/orders/:id/approve` | Approve PO |

### Manufacturing
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET/POST | `/api/v1/manufacturing/boms` | List/create BOMs |
| GET/POST | `/api/v1/manufacturing/work-orders` | List/create work orders |
| POST | `/api/v1/manufacturing/work-orders/:id/start` | Start production |
| POST | `/api/v1/manufacturing/work-orders/:id/complete` | Complete production |

### HR
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET/POST | `/api/v1/hr/employees` | List/create employees |
| POST | `/api/v1/hr/attendance/check-in` | Check in |
| POST | `/api/v1/hr/attendance/check-out` | Check out |
| GET/POST | `/api/v1/hr/payroll` | List/create payroll |

## Architecture

```
erp/
├── erp-core/           # Shared types, errors, database utilities
├── erp-finance/        # Finance domain logic
├── erp-inventory/      # Inventory domain logic
├── erp-sales/          # Sales domain logic
├── erp-purchasing/     # Purchasing domain logic
├── erp-manufacturing/  # Manufacturing domain logic
├── erp-hr/             # HR domain logic
├── erp-auth/           # Authentication and authorization
├── erp-api/            # REST API server (Axum)
├── frontend/           # React frontend
└── migrations/         # SQLite migrations
```

### Module Structure

Each business module follows this pattern:

```
erp-module/
├── src/
│   ├── lib.rs          # Module exports
│   ├── models.rs       # Domain models
│   ├── repository.rs   # Database operations (traits + SQLite impl)
│   └── service.rs      # Business logic and validation
└── Cargo.toml
```

## Authentication

### Roles
| Role | Permissions |
|------|-------------|
| Admin | Full access to all modules |
| Finance | Finance, Sales (read), Purchasing (read) |
| Warehouse | Inventory, Purchasing, Manufacturing (read) |
| Sales | Sales, Inventory (read) |
| HR | HR module only |
| User | Read-only access |

### Using the API

```bash
# Register
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","email":"admin@example.com","password":"password","full_name":"Admin"}'

# Login
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"password"}'

# Use token
curl http://localhost:3000/api/v1/inventory/products \
  -H "Authorization: Bearer <token>"
```

## Database Schema

The system uses SQLite with the following main tables:

- `users`, `roles` - Authentication
- `accounts`, `journal_entries`, `journal_lines`, `fiscal_years` - Finance
- `products`, `warehouses`, `stock_levels`, `stock_movements` - Inventory
- `customers`, `sales_orders`, `sales_order_lines`, `invoices` - Sales
- `vendors`, `purchase_orders`, `purchase_order_lines` - Purchasing
- `bills_of_material`, `bom_components`, `work_orders` - Manufacturing
- `employees`, `attendance`, `payroll` - HR

## Development

### Running Tests
```bash
# Run all tests (unit + integration)
cargo test

# Run specific test suite
cargo test -p erp-api --test integration_test
cargo test -p erp-auth --lib
```

### Building for Production
```bash
# Backend
cargo build --release

# Frontend
cd frontend && npm run build
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| DATABASE_URL | sqlite:erp.db?mode=rwc | SQLite database URL |
| SERVER_HOST | 127.0.0.1 | Server bind host |
| SERVER_PORT | 3000 | Server bind port |
| JWT_SECRET | (required) | JWT signing secret |
| RUST_LOG | info | Logging level |

## License

MIT
