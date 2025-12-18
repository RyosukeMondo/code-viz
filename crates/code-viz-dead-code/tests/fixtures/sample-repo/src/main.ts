/**
 * Main entry point of the application.
 * This file should be detected as an entry point.
 */

import { activeFunction, processData } from './used';
import { publicApi } from './internal';

/**
 * Main function - entry point
 * EXPECTED: LIVE - this is the entry point
 */
function main(): void {
  const result = activeFunction(5);
  console.log(`Active result: ${result}`);

  const data = ['hello', 'world', 'test'];
  const count = processData(data);
  console.log(`Processed ${count} items`);

  const apiResult = publicApi('sample');
  console.log(`API result: ${apiResult}`);
}

/**
 * Helper function used by main
 * EXPECTED: LIVE - used by entry point
 */
function setupApp(): void {
  console.log("Setting up application...");
}

// Execute main
setupApp();
main();
