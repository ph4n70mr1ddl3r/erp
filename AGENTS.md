# AGENTS.md

Instructions for AI assistants working on this codebase.

## Project Overview

This is a full-stack ERP system:
- **Backend**: Rust with Axum web framework, SQLite database
- **Frontend**: React + TypeScript with Vite
- **Auth**: JWT-based authentication

## Code Conventions

### Rust Backend

#### Module Pattern
Each business module (erp-finance, erp-inventory, etc.) follows this structure:

```rust
// src/lib.rs
pub mod models;
pub mod repository;
pub mod service;
pub use models::*;
pub use service::*;

// src/models.rs - Domain types
pub struct Account { pub base: BaseEntity, pub code: String, ... }

// src/repository.rs - Database operations
pub struct SqliteAccountRepository;
impl AccountRepository for SqliteAccountRepository { ... }

// src/service.rs - Business logic
pub struct AccountService { repo: SqliteAccountRepository }
```

#### Key Patterns

1. **BaseEntity**: All entities have a `base: BaseEntity` field with id, timestamps
2. **Money type**: Stored as cents (i64), use `Money::new(cents, Currency::USD)`
3. **Status enum**: Active, Inactive, Draft, Pending, Approved, etc.
4. **Repository trait + impl**: Separate trait definition from SQLite implementation
5. **Service layer**: All validation happens here, not in handlers

#### Database

- SQLite with sqlx
- Migrations in `migrations/` directory
- Auto-run on startup (see `erp-api/src/db.rs`)
- All IDs are UUIDs stored as TEXT

#### Adding a New Module

1. Create crate: `erp-<name>/`
2. Add to workspace in root `Cargo.toml`
3. Create models, repository, service following pattern
4. Add migration file: `migrations/YYYYMMDDHHMMSS_<name>.sql`
5. Add handlers in `erp-api/src/handlers/<name>.rs`
6. Add routes in `erp-api/src/routes.rs`
7. Add to `erp-api/src/db.rs` migrations list

#### Adding an Endpoint

1. Define handler in `erp-api/src/handlers/<module>.rs`:
```rust
pub async fn get_thing(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ThingResponse>> {
    let svc = ThingService::new();
    let thing = svc.get(&state.pool, id).await?;
    Ok(Json(ThingResponse::from(thing)))
}
```

2. Add route in `erp-api/src/routes.rs`:
```rust
.route("/things/:id", get(handlers::module::get_thing))
```

### React Frontend

#### Structure
```
frontend/src/
├── api/client.ts      # Axios API client with auth
├── hooks/useAuth.tsx  # Auth context and hook
├── components/        # Shared components
├── pages/             # Route pages
└── types/index.ts     # TypeScript types
```

#### API Client Usage
```typescript
import { inventory } from '../api/client';

// GET with pagination
const res = await inventory.getProducts(1, 20);
// res.data.items, res.data.total, etc.

// POST
await inventory.createProduct({ sku: 'ABC', name: 'Product', unit_of_measure: 'PCS' });
```

#### Adding a Page

1. Create `src/pages/NewPage.tsx`
2. Add route in `src/App.tsx`:
```tsx
<Route path="/new-page" element={<PrivateRoute><NewPage /></PrivateRoute>} />
```
3. Add nav link in `src/components/Layout.tsx`

## Common Tasks

### Run the Backend
```bash
cargo run --release
```

### Run the Frontend
```bash
cd frontend && npm run dev
```

### Build Everything
```bash
cargo build --release
cd frontend && npm run build
```

### Check for Compilation Errors
```bash
cargo check
cd frontend && npm run build
```

### Add a New Dependency
1. Add to `[workspace.dependencies]` in root `Cargo.toml`
2. Add `dep.workspace = true` in module's `Cargo.toml`

## Testing Authentication

1. Register a user:
```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"test","email":"test@example.com","password":"password","full_name":"Test User"}'
```

2. Use the returned token in subsequent requests:
```bash
curl http://localhost:3000/api/v1/inventory/products \
  -H "Authorization: Bearer <token>"
```

## Important Notes

- Never commit `.env` or `erp.db` files
- All prices stored as cents (multiply by 100 when creating, divide by 100 when displaying)
- Use `?` operator extensively for error propagation
- Handlers should be thin - business logic goes in services
- Use `BaseEntity::new()` for new entities
- UUIDs are stored as strings in SQLite but parsed to Uuid type in Rust
