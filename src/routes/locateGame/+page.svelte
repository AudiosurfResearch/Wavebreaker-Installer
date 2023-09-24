<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { gamePath } from '../../stores';

	let gameLocation: string;
	let isValid: boolean;

	$: gameLocation, gamePath.set(gameLocation);

	async function findAudiosurf() {
		gameLocation = await invoke('get_audiosurf_path');
		return gameLocation;
	}

	async function isValidInstall() {
		isValid = await invoke('is_valid_audiosurf_folder', {
			path: gameLocation
		});
		return isValid;
	}
</script>

<div class="prose p-5">
	<h3>Install location</h3>
	{#await findAudiosurf()}
		<p>Checking for existing install...</p>
	{:then gamePath}
		<p class="text-success">Found Audiosurf automagically!</p>
	{:catch error}
		<p class="text-error">{error}</p>
	{/await}
	<div class="form-control w-full max-w-xs">
		<input
			type="text"
			bind:value={gameLocation}
			placeholder="Game path goes here..."
			class="input input-bordered w-full max-w-xs"
		/>

		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label class="label">
			{#key gameLocation}
				{#await isValidInstall()}
					<span class="label-text-alt">Checking path...</span>
				{:then isValidInstall}
					{#if isValidInstall}
						<span class="label-text-alt text-success">Valid install!</span>
					{:else}
						<span class="label-text-alt text-error">Invalid path/install. </span>
					{/if}
				{/await}
			{/key}
		</label>
	</div>
	<p>
		This is where the Wavebreaker client will be installed.<br />
		<b>Keep in mind that only a legitimate Steam version of the game will work!</b>
	</p>
</div>

<div class="flex w-full absolute bottom-0 p-5">
	<a href="/" class="btn btn-sm btn-ghost normal-case">Go back</a>
	<a
		href="/install"
		class:btn-disabled={gameLocation == '' || !isValid}
		class="btn btn-sm btn-primary ml-auto normal-case">Install</a
	>
</div>
