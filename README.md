# Webhook CLI

A fast, efficient command-line tool for webhook testing and monitoring built in Rust.

## Features

- 🚀 **Fast & Efficient**: Built in Rust for maximum performance
- 🔑 **Token Management**: Generate new webhook tokens instantly
- 📊 **Real-time Monitoring**: Watch webhook requests as they arrive
- 📜 **Request Logs**: View historical webhook requests
- 🔍 **Detailed Inspection**: Show full request details including headers and body
- 🎯 **Method Filtering**: Filter requests by HTTP method
- 🎨 **Colorized Output**: Beautiful, readable colored terminal output
- ⚡ **Low Resource Usage**: Minimal CPU and memory footprint

## Installation

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)

### Build from Source
```bash
# Clone or navigate to the CLI directory
cd WebhookUI-CLI

# Build the project
cargo build --release

# The binary will be available at target/release/webhook
```

### Install Globally (Optional)
```bash
cargo install --path .
```

## Usage

### Generate a New Webhook Token
```bash
webhook generate
```
Output:
```
🔑 New webhook token generated!

Token: 123e4567-e89b-12d3-a456-426614174000
Webhook URL: https://webhooktest.emergemarket.dev/123e4567-e89b-12d3-a456-426614174000

💡 Usage examples:
  webhook monitor --token 123e4567-e89b-12d3-a456-426614174000
  webhook logs --token 123e4567-e89b-12d3-a456-426614174000
```

### Monitor Requests in Real-time
```bash
# Monitor with an existing token
webhook monitor --token YOUR_TOKEN

# Monitor and auto-generate a new token
webhook monitor

# Filter by HTTP method
webhook monitor --token YOUR_TOKEN --method POST

# Custom refresh interval (default: 3 seconds)
webhook monitor --token YOUR_TOKEN --interval 5

# Show full request body with proper JSON formatting
webhook monitor --token YOUR_TOKEN --full-body

# Show request headers
webhook monitor --token YOUR_TOKEN --show-headers

# Combine multiple options
webhook monitor --token YOUR_TOKEN --full-body --show-headers --method POST
```

### View Request Logs
```bash
# Show recent requests
webhook logs --token YOUR_TOKEN

# Show more requests
webhook logs --token YOUR_TOKEN --count 100

# Filter by method
webhook logs --token YOUR_TOKEN --method GET

# Show logs with full request bodies
webhook logs --token YOUR_TOKEN --full-body

# Show logs with headers
webhook logs --token YOUR_TOKEN --show-headers

# Combine options for detailed view
webhook logs --token YOUR_TOKEN --full-body --show-headers
```

### Show Request Details
```bash
webhook show --token YOUR_TOKEN --request-id REQUEST_ID
```

## Command Reference

### `webhook generate`
Generates a new webhook token (UUID) and displays the webhook URL.

### `webhook monitor`
Monitors webhook requests in real-time.

**Options:**
- `-t, --token <TOKEN>` - Webhook token (generates new if not provided)
- `-c, --count <COUNT>` - Number of recent requests to show initially (default: 10)
- `-i, --interval <INTERVAL>` - Refresh interval in seconds (default: 3)
- `-m, --method <METHOD>` - Filter by HTTP method (GET, POST, PUT, DELETE, PATCH)
- `--full-body` - Show full request body with proper formatting (JSON, form data, etc.)
- `--show-headers` - Show request headers

### `webhook logs`
Shows historical webhook requests.

**Options:**
- `-t, --token <TOKEN>` - Webhook token (required)
- `-c, --count <COUNT>` - Number of requests to fetch (default: 50)
- `-m, --method <METHOD>` - Filter by HTTP method
- `--full-body` - Show full request body with proper formatting
- `--show-headers` - Show request headers

### `webhook show`
Shows detailed information for a specific request.

**Options:**
- `-t, --token <TOKEN>` - Webhook token (required)
- `-r, --request-id <ID>` - Request ID to show details for (required)

## Examples

### Complete Workflow
```bash
# 1. Generate a new token
webhook generate

# 2. Monitor requests with full body and headers display (use token from step 1)
webhook monitor --token abc123-def456-ghi789 --full-body --show-headers

# 3. In another terminal, view logs with full details
webhook logs --token abc123-def456-ghi789 --full-body --show-headers

# 4. Show details of a specific request
webhook show --token abc123-def456-ghi789 --request-id req-12345
```

### Development Workflow
```bash
# Monitor only POST requests for your API with full details
webhook monitor --token YOUR_TOKEN --method POST --interval 1 --full-body --show-headers

# Check recent webhook activity with formatted bodies and headers
webhook logs --token YOUR_TOKEN --count 20 --full-body --show-headers
```

## Output Examples

### Monitor Mode
```
🔍 Starting webhook monitor...
Token: 123e4567-e89b-12d3-a456-426614174000
Press Ctrl+C to quit
────────────────────────────────────────────────────────────────────────────────

14:30:25 POST /api/notify (a1b2c3d4-e5f6-7890-abcd-ef1234567890) 📄 {"event": "payment.completed"}
📋 HEADERS
  Content-Type: application/json
  Authorization: Bearer token123
  User-Agent: MyApp/1.0

14:31:02 GET /webhook/status (e5f6g7h8-i9j0-1234-5678-90abcdef1234) 📄 (empty)

🆕 NEW REQUEST
14:31:45 POST /api/callback (i9j0k1l2-m3n4-5678-9012-34567890abcd) 📄 {"user_id": 12345}

📋 HEADERS
  Content-Type: application/json
  X-Signature: sha256=abc123...

📄 REQUEST BODY
──────────────────────────────
{
  "user_id": 12345,
  "action": "login",
  "timestamp": "2024-01-15T14:31:45Z",
  "metadata": {
    "ip": "192.168.1.1",
    "user_agent": "MyApp/1.0"
  }
}
────────────────────────────────────────────────────────────────────────────────
```

### Request Details
```
🔍 Fetching request details...
📋 REQUEST DETAILS
══════════════════════════════════════════════════════
ID: a1b2c3d4-e5f6-7890-abcd-ef1234567890
Token: 123e4567-e89b-12d3-a456-426614174000
Date: 14:30:25
Method: POST
Path: /api/notify

📋 HEADERS
──────────────────────────────
Content-Type: application/json
Authorization: Bearer token123
User-Agent: MyApp/1.0

📄 REQUEST BODY
──────────────────────────────
{
  "event": "payment.completed",
  "amount": 100.00,
  "currency": "USD",
  "user_id": 12345
}
```

## Performance

- **Memory Usage**: ~2-5 MB
- **CPU Usage**: Minimal (polling-based)
- **Network**: Efficient HTTP requests only when needed
- **Startup Time**: <100ms

## Comparison with Other Versions

| Feature | Web UI | Desktop App | CLI Tool |
|---------|--------|-------------|----------|
| Resource Usage | Medium | Medium | **Minimal** |
| Setup Required | Proxy Server | None | **None** |
| Real-time Monitoring | ✅ | ✅ | ✅ |
| Scriptable | ❌ | ❌ | **✅** |
| Portable | ❌ | ✅ | **✅** |
| Automation Friendly | ❌ | ❌ | **✅** |

## Use Cases

- **CI/CD Pipelines**: Monitor webhooks during automated testing
- **Development**: Quick webhook debugging without opening browsers
- **Server Monitoring**: Lightweight webhook monitoring on servers
- **Scripting**: Integrate webhook monitoring into shell scripts
- **Remote Development**: SSH-friendly webhook testing

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License
