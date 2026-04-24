/**
 * Copyright (c) 2026 Simon Project
 * Licensed under the Apache License, Version 2.0
 */

import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { ModuleMetadata } from '../types/modules';

/**
 * BlueCoreAPI Bridge
 * Centralized communication layer between React and Tauri/Rust
 */
export class BlueCoreAPI {
    private static instance: BlueCoreAPI;

    private constructor() { }

    static getInstance(): BlueCoreAPI {
        if (!this.instance) {
            this.instance = new BlueCoreAPI();
        }
        return this.instance;
    }

    /**
     * Fetch all registered modules from backend
     */
    async getModules(): Promise<ModuleMetadata[]> {
        try {
            return await invoke<ModuleMetadata[]>('get_available_modules');
        } catch (error) {
            console.error('Failed to fetch modules:', error);
            throw error;
        }
    }

    /**
     * Toggle module status (enabled/disabled)
     */
    async toggleModule(id: string, enabled: boolean): Promise<void> {
        try {
            await invoke('toggle_module', { id, enabled });
        } catch (error) {
            console.error(`Failed to toggle module ${id}:`, error);
            throw error;
        }
    }

    /**
     * Execute a specific action on a module
     */
    async executeModuleAction(id: string, action: string, data: any = {}): Promise<any> {
        try {
            return await invoke('execute_module_action', { id, action, data });
        } catch (error) {
            console.error(`Module action failed [${id}:${action}]:`, error);
            throw error;
        }
    }

    /**
     * Listen for module status updates from backend
     */
    async onModuleStatusUpdate(callback: (payload: any) => void) {
        return await listen('module_status_changed', (event) => {
            callback(event.payload);
        });
    }
}
