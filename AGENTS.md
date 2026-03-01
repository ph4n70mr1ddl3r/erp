# AGENTS.md

Instructions for AI coding assistants working on this codebase.

## Project Overview

Full-stack ERP system:
- **Backend**: Rust (Axum framework, SQLite via sqlx)
- **Frontend**: React 19 + TypeScript + Vite + Tailwind CSS
- **Auth**: JWT-based authentication
- **Architecture**: Modular workspace with 100+ Rust crates

## Build, Lint, and Test Commands

### Backend (Rust)
```bash
cargo build --release          # Build everything
cargo check                    # Check compilation (fast)
cargo test                     # Run all tests
cargo test -p erp-auth         # Run tests for a specific crate
cargo test test_hash_password  # Run a single test by name
cargo test -p erp-api test_create_product_with_auth  # Single test in crate
cargo test --test integration_test  # Run integration tests only
cargo clippy -- -D warnings    # Lint
cargo fmt                      # Format
```

### Frontend (TypeScript/React)
```bash
cd frontend
npm run dev           # Development server
npm run build         # Build for production (includes type check)
npm run lint          # Lint
npx eslint src/pages/Sales.tsx  # Lint specific file
```

### Full Stack
```bash
cargo run --release   # Run backend (port 3000)
cd frontend && npm run dev  # Run frontend (port 5173)
```

## Rust Backend Conventions

### Module Structure
```
erp-<name>/
├── src/lib.rs        # Exports
├── src/models.rs     # Domain types
├── src/repository.rs # Database operations
└── src/service.rs    # Business logic
```

### Import Order
```rust
// 1. Standard library
use std::collections::HashMap;
// 2. External crates (alphabetical)
use anyhow::Result;
use axum::{extract::State, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
// 3. Internal crates
use erp_core::{BaseEntity, Error, Money, Pagination, Status};
// 4. Current crate modules
use crate::models::*;
```

### Naming Conventions
- **Types**: PascalCase (`Account`, `JournalEntry`)
- **Functions/Variables**: snake_case (`get_account`, `account_list`)
- **Constants**: SCREAMING_SNAKE_CASE (`MAX_FILE_SIZE`)

### Error Handling
```rust
pub async fn get_account(pool: &SqlitePool, id: Uuid) -> Result<Account> {
    let row = sqlx::query_as::<_, AccountRow>(sql)
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("Account", &id.to_string()))?;
    Ok(row.into())
}
// Validation: Error::validation("Code is required")
// Business rules: Error::business_rule("Balance cannot be negative")
```

### Handler Pattern
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

### Key Patterns
- **BaseEntity**: All entities include `base: BaseEntity` for id/timestamps
- **Money**: Stored as cents (i64): `Money::new(10000, Currency::USD)` = $100.00
- **Status enum**: Active, Inactive, Draft, Pending, Approved, etc.
- **Service layer**: All validation in services, handlers stay thin
- **UUIDs**: Stored as TEXT in SQLite, parsed to Uuid type in Rust

## Frontend (TypeScript/React) Conventions

### Import Order
```typescript
// 1. React/external
import { useEffect, useState } from 'react';
import { useToast } from '../components/Toast';
// 2. API client
import { sales } from '../api/client';
// 3. Types
import type { Customer } from '../types';
// 4. Utilities
import { getErrorMessage } from '../utils/errors';
```

### Error Handling (CRITICAL)
```typescript
// ALWAYS use unknown in catch blocks - NEVER use any
try {
  await sales.createCustomer(data);
} catch (err: unknown) {
  toast.error(getErrorMessage(err, 'Failed to create customer'));
}
```

### Naming Conventions
- **Components/Types**: PascalCase (`SalesPage`, `CustomerResponse`)
- **Functions/Variables**: camelCase (`handleCreate`, `customers`)
- **Files**: PascalCase for components (`Sales.tsx`), camelCase for utils (`errors.ts`)

### Component Pattern
```typescript
export default function Page() {
  const toast = useToast();
  const [loading, setLoading] = useState(true);
  const [data, setData] = useState<Thing[]>([]);

  useEffect(() => { loadData(); }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      const res = await api.getThings(1, 50);
      setData(res.data.items);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load'));
    } finally {
      setLoading(false);
    }
  };

  if (loading) return <LoadingPage />;
  return <div>...</div>;
}
```

## Adding New Features

### New Backend Module
1. Create `erp-<name>/` with lib.rs, models.rs, repository.rs, service.rs
2. Add to workspace in root `Cargo.toml`
3. Create migration: `migrations/YYYYMMDDHHMMSS_<name>.sql`
4. Add handlers in `erp-api/src/handlers/<name>.rs`
5. Add routes in `erp-api/src/routes.rs`

### New Frontend Page
1. Create `frontend/src/pages/PageName.tsx`
2. Add route in `frontend/src/App.tsx`
3. Add nav link in `frontend/src/components/Layout.tsx`
4. Add API methods in `frontend/src/api/client.ts`

## Testing

### Backend Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_name() {
        let result = function_under_test();
        assert!(result.is_ok());
    }
}
```

### Integration Tests
Located in `erp-api/tests/integration_test.rs`. Uses in-memory SQLite.

## Important Notes

- Never commit `.env` or `erp.db` files
- All prices stored as cents (multiply by 100 when creating, divide by 100 when displaying)
- Use `?` operator extensively for error propagation
- Handlers should be thin - business logic goes in services
- Always validate input in service layer, not handlers
- Use `tracing` for logging: `tracing::info!`, `tracing::error!`
- Frontend: Never use `any` type - always use `unknown` in catch blocks
