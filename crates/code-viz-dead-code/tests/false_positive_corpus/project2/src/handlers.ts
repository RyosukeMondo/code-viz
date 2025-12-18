// Handlers accessed via reflection - should NOT be marked as dead (low confidence)
export function handleUser(data: { id: string }) {
  return `Handling user ${data.id}`;
}

export function handleOrder(data: { orderId: string }) {
  return `Handling order ${data.orderId}`;
}

export function handlePayment(data: { amount: number }) {
  return `Processing payment of ${data.amount}`;
}

// This one is truly dead - never referenced
export function handleRefund(data: { refundId: string }) {
  return `Processing refund ${data.refundId}`;
}
