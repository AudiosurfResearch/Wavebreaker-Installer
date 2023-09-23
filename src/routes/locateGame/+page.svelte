<script lang="ts">
	import { platform } from '@tauri-apps/api/os';
	import { invoke } from '@tauri-apps/api/tauri';

	async function findAudiosurf() {
		const audiosurfPath = await invoke('get_audiosurf_path');
        return audiosurfPath;
	}
</script>

<div class="prose p-5">
	<h3>Install location</h3>
	{#await findAudiosurf()}
		<p>Checking...</p>
	{:then gamePath}
		<p>Found Audiosurf at: {gamePath}</p>
    {:catch error}
        <p class="text-error">{error}</p>
	{/await}
	<p>Please select the location of your Audiosurf installation.</p>
</div>

<div class="flex w-full absolute bottom-0 p-5">
	<a href="/" class="btn btn-sm btn-ghost normal-case">Go back</a>
	<a href="/" class="btn btn-sm btn-primary ml-auto normal-case">Install</a>
</div>
