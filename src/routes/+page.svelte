<script lang="ts">
	import { UserAttentionType, appWindow } from '@tauri-apps/api/window';
</script>

<div class="prose p-5">
	<h3>Welcome!</h3>
	<p>
		You're about to install the <b>Wavebreaker Client.</b><br />
		It will:
	</p>
	<ul>
		<li>Make Audiosurf connect to a <b>custom server</b> instead of audio-surf.com</li>
		<li>Force Audiosurf to <b>always use HTTPS</b></li>
		<li>Enable <b>custom Audiosurf Radio songs</b></li>
	</ul>
</div>

<div class="flex w-full absolute bottom-0 p-5">
	<button on:click={() => appWindow.close()} class="btn btn-sm btn-error normal-case">Cancel</button
	>
	<a href="/locateGame" class="btn btn-sm btn-primary ml-auto normal-case">Let's go!</a>
</div>
