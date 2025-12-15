<script lang="ts">
	import { onMount } from 'svelte';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import { MousePointerClick, Power, Rocket } from '@lucide/svelte';
	import {
		startFilter,
		stopFilter,
		updateFilterThreshold,
		queryFilterStatus,
		subscribeFilterEvents,
		type FilterStatus,
		getAutostart,
		setAutostart
	} from '$lib/api/mouseFilter';

	let status = $state<FilterStatus>({ running: false, threshold_ms: 100, blocked_clicks: 0 });
	let threshold = $state(100);
	let autostart = $state(false);
	let busy = $state(false);
	let errorText = $state<string | null>(null);

	const parseError = (reason: unknown) => {
		if (typeof reason === 'string') {
			return reason;
		}
		if (reason && typeof reason === 'object' && 'message' in reason) {
			return String((reason as { message: unknown }).message ?? 'Unknown error');
		}
		return 'An unknown error occurred';
	};

	const clampThreshold = (value: number) => {
		if (Number.isNaN(value)) {
			return 100;
		}
		return Math.min(500, Math.max(50, Math.round(value)));
	};

	const applyStatus = (payload: FilterStatus) => {
		status = payload;
		threshold = payload.threshold_ms;
	};

	const refreshStatus = async () => {
		try {
			const current = await queryFilterStatus();
			applyStatus(current);
			const auto = await getAutostart();
			autostart = auto.enabled;
			errorText = null;
		} catch (error) {
			errorText = parseError(error);
		}
	};

	const handleStatusChange = async (nextValue: boolean) => {
		if (busy) {
			return;
		}
		const nextThreshold = clampThreshold(threshold);
		threshold = nextThreshold;
		busy = true;
		errorText = null;
		try {
			const payload = nextValue ? await startFilter(nextThreshold) : await stopFilter();
			applyStatus(payload);
		} catch (error) {
			errorText = parseError(error);
		} finally {
			busy = false;
		}
	};

	const handleAutostartChange = async (nextValue: boolean) => {
		if (busy) {
			return;
		}
		busy = true;
		errorText = null;
		try {
			const next = await setAutostart(nextValue);
			autostart = next.enabled;
		} catch (error) {
			errorText = parseError(error);
		} finally {
			busy = false;
		}
	};

	const handleThresholdChange = async (value: number) => {
		if (busy) {
			return;
		}
		const next = clampThreshold(value);
		threshold = next;
		busy = true;
		errorText = null;
		try {
			const payload = await updateFilterThreshold(next);
			applyStatus(payload);
		} catch (error) {
			errorText = parseError(error);
		} finally {
			busy = false;
		}
	};

	onMount(() => {
		let isActive = true;
		refreshStatus();
		let cleanup: (() => void) | undefined;
		subscribeFilterEvents({
			onStatus: (payload) => {
				if (isActive) {
					applyStatus(payload);
				}
			}
		}).then((stop) => {
			if (isActive) {
				cleanup = stop;
			} else {
				stop();
			}
		});
		return () => {
			isActive = false;
			cleanup?.();
		};
	});

	const statusLabel = $derived(status.running ? 'Blocking double click' : 'Off');
	const autoLabel = $derived(autostart ? 'Launch at system startup' : 'Do not launch on startup');
	const baseId = $props.id();
	const headingId = baseId;
	const filterId = `${baseId}-filter`;
	const autoId = `${baseId}-auto`;
	const thresholdId = `${baseId}-threshold`;
</script>

<div class="mx-auto flex w-full max-w-xl flex-col gap-3 px-3 py-4 text-white">
	<p id={headingId} class="text-xl font-semibold">Fix mouse double-click</p>
	<div class="relative flex w-full items-start gap-2 rounded-md border border-slate-800 bg-slate-900/70 p-3 shadow-sm outline-none">
		<Switch
			id={filterId}
			checked={status.running}
			onCheckedChange={handleStatusChange}
			class="order-1 h-4 w-6 after:absolute after:inset-0 [&_span]:size-3 data-[state=checked]:[&_span]:translate-x-2.5 data-[state=checked]:[&_span]:rtl:-translate-x-2.5"
			aria-describedby={`${filterId}-description`}
			aria-labelledby={`${filterId}-title`}
		/>
		<div class="flex grow items-center gap-3">
			<MousePointerClick class="size-6 text-emerald-300" />
			<div class="grid grow gap-2">
				<Label id={`${filterId}-title`} for={filterId} class="text-base font-semibold text-white">
					Enable double-click filter
				</Label>
				<p id={`${filterId}-description`} class="text-xs text-slate-400">
					Status: {statusLabel}
				</p>
			</div>
		</div>
	</div>
	<div class="space-y-2 rounded-md border border-slate-800 bg-slate-900/70 p-3 shadow-sm">
		<div class="flex items-center justify-between gap-2">
			<Label for={thresholdId} class="text-sm text-slate-200">Time threshold</Label>
			<span class="text-sm font-semibold text-white">{threshold} ms</span>
		</div>
		<input
			id={thresholdId}
			type="range"
			min="50"
			max="500"
			step="10"
			bind:value={threshold}
			disabled={busy}
			onchange={(event) => handleThresholdChange(Number(event.currentTarget.value))}
			class="w-full accent-emerald-500"
		/>
	</div>
	<div class="relative flex w-full items-start gap-2 rounded-md border border-slate-800 bg-slate-900/70 p-3 shadow-sm outline-none">
		<Switch
			id={autoId}
			checked={autostart}
			onCheckedChange={handleAutostartChange}
			class="order-1 h-4 w-6 after:absolute after:inset-0 [&_span]:size-3 data-[state=checked]:[&_span]:translate-x-2.5 data-[state=checked]:[&_span]:rtl:-translate-x-2.5"
			aria-describedby={`${autoId}-description`}
			aria-labelledby={`${autoId}-title`}
		/>
		<div class="flex grow items-center gap-3">
			<Rocket class="size-6 text-sky-300" />
			<div class="grid grow gap-2">
				<Label id={`${autoId}-title`} for={autoId} class="text-base font-semibold text-white">
					Launch at system startup
				</Label>
				<p id={`${autoId}-description`} class="text-xs text-slate-400">
					Status: {autoLabel}
				</p>
			</div>
		</div>
	</div>
	{#if errorText}
		<p class="rounded-lg border border-rose-600/60 bg-rose-500/10 px-3 py-2 text-rose-200">
			{errorText}
		</p>
	{/if}
</div>

