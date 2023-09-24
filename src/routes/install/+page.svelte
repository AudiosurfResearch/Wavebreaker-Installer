<script lang="ts">
	import { gamePath } from '../../stores';
	import { invoke } from '@tauri-apps/api/tauri';

	let isInstalling = true;

	async function install(path: string) {
		await invoke('install', { path });
		isInstalling = false;
	}
</script>

<div class="prose p-5">
	<h3>Installing</h3>
	<p>Using game at {$gamePath}</p>
	{#await install($gamePath) then}
		<p class="text-success">Installed!</p>
	{:catch error}
		<p class="text-error">
			Failed to install: {error}<br />
			Please report this. You may continue anyway, if you'd like, but things will most likely be broken.
		</p>
	{/await}
</div>

<div class="flex w-full absolute bottom-0 p-5">
	{#if isInstalling}
		<button class="btn btn-sm btn-primary ml-auto normal-case" disabled
			>Working... <span class="loading loading-spinner loading-xs" /></button
		>
	{:else}
		<a href="/finish" class="btn btn-sm btn-primary ml-auto normal-case">Finish</a>
	{/if}
</div>
