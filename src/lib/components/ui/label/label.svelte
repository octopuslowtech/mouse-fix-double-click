<script lang="ts">
	import type { ClassValue } from 'svelte/elements';
	import type { Snippet } from 'svelte';

	let {
		for: htmlFor,
		class: className,
		children,
		...rest
	}: { for?: string; class?: ClassValue; children?: Snippet; [key: string]: unknown } = $props();
</script>

<label for={htmlFor} class={className} {...rest}>
	{@render children?.()}
</label>

