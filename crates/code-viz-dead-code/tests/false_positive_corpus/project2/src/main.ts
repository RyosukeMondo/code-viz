// Main file that uses reflection patterns
import * as handlers from './handlers.ts';

// Call handlers by name using reflection
export function callHandler(handlerName: string, data: any) {
  const handler = (handlers as any)[handlerName];
  if (handler) {
    return handler(data);
  }
  throw new Error(`Handler ${handlerName} not found`);
}

// Entry point
const result1 = callHandler('handleUser', { id: '123' });
const result2 = callHandler('handleOrder', { orderId: '456' });
const result3 = callHandler('handlePayment', { amount: 100 });

console.log(result1, result2, result3);
