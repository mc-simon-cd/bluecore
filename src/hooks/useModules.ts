/**
 * Copyright (c) 2026 Simon Project
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

import { useState, useEffect, useCallback } from 'react';
import { BlueCoreAPI } from '../api/BlueCoreAPI';
import { ModuleMetadata, ModuleState } from '../types/modules';

export function useModules() {
    const [modules, setModules] = useState<ModuleState[]>([]);
    const [loading, setLoading] = useState(true);
    const api = BlueCoreAPI.getInstance();

    const refreshModules = useCallback(async () => {
        setLoading(true);
        try {
            const metadata = await api.getModules();
            const states: ModuleState[] = metadata.map(m => ({
                metadata: m,
                isLoaded: true,
                isActive: m.enabled,
            }));
            setModules(states);
        } catch (error) {
            console.error('Failed to load modules:', error);
        } finally {
            setLoading(false);
        }
    }, [api]);

    const toggleModule = async (id: string, enabled: boolean) => {
        try {
            await api.toggleModule(id, enabled);
            setModules(prev => prev.map(m =>
                m.metadata.id === id ? { ...m, isActive: enabled } : m
            ));
        } catch (error) {
            console.error(`Toggle failed for ${id}:`, error);
        }
    };

    useEffect(() => {
        refreshModules();

        // Listen for backend updates
        const unlisten = api.onModuleStatusUpdate((payload) => {
            console.log('Module status updated from backend:', payload);
            refreshModules();
        });

        return () => {
            unlisten.then(fn => fn());
        };
    }, [refreshModules, api]);

    return {
        modules,
        loading,
        toggleModule,
        refreshModules
    };
}
