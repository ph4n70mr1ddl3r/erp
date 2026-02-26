-- Promotions and Coupons Management Module
-- Manages promotional campaigns, discount codes, and coupon management

-- Promotions table
CREATE TABLE IF NOT EXISTS promotions (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    promotion_type TEXT NOT NULL DEFAULT 'OrderDiscount',
    discount_type TEXT NOT NULL DEFAULT 'Percentage',
    discount_value INTEGER NOT NULL DEFAULT 0,
    max_discount INTEGER,
    min_order_amount INTEGER,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    usage_limit INTEGER,
    usage_count INTEGER NOT NULL DEFAULT 0,
    per_customer_limit INTEGER,
    stackable INTEGER NOT NULL DEFAULT 0,
    auto_apply INTEGER NOT NULL DEFAULT 0,
    priority INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_promotions_code ON promotions(code);
CREATE INDEX IF NOT EXISTS idx_promotions_status ON promotions(status);
CREATE INDEX IF NOT EXISTS idx_promotions_dates ON promotions(start_date, end_date);

-- Promotion products (for product-specific promotions)
CREATE TABLE IF NOT EXISTS promotion_products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    promotion_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    include INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (promotion_id) REFERENCES promotions(id) ON DELETE CASCADE,
    UNIQUE(promotion_id, product_id)
);

CREATE INDEX IF NOT EXISTS idx_promotion_products_promotion ON promotion_products(promotion_id);
CREATE INDEX IF NOT EXISTS idx_promotion_products_product ON promotion_products(product_id);

-- Promotion categories (for category-specific promotions)
CREATE TABLE IF NOT EXISTS promotion_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    promotion_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    include INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (promotion_id) REFERENCES promotions(id) ON DELETE CASCADE,
    UNIQUE(promotion_id, category_id)
);

CREATE INDEX IF NOT EXISTS idx_promotion_categories_promotion ON promotion_categories(promotion_id);
CREATE INDEX IF NOT EXISTS idx_promotion_categories_category ON promotion_categories(category_id);

-- Promotion customer groups (for customer segment-specific promotions)
CREATE TABLE IF NOT EXISTS promotion_customer_groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    promotion_id TEXT NOT NULL,
    customer_group_id TEXT NOT NULL,
    FOREIGN KEY (promotion_id) REFERENCES promotions(id) ON DELETE CASCADE,
    UNIQUE(promotion_id, customer_group_id)
);

CREATE INDEX IF NOT EXISTS idx_promotion_customer_groups_promotion ON promotion_customer_groups(promotion_id);

-- Coupons table
CREATE TABLE IF NOT EXISTS coupons (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    promotion_id TEXT NOT NULL,
    coupon_type TEXT NOT NULL DEFAULT 'SingleUse',
    discount_type TEXT NOT NULL DEFAULT 'Percentage',
    discount_value INTEGER NOT NULL DEFAULT 0,
    max_discount INTEGER,
    min_order_amount INTEGER,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    usage_limit INTEGER,
    usage_count INTEGER NOT NULL DEFAULT 0,
    per_customer_limit INTEGER,
    customer_email TEXT,
    first_time_only INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (promotion_id) REFERENCES promotions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_coupons_code ON coupons(code);
CREATE INDEX IF NOT EXISTS idx_coupons_promotion ON coupons(promotion_id);
CREATE INDEX IF NOT EXISTS idx_coupons_status ON coupons(status);
CREATE INDEX IF NOT EXISTS idx_coupons_dates ON coupons(start_date, end_date);
CREATE INDEX IF NOT EXISTS idx_coupons_email ON coupons(customer_email);

-- Coupon batches (for bulk-generated coupons)
CREATE TABLE IF NOT EXISTS coupon_batches (
    id TEXT PRIMARY KEY,
    promotion_id TEXT NOT NULL,
    prefix TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    length INTEGER NOT NULL DEFAULT 8,
    created_count INTEGER NOT NULL DEFAULT 0,
    used_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (promotion_id) REFERENCES promotions(id) ON DELETE CASCADE
);

-- Promotion rules (advanced rule engine)
CREATE TABLE IF NOT EXISTS promotion_rules (
    id TEXT PRIMARY KEY,
    promotion_id TEXT NOT NULL,
    rule_type TEXT NOT NULL DEFAULT 'Eligibility',
    condition_type TEXT NOT NULL DEFAULT 'Order',
    condition_field TEXT NOT NULL,
    operator TEXT NOT NULL DEFAULT 'Equals',
    value TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    FOREIGN KEY (promotion_id) REFERENCES promotions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_promotion_rules_promotion ON promotion_rules(promotion_id);
CREATE INDEX IF NOT EXISTS idx_promotion_rules_type ON promotion_rules(rule_type);

-- Buy X Get Y rules
CREATE TABLE IF NOT EXISTS buy_x_get_y_rules (
    id TEXT PRIMARY KEY,
    promotion_id TEXT NOT NULL,
    buy_quantity INTEGER NOT NULL DEFAULT 1,
    get_quantity INTEGER NOT NULL DEFAULT 1,
    buy_product_ids TEXT NOT NULL DEFAULT '[]',
    get_product_ids TEXT NOT NULL DEFAULT '[]',
    discount_percent INTEGER NOT NULL DEFAULT 100,
    max_free_items INTEGER,
    created_at TEXT NOT NULL,
    FOREIGN KEY (promotion_id) REFERENCES promotions(id) ON DELETE CASCADE
);

-- Promotion usage tracking
CREATE TABLE IF NOT EXISTS promotion_usages (
    id TEXT PRIMARY KEY,
    promotion_id TEXT,
    coupon_id TEXT,
    order_id TEXT NOT NULL,
    customer_id TEXT,
    customer_email TEXT,
    discount_amount INTEGER NOT NULL DEFAULT 0,
    original_amount INTEGER NOT NULL DEFAULT 0,
    final_amount INTEGER NOT NULL DEFAULT 0,
    used_at TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Applied',
    FOREIGN KEY (promotion_id) REFERENCES promotions(id) ON DELETE SET NULL,
    FOREIGN KEY (coupon_id) REFERENCES coupons(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_promotion_usages_promotion ON promotion_usages(promotion_id);
CREATE INDEX IF NOT EXISTS idx_promotion_usages_coupon ON promotion_usages(coupon_id);
CREATE INDEX IF NOT EXISTS idx_promotion_usages_order ON promotion_usages(order_id);
CREATE INDEX IF NOT EXISTS idx_promotion_usages_customer ON promotion_usages(customer_id);
CREATE INDEX IF NOT EXISTS idx_promotion_usages_date ON promotion_usages(used_at);

-- Referral programs
CREATE TABLE IF NOT EXISTS referral_programs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    referrer_reward_type TEXT NOT NULL DEFAULT 'FixedAmount',
    referrer_reward_value INTEGER NOT NULL DEFAULT 0,
    referee_reward_type TEXT NOT NULL DEFAULT 'FixedAmount',
    referee_reward_value INTEGER NOT NULL DEFAULT 0,
    min_referee_purchase INTEGER NOT NULL DEFAULT 0,
    max_referrals_per_user INTEGER,
    total_referrals INTEGER NOT NULL DEFAULT 0,
    total_rewards_given INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_referral_programs_status ON referral_programs(status);

-- Referrals
CREATE TABLE IF NOT EXISTS referrals (
    id TEXT PRIMARY KEY,
    program_id TEXT NOT NULL,
    referrer_customer_id TEXT NOT NULL,
    referrer_email TEXT NOT NULL,
    referee_email TEXT NOT NULL,
    referral_code TEXT NOT NULL UNIQUE,
    coupon_id TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    referred_at TEXT NOT NULL,
    converted_at TEXT,
    reward_given_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (program_id) REFERENCES referral_programs(id) ON DELETE CASCADE,
    FOREIGN KEY (coupon_id) REFERENCES coupons(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_referrals_program ON referrals(program_id);
CREATE INDEX IF NOT EXISTS idx_referrals_referrer ON referrals(referrer_customer_id);
CREATE INDEX IF NOT EXISTS idx_referrals_code ON referrals(referral_code);
CREATE INDEX IF NOT EXISTS idx_referrals_status ON referrals(status);
