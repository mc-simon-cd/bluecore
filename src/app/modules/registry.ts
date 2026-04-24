/**
 * Copyright (c) 2026 Simon Project
 * Licensed under the Apache License, Version 2.0
 */

import React from 'react';

export interface ModuleMetadata {
    id: string;
    name: string;
    version: string;
    description: string;
    icon?: string;
}

export interface BlueCoreModule {
    metadata: ModuleMetadata;
    component: React.ComponentType;
}

export class ModuleRegistry {
    private static modules: Map<string, BlueCoreModule> = new Map();

    static register(module: BlueCoreModule) {
        this.modules.set(module.metadata.id, module);
        console.log(`Module registered: ${module.metadata.name} (${module.metadata.id})`);
    }

    static getModule(id: string): BlueCoreModule | undefined {
        return this.modules.get(id);
    }

    static getAllModules(): BlueCoreModule[] {
        return Array.from(this.modules.values());
    }
}
