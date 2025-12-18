// Main application
import { processData } from './processor.ts';

export function main() {
  const data = { value: 42 };
  processData(data);
}

main();
