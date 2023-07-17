<script>
	import { redirect } from '@sveltejs/kit';
	import { auth_store } from '../../stores/auth';
	function logout() {
		$auth_store = null;
	}
</script>

<button on:click={logout}>Выход</button>
