<script lang="ts">
	import '../app.postcss';
	import logo from '$lib/assets/wavebreaker_icon.svg';

	window.addEventListener('contextmenu', (e) => e.preventDefault());
</script>

<div class="navbar">
	<div class="font-semibold text-2xl gap-x-3 ml-2">
		<img src={logo} class="w-8 h-8" alt="Wavebreaker logo" />
		<span>Wavebreaker</span>
	</div>
</div>

<slot />
