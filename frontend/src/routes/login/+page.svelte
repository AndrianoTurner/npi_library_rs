<script>
	import { axiosClient } from '../../hooks.client';
	import { auth_store } from '../../stores/auth';
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
				loginResponse = {
					status: response.data.status,
					id: response.data.id,
					token: response.data.token,
					email: response.data.email,
					errorCode: response.data.errorCode
				};
				let auth_data = {
					id: loginResponse.id,
					token: loginResponse.token,
					email: loginResponse.email
				};
				$auth_store = auth_data;
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
