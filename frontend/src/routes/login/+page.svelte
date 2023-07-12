<script>
	import { axiosClient } from '../../hooks.client';
	export let email = '';
	export let password = '';
	export let loginResponse = {
		status: '',
		id: -1,
		token: '',
		email: '',
		errorCode: null
	};
	function login() {
		let data = {
			email: email,
			password: password
		};
		axiosClient
			.post('user/login', JSON.stringify(data), {
				headers: {
					'Content-Type': 'application/json'
				}
			})
			.then(function (response) {
				console.log(response);
				loginResponse = {
					status: response.data.status,
					id: response.data.id,
					token: response.data.token,
					email: response.data.email,
					errorCode: response.data.errorCode
				};
			});
	}
</script>

<div class="centered">
	<h1>Вход</h1>

	<input bind:value={email} placeholder="email" type="email" />
	<p />
	<input bind:value={password} placeholder="password" type="password" />
	<p />
	<button on:click={login}>Войти</button>
	<p />
	{#if loginResponse.errorCode}
		<p>{loginResponse.status}</p>
	{/if}
	<p><a href="/register">Не зарегистрированы?</a></p>
</div>
