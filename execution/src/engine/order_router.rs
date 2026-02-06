use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Order types supported by the router
#[derive(Clone, Debug, PartialEq)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
    TWAP,
    VWAP,
    Iceberg,
    PostOnly,
}

/// Order side
#[derive(Clone, Debug, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order state
#[derive(Clone, Debug, PartialEq)]
pub enum OrderState {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    Expired,
}

/// Order structure
#[derive(Clone, Debug)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub state: OrderState,
    pub exchange: String,
    pub timestamp: u64,
    pub updated_at: u64,
    
    // TWAP/VWAP parameters
    pub duration_secs: Option<u64>,
    pub num_slices: Option<u32>,
    
    // Iceberg parameters
    pub visible_quantity: Option<f64>,
}

impl Order {
    pub fn new(
        id: u64,
        symbol: String,
        side: OrderSide,
        order_type: OrderType,
        quantity: f64,
        price: Option<f64>,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Self {
            id,
            symbol,
            side,
            order_type,
            quantity,
            filled_quantity: 0.0,
            price,
            stop_price: None,
            state: OrderState::New,
            exchange: String::new(),
            timestamp: now,
            updated_at: now,
            duration_secs: None,
            num_slices: None,
            visible_quantity: None,
        }
    }

    pub fn remaining_quantity(&self) -> f64 {
        self.quantity - self.filled_quantity
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.state,
            OrderState::New | OrderState::PartiallyFilled
        )
    }
}

/// Exchange route
#[derive(Clone, Debug)]
pub struct ExchangeRoute {
    pub name: String,
    pub priority: u8,
    pub fee_rate: f64,
    pub min_order_size: f64,
    pub max_order_size: f64,
    pub supports_order_types: Vec<OrderType>,
}

impl ExchangeRoute {
    pub fn can_handle(&self, order: &Order) -> bool {
        order.quantity >= self.min_order_size
            && order.quantity <= self.max_order_size
            && self.supports_order_types.contains(&order.order_type)
    }
}

/// Order validation result
#[derive(Debug)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
}

/// Smart order router
pub struct OrderRouter {
    /// Next order ID
    next_order_id: AtomicU64,
    /// Active orders
    orders: Arc<RwLock<HashMap<u64, Order>>>,
    /// Exchange routes
    routes: Arc<RwLock<Vec<ExchangeRoute>>>,
}

impl OrderRouter {
    pub fn new() -> Self {
        Self {
            next_order_id: AtomicU64::new(1),
            orders: Arc::new(RwLock::new(HashMap::new())),
            routes: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add an exchange route
    pub fn add_route(&self, route: ExchangeRoute) {
        let mut routes = self.routes.write();
        routes.push(route);
        // Sort by priority (higher priority first)
        routes.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Generate next order ID
    pub fn next_order_id(&self) -> u64 {
        self.next_order_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Validate an order
    pub fn validate_order(&self, order: &Order) -> ValidationResult {
        // Basic validation
        if order.quantity <= 0.0 {
            return ValidationResult::Invalid("Quantity must be positive".to_string());
        }

        if matches!(order.order_type, OrderType::Limit | OrderType::StopLimit) {
            if order.price.is_none() {
                return ValidationResult::Invalid("Price required for limit orders".to_string());
            }
            if let Some(price) = order.price {
                if price <= 0.0 {
                    return ValidationResult::Invalid("Price must be positive".to_string());
                }
            }
        }

        if matches!(order.order_type, OrderType::Stop | OrderType::StopLimit) {
            if order.stop_price.is_none() {
                return ValidationResult::Invalid("Stop price required for stop orders".to_string());
            }
        }

        if order.order_type == OrderType::TWAP {
            if order.duration_secs.is_none() || order.num_slices.is_none() {
                return ValidationResult::Invalid("TWAP requires duration and slices".to_string());
            }
        }

        if order.order_type == OrderType::Iceberg {
            if order.visible_quantity.is_none() {
                return ValidationResult::Invalid("Iceberg requires visible quantity".to_string());
            }
            if let Some(visible) = order.visible_quantity {
                if visible >= order.quantity {
                    return ValidationResult::Invalid(
                        "Visible quantity must be less than total".to_string()
                    );
                }
            }
        }

        ValidationResult::Valid
    }

    /// Route an order to best exchange
    pub fn route_order(&self, mut order: Order) -> Result<Order, RouterError> {
        // Validate order
        match self.validate_order(&order) {
            ValidationResult::Valid => {}
            ValidationResult::Invalid(msg) => {
                order.state = OrderState::Rejected;
                return Err(RouterError::ValidationFailed(msg));
            }
        }

        // Find best exchange
        let routes = self.routes.read();
        let best_route = routes
            .iter()
            .find(|route| route.can_handle(&order))
            .ok_or(RouterError::NoRouteAvailable)?;

        order.exchange = best_route.name.clone();
        
        // Store order
        let mut orders = self.orders.write();
        orders.insert(order.id, order.clone());

        Ok(order)
    }

    /// Update order state
    pub fn update_order_state(&self, order_id: u64, state: OrderState) -> Result<(), RouterError> {
        let mut orders = self.orders.write();
        let order = orders
            .get_mut(&order_id)
            .ok_or(RouterError::OrderNotFound)?;

        order.state = state;
        order.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Ok(())
    }

    /// Fill order (partial or complete)
    pub fn fill_order(&self, order_id: u64, quantity: f64) -> Result<Order, RouterError> {
        let mut orders = self.orders.write();
        let order = orders
            .get_mut(&order_id)
            .ok_or(RouterError::OrderNotFound)?;

        order.filled_quantity += quantity;
        order.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        if order.filled_quantity >= order.quantity {
            order.state = OrderState::Filled;
        } else {
            order.state = OrderState::PartiallyFilled;
        }

        Ok(order.clone())
    }

    /// Cancel an order
    pub fn cancel_order(&self, order_id: u64) -> Result<Order, RouterError> {
        let mut orders = self.orders.write();
        let order = orders
            .get_mut(&order_id)
            .ok_or(RouterError::OrderNotFound)?;

        if !order.is_active() {
            return Err(RouterError::OrderNotActive);
        }

        order.state = OrderState::Cancelled;
        order.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Ok(order.clone())
    }

    /// Get order by ID
    pub fn get_order(&self, order_id: u64) -> Option<Order> {
        self.orders.read().get(&order_id).cloned()
    }

    /// Get all active orders
    pub fn get_active_orders(&self) -> Vec<Order> {
        self.orders
            .read()
            .values()
            .filter(|order| order.is_active())
            .cloned()
            .collect()
    }

    /// Get all orders for a symbol
    pub fn get_orders_by_symbol(&self, symbol: &str) -> Vec<Order> {
        self.orders
            .read()
            .values()
            .filter(|order| order.symbol == symbol)
            .cloned()
            .collect()
    }
}

impl Default for OrderRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum RouterError {
    ValidationFailed(String),
    NoRouteAvailable,
    OrderNotFound,
    OrderNotActive,
    ExchangeError(String),
}

impl std::fmt::Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouterError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            RouterError::NoRouteAvailable => write!(f, "No exchange route available"),
            RouterError::OrderNotFound => write!(f, "Order not found"),
            RouterError::OrderNotActive => write!(f, "Order is not active"),
            RouterError::ExchangeError(msg) => write!(f, "Exchange error: {}", msg),
        }
    }
}

impl std::error::Error for RouterError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_creation() {
        let order = Order::new(
            1,
            "BTCUSDT".to_string(),
            OrderSide::Buy,
            OrderType::Market,
            1.0,
            None,
        );
        assert_eq!(order.state, OrderState::New);
        assert_eq!(order.remaining_quantity(), 1.0);
    }

    #[test]
    fn test_order_validation() {
        let router = OrderRouter::new();
        
        let order = Order::new(
            1,
            "BTCUSDT".to_string(),
            OrderSide::Buy,
            OrderType::Market,
            1.0,
            None,
        );
        
        assert!(matches!(router.validate_order(&order), ValidationResult::Valid));
    }

    #[test]
    fn test_invalid_quantity() {
        let router = OrderRouter::new();
        
        let order = Order::new(
            1,
            "BTCUSDT".to_string(),
            OrderSide::Buy,
            OrderType::Market,
            -1.0,
            None,
        );
        
        assert!(matches!(
            router.validate_order(&order),
            ValidationResult::Invalid(_)
        ));
    }
}
