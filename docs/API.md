# API Documentation

Base URL: `http://localhost:3000`

## Authentication

All `/api/v1/*` endpoints require JWT authentication. Include the token in the Authorization header:

```
Authorization: Bearer <token>
```

---

## Auth Endpoints

### Register User
```http
POST /auth/register
Content-Type: application/json

{
  "username": "string",
  "email": "string",
  "password": "string",
  "full_name": "string"
}
```

**Response 200:**
```json
{
  "token": "eyJ0eXAi...",
  "expires_at": "2024-01-02T00:00:00Z",
  "user": {
    "id": "uuid",
    "username": "string",
    "email": "string",
    "full_name": "string",
    "role": "User"
  }
}
```

### Login
```http
POST /auth/login
Content-Type: application/json

{
  "username": "string",
  "password": "string"
}
```

**Response 200:** Same as register

**Response 401:**
```json
{
  "error": "Unauthorized"
}
```

### Get Current User
```http
GET /auth/me
Authorization: Bearer <token>
```

**Response 200:**
```json
{
  "id": "uuid",
  "username": "string",
  "email": "string",
  "full_name": "string",
  "role": "string"
}
```

---

## Finance Endpoints

### List Accounts
```http
GET /api/v1/finance/accounts?page=1&per_page=20
Authorization: Bearer <token>
```

**Response 200:**
```json
{
  "items": [
    {
      "id": "uuid",
      "code": "1000",
      "name": "Cash",
      "account_type": "Asset",
      "parent_id": null,
      "status": "Active",
      "description": null,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "total": 1,
  "page": 1,
  "per_page": 20,
  "total_pages": 1
}
```

### Create Account
```http
POST /api/v1/finance/accounts
Authorization: Bearer <token>
Content-Type: application/json

{
  "code": "1000",
  "name": "Cash",
  "account_type": "Asset",
  "parent_id": null,
  "description": "Main cash account"
}
```

**Account Types:** `Asset`, `Liability`, `Equity`, `Revenue`, `Expense`

### Create Journal Entry
```http
POST /api/v1/finance/journal-entries
Authorization: Bearer <token>
Content-Type: application/json

{
  "description": "Sales for cash",
  "reference": "INV-001",
  "lines": [
    {
      "account_id": "uuid",
      "debit": 10000,
      "credit": 0,
      "description": "Cash received"
    },
    {
      "account_id": "uuid",
      "debit": 0,
      "credit": 10000,
      "description": "Sales revenue"
    }
  ]
}
```

**Note:** Amounts are in cents. Debits must equal credits.

### Post Journal Entry
```http
POST /api/v1/finance/journal-entries/{id}/post
Authorization: Bearer <token>
```

**Response 200:**
```json
{
  "status": "posted"
}
```

---

## Inventory Endpoints

### List Products
```http
GET /api/v1/inventory/products?page=1&per_page=20
Authorization: Bearer <token>
```

### Create Product
```http
POST /api/v1/inventory/products
Authorization: Bearer <token>
Content-Type: application/json

{
  "sku": "WIDGET-001",
  "name": "Widget",
  "description": "A widget",
  "product_type": "Goods",
  "unit_of_measure": "PCS"
}
```

**Product Types:** `Goods`, `Service`, `Digital`

### Create Stock Movement
```http
POST /api/v1/inventory/stock-movements
Authorization: Bearer <token>
Content-Type: application/json

{
  "product_id": "uuid",
  "to_location_id": "uuid",
  "quantity": 100,
  "movement_type": "Receipt"
}
```

**Movement Types:** `Receipt`, `Issue`, `Transfer`, `Adjustment`

### Get Stock Levels
```http
GET /api/v1/inventory/stock/{product_id}
Authorization: Bearer <token>
```

**Response 200:**
```json
[
  {
    "product_id": "uuid",
    "location_id": "uuid",
    "quantity": 100,
    "reserved_quantity": 0,
    "available_quantity": 100
  }
]
```

---

## Sales Endpoints

### Create Customer
```http
POST /api/v1/sales/customers
Authorization: Bearer <token>
Content-Type: application/json

{
  "code": "C001",
  "name": "Acme Corp",
  "email": "acme@example.com",
  "phone": "+1234567890"
}
```

### Create Sales Order
```http
POST /api/v1/sales/orders
Authorization: Bearer <token>
Content-Type: application/json

{
  "customer_id": "uuid",
  "lines": [
    {
      "product_id": "uuid",
      "description": "Widget",
      "quantity": 10,
      "unit_price": 5000
    }
  ]
}
```

**Note:** `unit_price` is in cents (5000 = $50.00)

### Confirm Order
```http
POST /api/v1/sales/orders/{id}/confirm
Authorization: Bearer <token>
```

---

## Purchasing Endpoints

### Create Vendor
```http
POST /api/v1/purchasing/vendors
Authorization: Bearer <token>
Content-Type: application/json

{
  "code": "V001",
  "name": "Supply Co",
  "email": "supply@example.com"
}
```

### Create Purchase Order
```http
POST /api/v1/purchasing/orders
Authorization: Bearer <token>
Content-Type: application/json

{
  "vendor_id": "uuid",
  "lines": [
    {
      "product_id": "uuid",
      "description": "Raw material",
      "quantity": 100,
      "unit_price": 1000
    }
  ]
}
```

### Approve Purchase Order
```http
POST /api/v1/purchasing/orders/{id}/approve
Authorization: Bearer <token>
```

---

## Manufacturing Endpoints

### Create BOM
```http
POST /api/v1/manufacturing/boms
Authorization: Bearer <token>
Content-Type: application/json

{
  "product_id": "uuid",
  "name": "Assembly BOM",
  "quantity": 1,
  "components": [
    {
      "product_id": "uuid",
      "quantity": 2,
      "unit": "PCS"
    }
  ]
}
```

### Create Work Order
```http
POST /api/v1/manufacturing/work-orders
Authorization: Bearer <token>
Content-Type: application/json

{
  "product_id": "uuid",
  "bom_id": "uuid",
  "quantity": 50,
  "planned_start": "2024-02-01T08:00:00Z",
  "planned_end": "2024-02-01T17:00:00Z"
}
```

### Start Work Order
```http
POST /api/v1/manufacturing/work-orders/{id}/start
Authorization: Bearer <token>
```

### Complete Work Order
```http
POST /api/v1/manufacturing/work-orders/{id}/complete
Authorization: Bearer <token>
```

---

## HR Endpoints

### Create Employee
```http
POST /api/v1/hr/employees
Authorization: Bearer <token>
Content-Type: application/json

{
  "employee_number": "E001",
  "first_name": "John",
  "last_name": "Doe",
  "email": "john@example.com",
  "hire_date": "2024-01-15"
}
```

### Check In
```http
POST /api/v1/hr/attendance/check-in
Authorization: Bearer <token>
Content-Type: application/json

{
  "employee_id": "uuid"
}
```

### Check Out
```http
POST /api/v1/hr/attendance/check-out
Authorization: Bearer <token>
Content-Type: application/json

{
  "employee_id": "uuid"
}
```

### Create Payroll
```http
POST /api/v1/hr/payroll
Authorization: Bearer <token>
Content-Type: application/json

{
  "employee_id": "uuid",
  "period_start": "2024-02-01",
  "period_end": "2024-02-29",
  "base_salary": 500000,
  "overtime": 50000,
  "bonuses": 0,
  "deductions": 25000
}
```

**Note:** All amounts in cents

---

## Error Responses

All errors follow this format:
```json
{
  "error": "Error message"
}
```

Common HTTP status codes:
- `400` - Validation error
- `401` - Unauthorized (invalid/missing token)
- `404` - Resource not found
- `409` - Conflict (duplicate key)
- `422` - Business rule violation
- `500` - Internal server error

---

## Pagination

List endpoints support pagination via query parameters:
- `page` - Page number (default: 1)
- `per_page` - Items per page (default: 20)

Response includes:
```json
{
  "items": [...],
  "total": 100,
  "page": 1,
  "per_page": 20,
  "total_pages": 5
}
```

---

## Money Format

All monetary amounts are stored and transmitted as integers representing cents:
- $50.00 → `5000`
- $1,234.56 → `123456`

---

## Status Values

Common status values across entities:
- `Active` - Normal operational state
- `Inactive` - Disabled but not deleted
- `Draft` - Created but not submitted
- `Pending` - Awaiting approval
- `Approved` - Approved for processing
- `Completed` - Fully processed
- `Cancelled` - Cancelled
