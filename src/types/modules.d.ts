/**
 * Copyright (c) 2026 Simon Project
 * Licensed under the Apache License, Version 2.0
 */

export interface ModuleMetadata {
    id: string;
    name: string;
    version: string;
    description: string;
    enabled: boolean;
    priority: number;
    permissions: string[];
    icon?: string; // Optinal icon for UI
}

export type ModuleAction = {
    type: string;
    payload: any;
};

export interface ModuleState {
    metadata: ModuleMetadata;
    isLoaded: boolean;
    isActive: boolean;
    error?: string;
}
