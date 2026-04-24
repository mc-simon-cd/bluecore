/**
 * Copyright (c) 2026 Simon Project
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
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
