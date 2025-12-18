// Public library API - all exports should have LOW confidence for being dead
// since they're part of the public API

export function createUser(name: string, email: string) {
  return { name, email, id: Math.random().toString() };
}

export function deleteUser(userId: string) {
  console.log(`Deleting user ${userId}`);
}

export function updateUser(userId: string, data: any) {
  console.log(`Updating user ${userId}`, data);
}

// Internal function - not exported, truly dead
function internalHelper() {
  return 'This is never used';
}
