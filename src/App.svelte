<script lang="ts">
	import FilterDashboard from '$lib/components/FilterDashboard.svelte';
</script>

<main class="min-h-screen bg-gradient-to-b from-slate-950 via-slate-950 to-slate-950 text-white">
	<div class="mx-auto flex max-w-6xl flex-col gap-10 px-4 py-5">
		<section class="text-center">
			<p class="text-xs tracking-[0.32em] text-emerald-400 uppercase">Mouse control suite</p>
		</section>
		<FilterDashboard />
	</div>
</main>
