use crate::models::AppState;
use axum::{Router, extract::State, response::Html, routing::get};
use tera::{Context, Tera};

pub fn page_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(frontdesk))
        .route("/frontdesk", get(frontdesk))
        .route("/kitchen", get(kitchen))
        .route("/administrator", get(administrator))
}

fn create_tera() -> Tera {
    let mut tera = Tera::default();

    tera.add_raw_template("frontdesk.html", FRONTDESK_TEMPLATE)
        .unwrap();
    tera.add_raw_template("kitchen.html", KITCHEN_TEMPLATE)
        .unwrap();
    tera.add_raw_template("admin.html", ADMIN_TEMPLATE).unwrap();

    tera.build_inheritance_chains().unwrap();
    tera
}

pub fn render_template(template_name: &str, context: Context) -> String {
    static TERA: std::sync::OnceLock<Tera> = std::sync::OnceLock::new();
    let tera = TERA.get_or_init(create_tera);
    tera.render(template_name, &context).unwrap()
}

async fn frontdesk(State(state): State<AppState>) -> Html<String> {
    let config = state.config.lock().await.clone();
    let mut ctx = Context::new();
    ctx.insert("order_types", &config.order_types);
    ctx.insert("order_types_json", &serde_json::to_string(&config.order_types).unwrap());
    Html(render_template("frontdesk.html", ctx))
}

async fn kitchen(_state: State<AppState>) -> Html<String> {
    let ctx = Context::new();
    Html(render_template("kitchen.html", ctx))
}

async fn administrator(State(state): State<AppState>) -> Html<String> {
    let config = state.config.lock().await.clone();
    let mut ctx = Context::new();
    ctx.insert("config", &config);
    ctx.insert(
        "order_types_json",
        &serde_json::to_string(&config.order_types).unwrap(),
    );
    Html(render_template("admin.html", ctx))
}

static FRONTDESK_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Frontdesk</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background: #1a1a2e; color: #eee; min-height: 100vh; }
        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }
        h1 { text-align: center; color: #00d4ff; margin-bottom: 30px; }
        .tables-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 20px; }
        .table-card { background: #16213e; border-radius: 16px; padding: 20px; border: 3px solid #333; }
        .table-card.active { border-color: #00d4ff; }
        .table-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px; }
        .table-number { font-size: 28px; font-weight: bold; color: #00d4ff; }
        .table-badge { background: #ff6b6b; color: white; padding: 4px 12px; border-radius: 12px; font-size: 14px; }
        .plate-buttons { display: grid; grid-template-columns: repeat(2, 1fr); gap: 10px; margin-bottom: 15px; }
        .plate-btn { padding: 15px; background: #0f3460; border: 2px solid #333; border-radius: 10px; color: #eee; cursor: pointer; font-size: 14px; transition: all 0.1s; }
        .plate-btn:hover { border-color: #00d4ff; }
        .plate-btn:active { background: #00d4ff; color: #1a1a2e; }
        .plate-btn.selected { background: #00d4ff; color: #1a1a2e; border-color: #00d4ff; }
        .order-items { min-height: 60px; background: #0f3460; border-radius: 10px; padding: 10px; margin-bottom: 15px; }
        .order-item { display: flex; justify-content: space-between; align-items: center; padding: 8px; background: #16213e; border-radius: 6px; margin-bottom: 6px; }
        .order-item:last-child { margin-bottom: 0; }
        .order-item button { background: #ff4757; border: none; border-radius: 4px; padding: 4px 10px; color: white; cursor: pointer; }
        .send-btn { width: 100%; padding: 16px; background: #2ed573; border: none; border-radius: 10px; color: #1a1a2e; font-size: 18px; font-weight: bold; cursor: pointer; }
        .send-btn:hover { background: #26b863; }
        .send-btn:disabled { background: #333; color: #666; cursor: not-allowed; }
        .sent { position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); background: #2ed573; color: #1a1a2e; padding: 30px 60px; border-radius: 20px; font-size: 36px; font-weight: bold; opacity: 0; transition: opacity 0.2s; pointer-events: none; }
        .sent.show { opacity: 1; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Frontdesk</h1>
        <div class="tables-grid" id="tables"></div>
    </div>
    <div class="sent" id="sent">SENT!</div>
    <script>
        const NUM_TABLES = 10;
        const orderTypes = {{ order_types_json | safe }};
        const tables = {};

        function initTables() {
            for (let i = 1; i <= NUM_TABLES; i++) {
                tables[i] = { items: [] };
            }
            renderTables();
        }

        function renderTables() {
            const container = document.getElementById('tables');
            container.innerHTML = '';
            for (let i = 1; i <= NUM_TABLES; i++) {
                const table = tables[i];
                const card = document.createElement('div');
                card.className = 'table-card' + (table.items.length > 0 ? ' active' : '');
                card.innerHTML = `
                    <div class="table-header">
                        <span class="table-number">Table ${i}</span>
                        <span class="table-badge">${table.items.length} plates</span>
                    </div>
                    <div class="plate-buttons">
                        ${orderTypes.map(t => `<button class="plate-btn" onclick="addPlate(${i}, '${t}')">${t}</button>`).join('')}
                    </div>
                    <div class="order-items">
                        ${table.items.length === 0 ? '<div style="color:#666;text-align:center;padding:20px;">No items</div>' : 
                          table.items.map((item, idx) => `<div class="order-item"><span>${item}</span><button onclick="removePlate(${i}, ${idx})">×</button></div>`).join('')}
                    </div>
                    <button class="send-btn" onclick="sendOrder(${i})" ${table.items.length === 0 ? 'disabled' : ''}>SEND</button>
                `;
                container.appendChild(card);
            }
        }

        function addPlate(tableNum, plate) {
            tables[tableNum].items.push(plate);
            renderTables();
        }

        function removePlate(tableNum, idx) {
            tables[tableNum].items.splice(idx, 1);
            renderTables();
        }

        async function sendOrder(tableNum) {
            const table = tables[tableNum];
            if (table.items.length === 0) return;
            const res = await fetch('/api/orders', {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({
                    id: Date.now(),
                    table: tableNum,
                    plates: table.items,
                    notes: null
                })
            });
            if (res.ok) {
                tables[tableNum].items = [];
                renderTables();
                const el = document.getElementById('sent');
                el.classList.add('show');
                setTimeout(() => el.classList.remove('show'), 800);
            }
        }

        initTables();
    </script>
</body>
</html>"#;

static KITCHEN_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Kitchen - Orders</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background: #1a1a2e; color: #eee; min-height: 100vh; }
        * { margin: 0; padding: 0; box-sizing: border-box; }
        .container { max-width: 800px; margin: 0 auto; padding: 20px; }
        h1 { text-align: center; color: #ff6b6b; margin-bottom: 30px; }
        .order { background: #16213e; border-radius: 12px; padding: 20px; margin-bottom: 20px; border-left: 4px solid #ff6b6b; animation: slideIn 0.3s ease; }
        @keyframes slideIn { from { transform: translateX(-20px); opacity: 0; } to { transform: translateX(0); opacity: 1; } }
        .order-table { display: inline-block; background: #00d4ff; color: #1a1a2e; padding: 8px 20px; border-radius: 12px; font-size: 24px; font-weight: bold; margin-bottom: 10px; }
        .order-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px; }
        .order-id { color: #666; font-size: 14px; }
        .order-plates { list-style: none; margin-bottom: 15px; }
        .order-plates li { padding: 12px 0; border-bottom: 1px solid #333; font-size: 20px; font-weight: bold; display: flex; align-items: center; gap: 10px; }
        .order-plates li::before { content: '•'; color: #ff6b6b; }
        .order-plates li:last-child { border-bottom: none; }
        .order-notes { background: #0f3460; padding: 10px; border-radius: 8px; margin-bottom: 15px; font-style: italic; color: #ffd93d; }
        .done-btn { width: 100%; padding: 16px; background: #2ed573; border: none; border-radius: 8px; color: #1a1a2e; font-size: 18px; font-weight: bold; cursor: pointer; }
        .done-btn:hover { background: #26b863; }
        .empty { text-align: center; padding: 60px; color: #666; font-size: 24px; }
        .status { position: fixed; top: 20px; right: 20px; background: #2ed573; padding: 8px 16px; border-radius: 8px; font-size: 12px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Kitchen Display</h1>
        <div class="status" id="status">Live</div>
        <div id="orders"></div>
    </div>
    <script>
        let lastOrderCount = 0;
        async function loadOrders() {
            const res = await fetch('/api/orders');
            const orders = await res.json();
            const container = document.getElementById('orders');
            if (orders.length === 0) {
                container.innerHTML = '<div class="empty">No pending orders</div>';
                lastOrderCount = 0;
                return;
            }
            if (orders.length !== lastOrderCount) {
                lastOrderCount = orders.length;
                container.innerHTML = orders.map(o => `
                    <div class="order">
                        <div class="order-header">
                            <span class="order-table">Table ${o.table}</span>
                            <span class="order-id">#${o.id}</span>
                        </div>
                        <ul class="order-plates">
                            ${o.plates.map(plate => `<li>${plate}</li>`).join('')}
                        </ul>
                        ${o.notes ? `<div class="order-notes">${o.notes}</div>` : ''}
                        <button class="done-btn" onclick="done(${o.id})">DONE</button>
                    </div>`).join('');
            }
        }
        async function done(orderId) {
            await fetch(`/api/orders/${orderId}`, { method: 'DELETE' });
            lastOrderCount = 0;
            loadOrders();
        }
        loadOrders();
        setInterval(loadOrders, 1000);
    </script>
</body>
</html>"#;

static ADMIN_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Administrator</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background: #1a1a2e; color: #eee; min-height: 100vh; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        h1 { text-align: center; color: #ffd93d; margin-bottom: 30px; }
        .card { background: #16213e; border-radius: 12px; padding: 30px; margin-bottom: 20px; }
        .form-group { margin-bottom: 20px; }
        label { display: block; margin-bottom: 8px; color: #aaa; }
        input { width: 100%; padding: 12px; border: 2px solid #333; border-radius: 8px; background: #0f3460; color: #fff; font-size: 16px; }
        input:focus { outline: none; border-color: #ffd93d; }
        .checkbox-group { display: flex; align-items: center; gap: 10px; }
        .checkbox-group input { width: auto; }
        button { padding: 14px 30px; background: #ffd93d; border: none; border-radius: 8px; color: #1a1a2e; font-size: 16px; font-weight: bold; cursor: pointer; }
        button:hover { background: #f0c000; }
        #status { text-align: center; margin-top: 20px; color: #2ed573; }
        .section-title { color: #ff6b6b; margin-bottom: 15px; font-size: 18px; }
        .order-type-item { display: flex; justify-content: space-between; align-items: center; padding: 12px; background: #0f3460; border-radius: 8px; margin-bottom: 10px; }
        .order-type-item span { flex: 1; }
        .remove-type { background: #ff4757; padding: 8px 15px; border: none; border-radius: 6px; color: white; cursor: pointer; }
        .add-type-form { display: flex; gap: 10px; }
        .add-type-form input { flex: 1; }
        .add-type-form button { background: #2ed573; white-space: nowrap; }
        .orders-section { margin-top: 30px; }
        .orders-section h2 { color: #ff6b6b; margin-bottom: 15px; }
        .order-item { background: #0f3460; padding: 15px; border-radius: 8px; margin-bottom: 10px; display: flex; justify-content: space-between; }
        .clear-orders { background: #ff4757; color: white; width: 100%; }
        .clear-orders:hover { background: #ff3344; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Administrator</h1>
        <div class="card">
            <div class="form-group">
                <label>Display Name</label>
                <input type="text" id="displayName" value="{{ config.display_name }}">
            </div>
            <div class="form-group">
                <label>Auto Refresh (ms)</label>
                <input type="number" id="autoRefresh" value="{{ config.auto_refresh_ms }}">
            </div>
            <div class="form-group checkbox-group">
                <input type="checkbox" id="soundEnabled" {% if config.sound_enabled %}checked{% endif %}>
                <label style="margin: 0;">Sound Enabled</label>
            </div>
            <button onclick="saveConfig()">Save Configuration</button>
            <div id="status"></div>
        </div>
        <div class="card">
            <div class="section-title">Order Types</div>
            <div id="orderTypesList"></div>
            <div class="add-type-form">
                <input type="text" id="newTypeInput" placeholder="New order type...">
                <button onclick="addOrderType()">Add</button>
            </div>
        </div>
        <div class="card orders-section">
            <h2>Order Queue</h2>
            <div id="orderQueue"></div>
            <button class="clear-orders" onclick="clearAllOrders()" style="margin-top: 15px; width: 100%;">Clear All Orders</button>
        </div>
    </div>
    <script>
        let currentOrderTypes = {{ order_types_json | safe }};

        async function loadConfig() {
            const res = await fetch('/api/config');
            const config = await res.json();
            document.getElementById('displayName').value = config.display_name;
            document.getElementById('autoRefresh').value = config.auto_refresh_ms;
            document.getElementById('soundEnabled').checked = config.sound_enabled;
            currentOrderTypes = config.order_types;
            renderOrderTypes();
        }
        async function saveConfig() {
            const config = {
                display_name: document.getElementById('displayName').value,
                auto_refresh_ms: parseInt(document.getElementById('autoRefresh').value),
                sound_enabled: document.getElementById('soundEnabled').checked,
                order_types: currentOrderTypes
            };
            await fetch('/api/config', { method: 'POST', headers: {'Content-Type': 'application/json'}, body: JSON.stringify(config) });
            document.getElementById('status').textContent = 'Configuration saved!';
            setTimeout(() => document.getElementById('status').textContent = '', 2000);
        }
        function renderOrderTypes() {
            document.getElementById('orderTypesList').innerHTML = currentOrderTypes.length === 0
                ? '<p style="color:#666;">No order types configured</p>'
                : currentOrderTypes.map((t, i) => `<div class="order-type-item"><span>${t}</span><button class="remove-type" onclick="removeOrderType(${i})">Remove</button></div>`).join('');
        }
        function addOrderType() {
            const input = document.getElementById('newTypeInput');
            const type = input.value.trim();
            if (type && !currentOrderTypes.includes(type)) {
                currentOrderTypes.push(type);
                renderOrderTypes();
                input.value = '';
                saveConfig();
            }
        }
        function removeOrderType(index) {
            currentOrderTypes.splice(index, 1);
            renderOrderTypes();
            saveConfig();
        }
        async function loadOrders() {
            const res = await fetch('/api/orders');
            const orders = await res.json();
            document.getElementById('orderQueue').innerHTML = orders.length === 0
                ? '<p style="color:#666;">No orders in queue</p>'
                : orders.map(o => `<div class="order-item"><span>${o.order_type} - ${o.table}</span><span style="color:#666;">#${o.id}</span></div>`).join('');
        }
        async function clearAllOrders() {
            if (confirm('Clear all orders?')) {
                while (true) {
                    const res = await fetch('/api/orders');
                    const orders = await res.json();
                    if (orders.length === 0) break;
                    await fetch('/api/orders/next', { method: 'DELETE' });
                }
                loadOrders();
            }
        }
        loadConfig();
        loadOrders();
    </script>
</body>
</html>"#;
