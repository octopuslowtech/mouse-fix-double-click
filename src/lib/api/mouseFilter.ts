import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

export type FilterStatus = {
    running: boolean;
    threshold_ms: number;
    blocked_clicks: number;
};

export type AutostartState = {
    enabled: boolean;
};

export type BlockedEventPayload = {
    delta_ms: number;
};

export const startFilter = (thresholdMs: number) =>
    invoke<FilterStatus>('start_filter', { thresholdMs });

export const stopFilter = () => invoke<FilterStatus>('stop_filter');

export const updateFilterThreshold = (thresholdMs: number) =>
    invoke<FilterStatus>('update_threshold', { thresholdMs });

export const queryFilterStatus = () => invoke<FilterStatus>('get_filter_status');

export const getAutostart = () =>
    invoke<boolean>('get_autostart_enabled').then((enabled) => ({ enabled }));

export const setAutostart = (enabled: boolean) =>
    invoke<boolean>('set_autostart_enabled', { enabled }).then((state) => ({ enabled: state }));

type ListenerOptions = {
    onStatus?: (payload: FilterStatus) => void;
    onBlocked?: (payload: BlockedEventPayload) => void;
};

export const subscribeFilterEvents = async (options: ListenerOptions = {}) => {
    const unsubs: UnlistenFn[] = [];
    if (options.onStatus) {
        const unlisten = await listen<FilterStatus>('filter_status_changed', (event) => {
            options.onStatus?.(event.payload);
        });
        unsubs.push(unlisten);
    }
    if (options.onBlocked) {
        const unlisten = await listen<BlockedEventPayload>('click_blocked', (event) => {
            options.onBlocked?.(event.payload);
        });
        unsubs.push(unlisten);
    }
    return () => {
        unsubs.forEach((unsub) => unsub());
    };
};

