// Utility functions exported as separate entry point
export function formatDate(date: Date): string {
  return date.toISOString();
}

export function parseDate(dateStr: string): Date {
  return new Date(dateStr);
}

export function validateEmail(email: string): boolean {
  return email.includes('@');
}

// Truly unused utility - dead code
export function obsoleteFunction() {
  return 'This was deprecated and never used';
}
