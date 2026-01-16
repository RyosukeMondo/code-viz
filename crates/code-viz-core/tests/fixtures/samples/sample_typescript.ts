// Sample TypeScript file for testing
import { EventEmitter } from 'events';

/**
 * A simple user management class
 */
export class UserManager extends EventEmitter {
    private users: Map<string, User>;

    constructor() {
        super();
        this.users = new Map();
    }

    /**
     * Add a new user
     */
    addUser(id: string, name: string, email: string): void {
        const user: User = { id, name, email };
        this.users.set(id, user);
        this.emit('userAdded', user);
    }

    /**
     * Get a user by ID
     */
    getUser(id: string): User | undefined {
        return this.users.get(id);
    }

    /**
     * Remove a user
     */
    removeUser(id: string): boolean {
        const existed = this.users.delete(id);
        if (existed) {
            this.emit('userRemoved', id);
        }
        return existed;
    }

    /**
     * Get all users
     */
    getAllUsers(): User[] {
        return Array.from(this.users.values());
    }

    /**
     * Find users by name
     */
    findUsersByName(name: string): User[] {
        return this.getAllUsers().filter(user => user.name.includes(name));
    }
}

interface User {
    id: string;
    name: string;
    email: string;
}

// Helper function
export const validateEmail = (email: string): boolean => {
    return email.includes('@');
};
