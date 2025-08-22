# 🚀 Webhook CLI - Quick Examples

## Real-world Usage Scenarios

### 1. API Development
```bash
# Generate a token for your new API
webhook generate

# Monitor POST requests during development
webhook monitor --token YOUR_TOKEN --method POST --interval 1

# Check what requests came in
webhook logs --token YOUR_TOKEN --count 20
```

### 2. CI/CD Pipeline Testing
```bash
# In your test script:
TOKEN=$(webhook generate | grep "Token:" | cut -d' ' -f2)
echo "Testing webhook: $TOKEN"

# Run your tests that send webhooks...

# Check results
webhook logs --token $TOKEN
```

### 3. Third-party Integration Testing
```bash
# Monitor webhooks from GitHub, Stripe, etc.
webhook monitor --token YOUR_TOKEN

# Check specific request details
webhook show --token YOUR_TOKEN --request-id abc123...
```

### 4. Debug Production Issues
```bash
# Quick monitoring session
webhook monitor --token PROD_TOKEN --method POST

# Check recent activity
webhook logs --token PROD_TOKEN --count 100 --method POST
```

## Output Examples

### Real-time Monitoring
```
🔍 Starting webhook monitor...
Token: abc123-def456-ghi789
Press Ctrl+C to quit
────────────────────────────────────────────────────────────────────────────────

14:30:25 POST /api/payments (a1b2c3d4...) 📄 {"amount": 100, "currency": "USD"}
14:31:02 GET /health (e5f6g7h8...) 📄 (empty)

🆕 NEW REQUEST
14:31:45 POST /webhooks/stripe (i9j0k1l2...) 📄 {"type": "payment_intent.succeeded"}
Body: {"id": "pi_123", "object": "payment_intent", "amount": 2000, "currency": "usd"}
────────────────────────────────────────────────────────────────────────────────
```

### Request Details
```
📋 REQUEST DETAILS
══════════════════════════════════════════════════════
ID: a1b2c3d4-e5f6-7890-abcd-ef1234567890
Method: POST
Path: /api/webhooks/payment

📋 HEADERS
──────────────────────────────
Content-Type: application/json
Stripe-Signature: t=1234567890,v1=abc123...
User-Agent: Stripe/1.0

📄 REQUEST BODY
──────────────────────────────
{
  "id": "evt_1234567890",
  "object": "event",
  "type": "payment_intent.succeeded",
  "data": {
    "object": {
      "id": "pi_1234567890",
      "amount": 2000,
      "currency": "usd",
      "status": "succeeded"
    }
  }
}
```

## Performance Tips

- **Filter by method** for focused monitoring: `--method POST`
- **Adjust intervals** for different use cases: `--interval 1` (fast) or `--interval 10` (slower)
- **Limit request counts** for faster loading: `--count 10`

## Integration with Other Tools

### Pipe to grep
```bash
webhook logs --token TOKEN | grep "POST"
```

### Save to file
```bash
webhook logs --token TOKEN > webhook-logs.txt
```

### Watch mode with external tools
```bash
watch -n 5 "webhook logs --token TOKEN --count 5"
```

Enjoy fast, efficient webhook testing! ⚡
