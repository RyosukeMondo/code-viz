// Plugin loaded dynamically - should NOT be marked as dead
export default {
  name: 'user',
  async init() {
    console.log('User plugin initialized');
  },
  async handleUserAction(action: string) {
    console.log(`Handling user action: ${action}`);
  }
};

// Helper function used by plugin - should NOT be dead
export function validateUser(userId: string): boolean {
  return userId.length > 0;
}
