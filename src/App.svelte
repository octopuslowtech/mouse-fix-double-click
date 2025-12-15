<script lang="ts">
	import FilterDashboard from '$lib/components/FilterDashboard.svelte';
</script>

<main class="min-h-screen bg-gradient-to-b from-slate-950 via-slate-950 to-slate-950 text-white">
	<div class="mx-auto flex max-w-6xl flex-col gap-10 px-6 py-10">
		<section class="space-y-3 text-center">
			<p class="text-xs uppercase tracking-[0.32em] text-emerald-400">Mouse control suite</p>
			<h1 class="text-3xl font-semibold text-white">Chặn double click tức thì, giao diện tối giản</h1>
			<p class="text-base text-slate-400">Tập trung vào tốc độ thao tác, quan sát trạng thái và log theo thời gian thực.</p>
		</section>
		<FilterDashboard />
	</div>
</main>
