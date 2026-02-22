# Contributing Guide

Thank you for your interest in contributing to this ERP system!

## Development Setup

1. **Fork and Clone**
```bash
git clone https://github.com/your-username/erp.git
cd erp
```

2. **Install Dependencies**
- Rust 1.70+
- Node.js 18+
- SQLite 3

3. **Run Backend**
```bash
cargo run
```

4. **Run Frontend**
```bash
cd frontend
npm install
npm run dev
```

## Code Style

### Rust
- Use `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Follow standard Rust conventions

### TypeScript/React
- Use TypeScript strict mode
- Follow existing component patterns
- Use Tailwind CSS classes

## Project Structure

```
erp/
├── erp-core/           # Shared types and utilities
├── erp-finance/        # Finance domain
├── erp-inventory/      # Inventory domain
├── erp-sales/          # Sales domain
├── erp-purchasing/     # Purchasing domain
├── erp-manufacturing/  # Manufacturing domain
├── erp-hr/             # HR domain
├── erp-auth/           # Authentication
├── erp-api/            # REST API
├── frontend/           # React frontend
└── migrations/         # Database migrations
```

## Adding a New Module

1. Create new crate: `erp-<name>/`
2. Add to workspace in root `Cargo.toml`
3. Follow the module pattern (models, repository, service)
4. Create database migration
5. Add handlers and routes
6. Update documentation

## Pull Request Process

1. Create a feature branch
2. Make your changes
3. Run tests: `cargo test`
4. Format code: `cargo fmt`
5. Check lints: `cargo clippy`
6. Update documentation if needed
7. Submit PR with description of changes

## Commit Messages

Follow conventional commits:
- `feat: add new feature`
- `fix: fix bug`
- `docs: update documentation`
- `refactor: code cleanup`
- `test: add tests`

## Questions?

Open an issue for discussion before starting major changes.
