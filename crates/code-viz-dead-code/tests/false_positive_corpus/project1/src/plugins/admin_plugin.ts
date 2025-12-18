// Plugin loaded dynamically - should NOT be marked as dead
export default {
  name: 'admin',
  async init() {
    console.log('Admin plugin initialized');
  },
  async handleAdminAction(action: string) {
    console.log(`Handling admin action: ${action}`);
  }
};

// Helper function - should NOT be dead
export function validateAdmin(adminId: string): boolean {
  return adminId.startsWith('admin_');
}
