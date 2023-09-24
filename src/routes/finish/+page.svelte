<script lang="ts">
	import { open } from '@tauri-apps/api/shell';
    import { appWindow } from '@tauri-apps/api/window';

    async function startGame() {
        await open('steam://launch/12900');
        appWindow.close();
    }
</script>

<div class="prose p-5">
	<h3>Installation finished</h3>
	<p>Wavebreaker is now installed. Hit the button below to launch the game through Steam!</p>
</div>

<div class="flex w-full absolute bottom-0 p-5">
	<button on:click={startGame} class="btn btn-sm btn-primary ml-auto normal-case">Launch game!</button>
</div>
