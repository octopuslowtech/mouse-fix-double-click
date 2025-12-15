<script lang="ts">
	import type { ClassValue } from 'svelte/elements';

	let {
		id,
		checked = false,
		disabled = false,
		class: className,
		onCheckedChange,
		...rest
	}: {
		id?: string;
		checked?: boolean;
		disabled?: boolean;
		class?: ClassValue;
		onCheckedChange?: (value: boolean) => void;
		[key: string]: unknown;
	} = $props();

	let current = $state(checked);

	$effect(() => {
		current = checked;
	});

	const toggle = () => {
		if (disabled) {
			return;
		}
		const next = !current;
		current = next;
		onCheckedChange?.(next);
	};
</script>

<button
	type="button"
	{id}
	role="switch"
	aria-checked={current}
	aria-disabled={disabled}
	data-state={current ? 'checked' : 'unchecked'}
	onclick={toggle}
	disabled={disabled}
	{...rest}
	class={[
		'relative inline-flex h-6 w-10 items-center rounded-full bg-slate-800 transition-all data-[state=checked]:bg-emerald-500',
		className
	]}
>
	<span
		class={[
			'ml-1 inline-block h-4 w-4 rounded-full bg-white transition-transform',
			current ? 'translate-x-4' : 'translate-x-0'
		]}
	></span>
</button>

